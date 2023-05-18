// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use llm::load_progress_callback_stdout as load_callback;
use llm::InferenceRequest;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use std::{convert::Infallible, io::Write, path::Path};
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;

mod configs;
mod downloader;
struct AppState {
    system_message: Mutex<String>,
    messages: Mutex<Vec<String>>,
    model: Mutex<Option<Box<dyn llm::Model>>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LanguageModel {
    id: u32,
    name: String,
    filename: String,
    url: String,
    image: String,
    downloaded: bool,
    current: bool,
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.app_handle();

            let current_model_path = get_current_model_path(app_handle);
            let model: Option<Box<dyn llm::Model>> = if current_model_path.is_some() {
                Some(load_model(&current_model_path.unwrap()))
            } else {
                None
            };
            let state = AppState {
                messages: Mutex::from(vec![]), //TODO maybe reload chat conversation
                system_message: Mutex::from("".to_string()), //TODO get system message
                model: Mutex::from(model),
            };
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            infere,
            download_model,
            get_language_models,
            set_current_model,
            log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_current_model_path(app_handle: tauri::AppHandle) -> Option<String> {
    let store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.get("current_model_path") {
        Some(value) => {
            let current_model_path: String = serde_json::from_value(value.clone()).unwrap();
            return Some(current_model_path);
        }
        None => {
            println!("No current model path found");
            return None;
        }
    }
}

#[tauri::command]
async fn log(window: tauri::Window, app_handle: tauri::AppHandle) -> () {
    let send_this = app_handle.path_resolver().app_data_dir().unwrap();
    let _ = window.emit("log", send_this);
    ()
}

#[tauri::command]
async fn download_model(
    url: String,
    model_id: u32,
    file_name: String,
    finish_download_notice: String,
    window: tauri::Window,
    app_handle: tauri::AppHandle,
) -> Result<u32, String> {
    let mut download_path = app_handle.path_resolver().app_data_dir().unwrap();
    download_path.push("models");
    fs::create_dir_all(&download_path).unwrap();
    download_path.push(file_name);

    println!("Downloading model to {}", download_path.to_str().unwrap());
    let callback = Box::new(|| {
        println!("Callback called!");
    });

    let result = downloader::download(
        window,
        model_id,
        &url,
        download_path.to_str().unwrap(),
        HashMap::new(),
        &finish_download_notice,
        callback,
    )
    .await;

    match result {
        Ok(r) => {
            println!("resultooo {}", r);
            Ok(model_id)
        }
        Err(err) => {
            println!("Error downloading model: {}", err);
            Err(err.to_string())
        }
    }
}

fn on_finish_download() {
    // set the model as downloaded
}

#[tauri::command]
fn set_current_model(
    model_id: u32,
    model_filename: &str,
    window: tauri::Window,
    app_handle: tauri::AppHandle,
) {
    println!("Command: set_current_model id:{}", model_id);
    let model_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("models")
        .join(model_filename);

    let model = load_model(model_path.to_str().unwrap());
    let app_state = app_handle.state::<AppState>();
    app_state.model.lock().unwrap().replace(model);

    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    store
        .insert("current_model_path".to_string(), json!(model_path))
        .unwrap();
    store
        .insert("current_model_id".to_string(), json!(model_id))
        .unwrap();
    store.save().unwrap();
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn infere(text: String, app_handle: tauri::AppHandle, window: tauri::Window) -> String {
    let app_state = app_handle.state::<AppState>();
    match app_state.inner().model.lock().unwrap().as_ref() {
        Some(model) => {
            let mut session = model.start_session(Default::default());
            let mut conversation = app_state.inner().messages.lock().unwrap();
            conversation.push(format!("Human: {text}"));
            let system_message = app_state.inner().system_message.lock().unwrap();
            let mut prompt = format!("{}\n\n", system_message);
            for (_, message) in conversation.iter().enumerate() {
                prompt.push_str(&format!("{}\n", message));
            }
            prompt.push_str(&format!("Assistant: "));

            let mut answer: String = "".to_string();
            let res = session.infer::<Infallible>(
                model.as_ref(),
                &mut rand::thread_rng(),
                &InferenceRequest {
                    prompt: prompt.as_str(),
                    play_back_previous_tokens: false,
                    ..Default::default()
                },
                // OutputRequest
                &mut Default::default(),
                |inference_response| match inference_response {
                    llm::InferenceResponse::PromptToken(_) => Ok(llm::InferenceFeedback::Continue),
                    llm::InferenceResponse::InferredToken(t) => {
                        std::io::stdout().flush().unwrap();
                        println!("{t}");
                        answer.push_str(&t);
                        window
                            .emit(
                                "new_token",
                                Payload {
                                    message: t.to_string(),
                                },
                            )
                            .unwrap();
                        Ok(llm::InferenceFeedback::Continue)
                    }
                    _ => Ok(llm::InferenceFeedback::Continue),
                },
            );
            conversation.push(answer.clone());

            match res {
                Ok(_) => format!("{}", answer),
                Err(err) => format!("\n{err}"),
            }
        }
        None => {
            println!("No model loaded");
            return "No model loaded".to_string();
        }
    }
}

fn load_model(model_path: &str) -> Box<dyn llm::Model> {
    println!("Loading model from {}", model_path);
    //TODO return error to frontend
    let model = llm::load_dynamic(
        "llama".parse().unwrap_or_else(|e| panic!("{e}")),
        Path::new(model_path),
        Default::default(),
        load_callback,
    )
    .unwrap_or_else(|err| panic!("Failed to load  model from : {err}"));

    let now = std::time::Instant::now();
    println!(
        "Model fully loaded! Elapsed: {}ms",
        now.elapsed().as_millis()
    );
    model
}

#[derive(serde::Serialize)]
pub struct GetLanguageModelsResponse {
    pub models: Vec<LanguageModel>,
}

#[tauri::command]
fn get_language_models(app_handle: tauri::AppHandle) -> GetLanguageModelsResponse {
    println!("Command: get_language_models");

    let config_models = configs::get_config_language_models(&app_handle);

    let download_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("models");

    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    let current_model_id = store.get("current_model_id".to_string());
    println!("Current model id: {:?}", current_model_id);

    let mut language_models: Vec<LanguageModel> = vec![];
    for model in config_models.iter() {
        let mut model_download_path = download_path.clone();
        model_download_path.push(&model.filename);

        language_models.push(LanguageModel {
            id: model.id,
            name: model.name.clone(),
            url: model.url.clone(),
            downloaded: model_download_path.exists(),
            current: current_model_id.is_some()
                && current_model_id.unwrap().as_u64().unwrap() as u32 == model.id,
            filename: model.filename.clone(),
            image: model.image.clone(),
        });
    }

    return GetLanguageModelsResponse {
        models: language_models,
    };
}
