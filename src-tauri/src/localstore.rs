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

pub(crate) fn get_current_model(app_handle: tauri::AppHandle) -> Option<CurrentLanguageModel> {
    let store = load_store(app_handle);
    match store.get("current_language_model".to_string()) {
        Some(value) => return Some(serde_json::from_value(value.clone()).unwrap()),
        None => {
            println!("No current language model found");
            return None;
        }
    }
}

pub(crate) fn get_prompt_base(app_handle: tauri::AppHandle) -> String {
    let store = load_store(app_handle);
    match store.get("prompt_base".to_string()) {
        Some(value) => return serde_json::from_value(value.clone()).unwrap(),
        None => {
            println!("No prompt found");
            return "".to_string();
        }
    }
}

pub(crate) fn get_current_model_filename(app_handle: tauri::AppHandle) -> String {
    if let Some(model) = get_current_model(app_handle) {
        return model.filename;
    } else {
        println!("No model_filename found");
        return "".to_string();
    }
}

pub(crate) fn save_prompt_base(
    app_handle: tauri::AppHandle,
    prompt_base: String,
) -> Result<(), String> {
    let mut store = load_store(app_handle);
    store
        .insert("prompt_base".to_string(), json!(prompt_base))
        .unwrap();
    store.save().unwrap();

    Ok(())
}

fn load_store(app_handle: tauri::AppHandle) -> Store<tauri::Wry> {
    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    return store;
}
