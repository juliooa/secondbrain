// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use llm::load_progress_callback_stdout as load_callback;
use llm::InferenceRequest;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use std::{convert::Infallible, io::Write, path::Path};
use tauri::Manager;

mod configs;
mod downloader;
struct AppState {
    system_message: Mutex<String>,
    messages: Mutex<Vec<String>>,
    model: Mutex<Box<dyn llm::Model>>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LanguageModel {
    id: u32,
    name: String,
    filename: String,
    url: String,
    image: String,
    downloaded: bool,
    default: bool,
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .manage(AppState {
            messages: Mutex::from(vec![]),
            system_message: Mutex::from("".to_string()),
            model: Mutex::from(load_model()),
        })
        .invoke_handler(tauri::generate_handler![
            infere,
            download_model,
            get_language_models,
            log
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
    //
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
async fn infere(text: String, app_handle: tauri::AppHandle, window: tauri::Window) -> String {
    let app_state = app_handle.state::<AppState>();

    let model = app_state.inner().model.lock().unwrap();
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

fn load_model() -> Box<dyn llm::Model> {
    let model = llm::load_dynamic(
        "llama".parse().unwrap_or_else(|e| panic!("{e}")),
        Path::new("/Users/julioandres/Development/llm-models/wizardLM-7B.ggml.q4_2.bin"),
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
    let config_models = configs::get_config_language_models(&app_handle);

    let mut download_path = app_handle.path_resolver().app_data_dir().unwrap();
    download_path.push("models");

    let mut language_models: Vec<LanguageModel> = vec![];
    for model in config_models.iter() {
        let mut model_download_path = download_path.clone();
        model_download_path.push(&model.filename);
        println!("> {:?}", model_download_path);
        println!("+ {:?}", model_download_path.exists());

        language_models.push(LanguageModel {
            id: model.id,
            name: model.name.clone(),
            url: model.url.clone(),
            downloaded: model_download_path.exists(),
            default: false,
            filename: model.filename.clone(),
            image: model.image.clone(),
        });
    }

    //3. compare the files with the models from the config file to set which
    //   models are downloaded and which are not
    //4. get the default model from settings and set it as default
    //5. return the models

    // match fs::read_dir(download_path) {
    //     Err(why) => println!("! {:?}", why.kind()),
    //     Ok(paths) => {
    //         for path in paths {
    //             println!("> {:?}", path.unwrap().path());
    //         }
    //     }
    // }

    //return models;
    return GetLanguageModelsResponse {
        models: language_models,
    };
}
