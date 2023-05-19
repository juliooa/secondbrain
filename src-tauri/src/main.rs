// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use downloader::DownloadState;

use std::sync::Mutex;
use tauri::Manager;

mod configs;
mod downloader;
mod language_model;
mod localstore;
struct AppState {
    system_message: Mutex<String>,
    messages: Mutex<Vec<String>>,
    model: Mutex<Option<Box<dyn llm::Model>>>,
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.app_handle();

            let current_model_path = get_current_model_path(app_handle);
            let model: Option<Box<dyn llm::Model>> =
                current_model_path.and_then(|path| match language_model::load_model(&path) {
                    Ok(model) => Some(model),
                    Err(err) => {
                        println!("Error loading model: {}", err);
                        None
                    }
                });

            let state = AppState {
                messages: Mutex::from(vec![]), //TODO maybe reload chat conversation
                system_message: Mutex::from("".to_string()), //TODO get system message
                model: Mutex::from(model),
            };
            app.manage(state);
            app.manage(DownloadState {
                tokio_handle: Mutex::from(None),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            language_model::infere,
            downloader::download_model,
            language_model::get_language_models,
            language_model::set_current_model,
            language_model::delete_model,
            downloader::cancel_download,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_current_model_path(app_handle: tauri::AppHandle) -> Option<String> {
    match localstore::get_current_model_path(app_handle) {
        Some(value) => {
            println!("Current model path: {:?}", value);
            let current_model_path: String = serde_json::from_value(value).unwrap();
            return Some(current_model_path);
        }
        None => {
            println!("No current model path found");
            return None;
        }
    }
}
