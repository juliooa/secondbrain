use llm::load_progress_callback_stdout as load_callback;
use llm::InferenceRequest;
use llm::LoadError;
use llm::Prompt;
use std::convert::Infallible;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::Manager;

use crate::localstore::CurrentLanguageModel;
use crate::{configs, downloader, localstore, AppState};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LanguageModel {
    filename: String,
    name: String,
    arquitecture: String,
    url: String,
    image: String,
    downloaded: bool,
    current: bool,
    prompt_base: String,
}

pub struct ChatState {
    pub messages: Mutex<Vec<Message>>,
}
pub struct Message {
    text: String,
    role: MessageRole,
}

#[derive(Debug)]
pub enum MessageRole {
    Human,
    AI,
}

#[derive(serde::Serialize)]
pub struct GetLanguageModelsResponse {
    pub models: Vec<LanguageModel>,
}

pub(crate) static MODELS_FOLDER: &str = "models";

#[tauri::command]
pub fn get_prompt_base(app_handle: tauri::AppHandle) -> String {
    println!("Command: get_prompt");
    return localstore::get_prompt_base(app_handle);
}

#[tauri::command]
pub fn get_language_models(app_handle: tauri::AppHandle) -> GetLanguageModelsResponse {
    println!("Command: get_language_models");

    let config_models = configs::get_config_language_models(&app_handle);
    let mut language_models: Vec<LanguageModel> = vec![];
    let mut language_model_files: Vec<String> = vec![];

    let download_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join(MODELS_FOLDER);

    let current_model_filename = localstore::get_current_model_filename(app_handle);
    if let Ok(entries) = fs::read_dir(&download_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(filename) = entry.file_name().to_str() {
                            println!("Model file found: {}", filename);
                            language_model_files.push(filename.to_string());
                        }
                    }
                }
            }
        }
    }

    for model in config_models.iter() {
        let mut model_download_path = download_path.clone();
        model_download_path.push(&model.filename);

        let mut model_downloaded = false;
        if let Some(index) = language_model_files
            .iter()
            .position(|file| file == &model.filename)
        {
            model_downloaded = true;
            language_model_files.remove(index);
        }

        language_models.push(LanguageModel {
            filename: model.filename.clone(),
            name: model.name.clone(),
            url: model.url.clone(),
            arquitecture: model.arquitecture.clone(),
            downloaded: model_downloaded,
            current: current_model_filename == model.filename,
            image: model.image.clone(),
            prompt_base: model.prompt_base.clone(),
        });
    }

    // Adding all the models that are not in the config file
    // but are files in the models folder
    for file in language_model_files.iter() {
        let filename = file.clone();
        language_models.push(LanguageModel {
            name: filename.clone(),
            url: "".to_string(),
            downloaded: true,
            current: current_model_filename == filename,
            filename: filename,
            image: "".to_string(),
            prompt_base: "".to_string(),
            arquitecture: "llama".to_string(), //FIXME we are hardcoding Llama, but we need to ask the user what arquitecture is this
        });
    }

    return GetLanguageModelsResponse {
        models: language_models,
    };
}

#[tauri::command]
pub fn set_current_model(
    model_filename: &str,
    model_name: &str,
    model_arquitecture: &str,
    prompt_base: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    println!("Command: set_current_model, filename:{}", model_filename);
    let model_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join(MODELS_FOLDER)
        .join(model_filename);

    match load_model(model_path.to_str().unwrap(), model_arquitecture) {
        Ok(model) => {
            let app_state = app_handle.state::<AppState>();
            app_state.model.lock().unwrap().replace(model);

            let app_handle_cloned = Arc::new(Mutex::new(app_handle));
            localstore::save_current_model(
                Arc::clone(&app_handle_cloned).lock().unwrap().app_handle(),
                CurrentLanguageModel {
                    name: model_name.to_string(),
                    filename: model_filename.to_string(),
                    path: model_path.to_str().unwrap().to_string(),
                    arquitecture: model_arquitecture.to_string(),
                },
            )?;
            localstore::save_prompt_base(
                Arc::clone(&app_handle_cloned).lock().unwrap().app_handle(),
                prompt_base.to_string(),
            )?;

            Ok(())
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[tauri::command]
pub fn get_current_model(app_handle: tauri::AppHandle) -> Option<CurrentLanguageModel> {
    return localstore::get_current_model(app_handle);
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command]
pub async fn chat(message: String, app_handle: tauri::AppHandle, window: tauri::Window) -> String {
    let app_state = app_handle.state::<AppState>();
    let chat_state = app_handle.state::<ChatState>();

    match app_state.inner().model.lock().unwrap().as_ref() {
        Some(model) => {
            let mut session = model.start_session(Default::default());
            let mut messages = chat_state.inner().messages.lock().unwrap();
            messages.push(Message {
                text: message.clone(),
                role: MessageRole::Human,
            });

            // TODO: limit the number of messages to the last X,
            // so the context is not too big
            let prompt = messages
                .iter()
                .map(|message| match message.role {
                    MessageRole::Human => {
                        let prompt_base = localstore::get_prompt_base(app_handle.clone());
                        let mut prompt = message.text.clone();
                        if prompt_base.contains("[[message]]") {
                            prompt = prompt_base.replace("[[message]]", &prompt);
                        }
                        prompt
                    }
                    MessageRole::AI => message.text.clone(),
                })
                .collect::<Vec<String>>()
                .join(" ");
            println!("Prompt: {}", prompt);

            let mut answer: String = "".to_string();
            let res = session.infer::<Infallible>(
                model.as_ref(),
                &mut rand::thread_rng(),
                &InferenceRequest {
                    prompt: Prompt::Text(&prompt),
                    play_back_previous_tokens: false,
                    ..Default::default()
                },
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
            messages.push(Message {
                text: answer.clone(),
                role: MessageRole::AI,
            });

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
pub async fn ask(message: String, app_handle: tauri::AppHandle, window: tauri::Window) -> String {
    let app_state = app_handle.state::<AppState>();
    match app_state.inner().model.lock().unwrap().as_ref() {
        Some(model) => {
            let mut session = model.start_session(Default::default());
            let prompt_base = localstore::get_prompt_base(app_handle.clone());
            let mut prompt = message;
            if prompt_base.contains("[[message]]") {
                prompt = prompt_base.replace("[[message]]", &prompt);
            }
            println!("Prompt: {}", prompt);

            let mut answer: String = "".to_string();
            let res = session.infer::<Infallible>(
                model.as_ref(),
                &mut rand::thread_rng(),
                &InferenceRequest {
                    prompt: Prompt::Text(&prompt),
                    play_back_previous_tokens: false,
                    ..Default::default()
                },
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
    model_filename: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let models_path_str = localstore::get_models_folder(app_handle.clone()).unwrap();
    let mut models_path = PathBuf::from(&models_path_str);
    models_path.push(&model_filename);

    let delete_done_callback = Box::new(|| {
        //TODO: maybe send delete done event here
    });

    return downloader::delete(models_path.to_str().unwrap(), delete_done_callback)
        .await
        .map_err(|err| {
            println!("Error downloading model: {}", err);
            return err.to_string();
        });
}

pub fn load_model(
    model_path: &PathBuf,
    arquitecture: &str,
) -> Result<Box<dyn llm::Model>, LoadError> {
    println!("Loading model:");
    println!("- Path: {}", model_path.display());
    println!("- Arquitecture: {}", arquitecture);

    let model = llm::load_dynamic(
        arquitecture.parse().unwrap_or_else(|e| panic!("{e}")),
        model_path,
        llm::VocabularySource::Model,
        Default::default(),
        load_callback,
    );
    match model {
        Ok(model) => {
            println!("Model loaded!");
            //println!("Vocabulary size: {}", model.vocabulary());
            return Ok(model);
        }
        Err(err) => {
            println!("Error loading model: {}", err);
            return Err(err);
        }
    }
    // if (model.is_err()) {
    //     println!("Error loading model: {}", model.err().unwrap());
    // }
    // if model.is_ok() {
    //     println!("Model loaded!");
    // }
}

}
