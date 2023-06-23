use llm::load_progress_callback_stdout as load_callback;
use llm::InferenceError;
use llm::InferenceFeedback;
use llm::InferenceRequest;
use llm::InferenceStats;
use llm::LoadError;
use llm::Prompt;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use std::convert::Infallible;
use std::fs;
use std::io::Write;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::vec;
use tauri::Manager;

use crate::localstore::CurrentLanguageModel;
use crate::{configs, downloader, localstore, AppState};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LanguageModel {
    filename: String,
    current: bool,
    downloaded: bool,
    has_info: bool,
    info: Option<LanguageModelInfo>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LanguageModelInfo {
    name: String,
    arquitecture: String,
    url: String,
    image: String,
    prompt_template: String,
    size: String,
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

#[tauri::command]
pub fn get_prompt_template(app_handle: tauri::AppHandle) -> String {
    println!("Command: get_prompt");
    return localstore::get_prompt_template(app_handle);
}

#[tauri::command]
pub fn get_language_models(app_handle: tauri::AppHandle) -> GetLanguageModelsResponse {
    println!("Command: get_language_models");

    let config_models = configs::get_config_language_models(&app_handle);
    let mut language_models: Vec<LanguageModel> = vec![];

    let models_path_option = localstore::get_models_folder(app_handle.clone());
    if let None = models_path_option {
        println!("No models folder found");
        return GetLanguageModelsResponse { models: vec![] };
    }
    let models_path = models_path_option.unwrap();

    let current_model_filename = localstore::get_current_model_filename(app_handle);
    if let Ok(entries) = fs::read_dir(&models_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(filename) = entry.file_name().to_str() {
                            //if name is similar to .DS_Store we continue
                            if filename.to_lowercase().starts_with(".ds_store") {
                                continue;
                            }
                            println!("Model file found: {}", filename);
                            language_models.push(LanguageModel {
                                filename: filename.to_string(),
                                current: current_model_filename == filename,
                                downloaded: true,
                                has_info: false,
                                info: None,
                            });
                            //language_model_files.push(filename.to_string());
                        }
                    }
                }
            }
        }
    }

    for config_model in config_models.iter() {
        let model_info = Some(LanguageModelInfo {
            name: config_model.name.clone(),
            arquitecture: config_model.arquitecture.clone(),
            url: config_model.url.clone(),
            image: config_model.image.clone(),
            prompt_template: config_model.prompt_template.clone(),
            size: config_model.size.clone(),
        });

        if let Some(index) = language_models
            .iter()
            .position(|model| model.filename == config_model.filename)
        {
            language_models[index].has_info = true;
            language_models[index].info = model_info;
        } else {
            language_models.push(LanguageModel {
                has_info: true,
                info: model_info,
                filename: config_model.filename.clone(),
                current: false,
                downloaded: false,
            });
        }
    }
    language_models.sort_by(|a, b| a.filename.to_lowercase().cmp(&b.filename.to_lowercase()));
    return GetLanguageModelsResponse {
        models: language_models,
    };
}

#[tauri::command]
pub fn set_current_model(
    model_filename: &str,
    model_name: &str,
    model_arquitecture: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    println!("Command: set_current_model, filename:{}", model_filename);
    let models_path_str = localstore::get_models_folder(app_handle.clone()).unwrap();
    let mut models_path = PathBuf::from(&models_path_str);
    models_path.push(&model_filename);
    match load_model(&models_path, model_arquitecture) {
        Ok(model) => {
            let app_state = app_handle.state::<AppState>();
            app_state.model.lock().unwrap().replace(model);

            let app_handle_cloned = Arc::new(Mutex::new(app_handle));
            localstore::save_current_model(
                Arc::clone(&app_handle_cloned).lock().unwrap().app_handle(),
                CurrentLanguageModel {
                    name: model_name.to_string(),
                    filename: model_filename.to_string(),
                    path: models_path.to_string_lossy().to_string(),
                    arquitecture: model_arquitecture.to_string(),
                },
            )?;

            Ok(())
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[tauri::command]
pub fn get_active_model(app_handle: tauri::AppHandle) -> Option<CurrentLanguageModel> {
    return localstore::get_active_model(app_handle);
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
                        let prompt_template = localstore::get_prompt_template(app_handle.clone());
                        let mut prompt = message.text.clone();
                        if prompt_template.contains("[[message]]") {
                            prompt = prompt_template.replace("[[message]]", &prompt);
                        }
                        prompt
                    }
                    MessageRole::AI => message.text.clone(),
                })
                .collect::<Vec<String>>()
                .join(" ");
            println!("Prompt: {}", prompt);

            let mut answer: String = "".to_string();
            let res = start_inference(&app_handle, model, "this is".to_string(), |token| {
                println!("{token}");
                answer.push_str(&token);
                window
                    .emit(
                        "new_token",
                        Payload {
                            message: token.to_string(),
                        },
                    )
                    .unwrap();
                Ok(InferenceFeedback::Continue)
            });
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

fn stop_infering(app_handle: &tauri::AppHandle) -> bool {
    let session_state = app_handle.state::<SessionState>();
    let should_stop_infering = session_state.inner().should_stop_infering.lock().unwrap();
    return *should_stop_infering;
}

fn reset_stop_infering(app_handle: &tauri::AppHandle) {
    let session_state = app_handle.state::<SessionState>();
    let mut should_stop_infering = session_state.inner().should_stop_infering.lock().unwrap();
    *should_stop_infering = false;
}

#[tauri::command]
pub async fn ask(message: String, app_handle: tauri::AppHandle, window: tauri::Window) -> String {
    let app_state = app_handle.state::<AppState>();
    match app_state.inner().model.lock().unwrap().as_ref() {
        Some(model) => {
            let prompt_template = localstore::get_prompt_template(app_handle.clone());
            let mut prompt = message;
            if prompt_template.contains("[[message]]") {
                prompt = prompt_template.replace("[[message]]", &prompt);
            }
            println!("Prompt: {}", prompt);

            let mut answer: String = "".to_string();
            let res = start_inference(&app_handle, model, prompt, |token| {
                std::io::stdout().flush().unwrap();
                println!("{token}");
                answer.push_str(&token);
                window
                    .emit(
                        "new_token",
                        Payload {
                            message: token.to_string(),
                        },
                    )
                    .unwrap();
                Ok(InferenceFeedback::Continue)
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
            return Ok(model);
        }
        Err(err) => {
            println!("Error loading model: {}", err);
            return Err(err);
        }
    }
}

fn start_inference(
    app_handle: &tauri::AppHandle,
    model: &Box<dyn llm::Model>,
    prompt: String,
    mut inference_token_callback: impl FnMut(String) -> Result<InferenceFeedback, Infallible>,
) -> Result<InferenceStats, InferenceError> {
    reset_stop_infering(&app_handle);

    let mut session = model.start_session(Default::default());
    return session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &InferenceRequest {
            prompt: Prompt::Text(&prompt),
            play_back_previous_tokens: false,
            parameters: &build_parameters(app_handle.clone()),
            maximum_token_count: Some(1000),
        },
        &mut Default::default(),
        |inference_response| match inference_response {
            llm::InferenceResponse::PromptToken(_) => Ok(llm::InferenceFeedback::Continue),
            llm::InferenceResponse::InferredToken(t) => {
                if stop_infering(&app_handle) {
                    println!("Stop infering");
                    return Ok(InferenceFeedback::Halt);
                }
                return inference_token_callback(t);
            }
            _ => Ok(llm::InferenceFeedback::Continue),
        },
    );
}

fn build_parameters(app_handle: tauri::AppHandle) -> llm::InferenceParameters {
    let top_p_top_ksampler = llm::samplers::TopPTopK {
        top_p: localstore::get_top_p(app_handle.clone())
            .parse::<f32>()
            .unwrap(),
        top_k: localstore::get_top_k(app_handle.clone())
            .parse::<usize>()
            .unwrap(),
        repeat_penalty: localstore::get_repetition_penalty(app_handle.clone())
            .parse::<f32>()
            .unwrap(),
        temperature: localstore::get_temperature(app_handle)
            .parse::<f32>()
            .unwrap(),
        ..Default::default()
    };

    return llm::InferenceParameters {
        sampler: Arc::new(top_p_top_ksampler),
        ..Default::default()
    };
}

pub(crate) struct SessionState {
    pub should_stop_infering: Mutex<bool>,
}

#[tauri::command]
pub async fn cancel_inference(app_handle: tauri::AppHandle) {
    let session_state = app_handle.state::<SessionState>();
    let mut should_stop_infering = session_state.inner().should_stop_infering.lock().unwrap();
    *should_stop_infering = true;
}

#[tauri::command]
pub async fn save_parameters(
    prompt_template: &str,
    temperature: &str,
    top_p: &str,
    top_k: &str,
    repetition_penalty: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    localstore::save_parameters(
        app_handle,
        prompt_template,
        temperature,
        top_p,
        top_k,
        repetition_penalty,
    )?;

    return Ok(());
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct InferenceParameters {
    pub prompt_template: String,
    pub temperature: String,
    pub top_p: String,
    pub top_k: String,
    pub repetition_penalty: String,
}

#[tauri::command]
pub async fn get_parameters(app_handle: tauri::AppHandle) -> Result<InferenceParameters, String> {
    let parameters = InferenceParameters {
        prompt_template: localstore::get_prompt_template(app_handle.clone()),
        temperature: localstore::get_temperature(app_handle.clone()),
        top_p: localstore::get_top_p(app_handle.clone()),
        top_k: localstore::get_top_k(app_handle.clone()),
        repetition_penalty: localstore::get_repetition_penalty(app_handle.clone()),
    };

    return Ok(parameters);
}
