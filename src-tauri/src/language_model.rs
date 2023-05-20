use llm::load_progress_callback_stdout as load_callback;
use llm::InferenceRequest;
use llm::LoadError;
use std::convert::Infallible;
use std::io::Write;
use std::path::Path;
use tauri::Manager;

use crate::{configs, downloader, localstore, AppState};

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

#[derive(serde::Serialize)]
pub struct GetLanguageModelsResponse {
    pub models: Vec<LanguageModel>,
}

#[tauri::command]
pub fn get_language_models(app_handle: tauri::AppHandle) -> GetLanguageModelsResponse {
    println!("Command: get_language_models");

    let config_models = configs::get_config_language_models(&app_handle);
    let download_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("models");

    let current_model_id = localstore::get_current_model_id(app_handle);
    let mut language_models: Vec<LanguageModel> = vec![];

    for model in config_models.iter() {
        let mut model_download_path = download_path.clone();
        model_download_path.push(&model.filename);

        language_models.push(LanguageModel {
            id: model.id,
            name: model.name.clone(),
            url: model.url.clone(),
            downloaded: model_download_path.exists(),
            current: current_model_id.as_ref().is_some()
                && current_model_id.as_ref().unwrap().as_u64().unwrap() as u32 == model.id,
            filename: model.filename.clone(),
            image: model.image.clone(),
        });
    }

    return GetLanguageModelsResponse {
        models: language_models,
    };
}

#[tauri::command]
pub fn set_current_model(
    model_id: u32,
    model_filename: &str,
    model_name: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    println!("Command: set_current_model id:{}", model_id);
    let model_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("models")
        .join(model_filename);

    match load_model(model_path.to_str().unwrap()) {
        Ok(model) => {
            let app_state = app_handle.state::<AppState>();
            app_state.model.lock().unwrap().replace(model);

            localstore::insert_current_model_id(app_handle.clone(), model_id)?;
            localstore::insert_current_model_name(app_handle.clone(), model_name.to_string())?;
            localstore::insert_current_model_path(
                app_handle,
                model_path.to_str().unwrap().to_string(),
            )?;

            Ok(())
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub async fn infere(text: String, app_handle: tauri::AppHandle, window: tauri::Window) -> String {
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
            return "Error: No model loaded".to_string();
        }
    }
}

#[tauri::command]
pub async fn delete_model(
    model_id: u32,
    model_filename: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let model_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("models")
        .join(model_filename);

    return downloader::delete(model_path.to_str().unwrap(), Box::new(|| {}))
        .await
        .map_err(|err| {
            println!("Error downloading model: {}", err);
            return err.to_string();
        });
}

pub fn load_model(model_path: &str) -> Result<Box<dyn llm::Model>, LoadError> {
    println!("Loading model from {}", model_path);

    let model = llm::load_dynamic(
        "llama".parse().unwrap_or_else(|e| panic!("{e}")),
        Path::new(model_path),
        Default::default(),
        None,
        load_callback,
    );
    if model.is_ok() {
        println!("Model loaded!");
    }

    return model;
}
