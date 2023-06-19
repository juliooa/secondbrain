// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use downloader::DownloadState;

use std::{path::PathBuf, sync::Mutex, vec};
use tauri::Manager;

mod configs;
mod downloader;
mod language_model;
mod localstore;
struct AppState {
    model: Mutex<Option<Box<dyn llm::Model>>>,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.app_handle();

            let model: Option<Box<dyn llm::Model>> = match localstore::get_active_model(app_handle)
            {
                Some(current_model) => {
                    match language_model::load_model(
                        &PathBuf::from(current_model.path),
                        &current_model.arquitecture,
                    ) {
                        Ok(model) => {
                            println!("Loaded model: {:?}", current_model.name);
                            Some(model)
                        }
                        Err(e) => {
                            println!("Error loading model: {:?}", e);
                            None
                        }
                    }
                }
                None => {
                    println!("No current model");
                    None
                }
            };

            app.manage(AppState {
                model: Mutex::from(model),
            });
            app.manage(language_model::ChatState {
                messages: Mutex::from(vec![]),
            });
            app.manage(DownloadState {
                tokio_handle: Mutex::from(None),
            });
            app.manage(language_model::SessionState {
                should_stop_infering: Mutex::from(false),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            language_model::chat,
            language_model::ask,
            language_model::get_language_models,
            language_model::set_current_model,
            language_model::delete_model,
            language_model::get_prompt_template,
            language_model::get_active_model,
            language_model::cancel_inference,
            language_model::save_parameters,
            language_model::get_parameters,
            downloader::download_model,
            downloader::cancel_download,
            configs::show_in_folder,
            configs::choose_directory,
            configs::get_models_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
