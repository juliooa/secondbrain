use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde_json::json;
use tauri_plugin_store::{Store, StoreBuilder};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CurrentLanguageModel {
    pub name: String,
    pub filename: String,
    pub arquitecture: String,
    pub path: String,
}

pub(crate) fn save_current_model(
    app_handle: tauri::AppHandle,
    model: CurrentLanguageModel,
) -> Result<(), String> {
    let mut store = load_store(app_handle);
    store
        .insert("current_language_model".to_string(), json!(model))
        .unwrap();
    store.save().unwrap();

    Ok(())
}

pub(crate) fn get_active_model(app_handle: tauri::AppHandle) -> Option<CurrentLanguageModel> {
    let store = load_store(app_handle);
    match store.get("current_language_model".to_string()) {
        Some(value) => return Some(serde_json::from_value(value.clone()).unwrap()),
        None => {
            println!("No current language model found");
            return None;
        }
    }
}

pub(crate) fn get_prompt_template(app_handle: tauri::AppHandle) -> String {
    let store = load_store(app_handle);
    match store.get("prompt_template".to_string()) {
        Some(value) => return serde_json::from_value(value.clone()).unwrap(),
        None => {
            println!("No prompt found");
            return "[[message]]".to_string();
        }
    }
}

pub(crate) fn get_current_model_filename(app_handle: tauri::AppHandle) -> String {
    if let Some(model) = get_active_model(app_handle) {
        return model.filename;
    } else {
        println!("No model_filename found");
        return "".to_string();
    }
}

pub(crate) fn save_prompt_template(
    app_handle: tauri::AppHandle,
    prompt_template: &str,
) -> Result<(), String> {
    let mut store = load_store(app_handle);
    store
        .insert("prompt_template".to_string(), json!(prompt_template))
        .unwrap();
    store.save().unwrap();

    Ok(())
}

pub(crate) fn save_parameters(
    app_handle: tauri::AppHandle,
    prompt_template: &str,
    temperature: &str,
    top_p: &str,
    top_k: &str,
    repetition_penalty: &str,
) -> Result<(), String> {
    save_prompt_template(app_handle.clone(), prompt_template)?;
    let mut store = load_store(app_handle);
    store
        .insert("temperature".to_string(), json!(temperature))
        .unwrap();
    store.insert("top_p".to_string(), json!(top_p)).unwrap();
    store.insert("top_k".to_string(), json!(top_k)).unwrap();
    store
        .insert("repetition_penalty".to_string(), json!(repetition_penalty))
        .unwrap();

    store.save().unwrap();

    Ok(())
}

pub(crate) fn get_temperature(app_handle: tauri::AppHandle) -> String {
    let store = load_store(app_handle);
    match store.get("temperature".to_string()) {
        Some(value) => return serde_json::from_value(value.clone()).unwrap(),
        None => {
            println!("No temperature found");
            return "0.8".to_string();
        }
    }
}

pub(crate) fn get_top_p(app_handle: tauri::AppHandle) -> String {
    let store = load_store(app_handle);
    match store.get("top_p".to_string()) {
        Some(value) => return serde_json::from_value(value.clone()).unwrap(),
        None => {
            println!("No top_p found");
            return "0.95".to_string();
        }
    }
}

pub(crate) fn get_top_k(app_handle: tauri::AppHandle) -> String {
    let store = load_store(app_handle);
    match store.get("top_k".to_string()) {
        Some(value) => return serde_json::from_value(value.clone()).unwrap(),
        None => {
            println!("No top_k found");
            return "40".to_string();
        }
    }
}

pub(crate) fn get_repetition_penalty(app_handle: tauri::AppHandle) -> String {
    let store = load_store(app_handle);
    match store.get("repetition_penalty".to_string()) {
        Some(value) => return serde_json::from_value(value.clone()).unwrap(),
        None => {
            println!("No repetition_penalty found");
            return "1.3".to_string();
        }
    }
}

pub(crate) fn save_models_folder(
    app_handle: tauri::AppHandle,
    models_folder: String,
) -> Result<(), String> {
    let mut store = load_store(app_handle);
    store
        .insert("models_folder".to_string(), json!(models_folder))
        .unwrap();
    store.save().unwrap();

    Ok(())
}

pub(crate) fn get_models_folder(app_handle: tauri::AppHandle) -> Option<String> {
    match load_store(app_handle).get("models_folder".to_string()) {
        Some(value) => return Some(serde_json::from_value(value.clone()).unwrap()),
        None => {
            println!("No models folder found");
            return None;
        }
    }
}

fn load_store(app_handle: tauri::AppHandle) -> Store<tauri::Wry> {
    let mut store = StoreBuilder::new(app_handle.clone(), "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => {
            println!("Store loaded");
        }
        Err(err) => {
            do_first_time(&mut store, &app_handle);
            println!("Store file not found: {}", err);
        }
    }
    return store;
}

fn do_first_time(store: &mut Store<tauri::Wry>, app_handle: &tauri::AppHandle) {
    if let None = store.get("first_time") {
        let download_path = app_handle
            .path_resolver()
            .app_data_dir()
            .unwrap()
            .join("models");

        store
            .insert(
                "models_folder".to_string(),
                json!(download_path.to_string_lossy()),
            )
            .unwrap();
        store
            .insert("first_time".to_string(), json!(false))
            .unwrap();
        store.save().unwrap();
    }
}
