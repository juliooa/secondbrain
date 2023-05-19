use serde_json::{json, Value};
use tauri_plugin_store::StoreBuilder;

pub(crate) fn get_current_model_id(app_handle: tauri::AppHandle) -> Option<Value> {
    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    return store.get("current_model_id".to_string()).cloned();
}

pub(crate) fn get_current_model_path(app_handle: tauri::AppHandle) -> Option<Value> {
    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    return store.get("current_model_path".to_string()).cloned();
}

pub(crate) fn insert_current_model_path(
    app_handle: tauri::AppHandle,
    model_path: String,
) -> Result<(), String> {
    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    store
        .insert("current_model_path".to_string(), json!(model_path))
        .unwrap();
    store.save().unwrap();

    Ok(())
}

pub(crate) fn insert_current_model_id(
    app_handle: tauri::AppHandle,
    model_id: u32,
) -> Result<(), String> {
    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    store
        .insert("current_model_id".to_string(), json!(model_id))
        .unwrap();
    store.save().unwrap();

    Ok(())
}

pub(crate) fn insert_current_model_name(
    app_handle: tauri::AppHandle,
    model_name: String,
) -> Result<(), String> {
    let mut store = StoreBuilder::new(app_handle, "store.bin".parse().unwrap()).build();
    match store.load() {
        Ok(_) => println!("Store loaded"),
        Err(err) => println!("Store file not found: {}", err),
    }
    store
        .insert("current_model_name".to_string(), json!(model_name))
        .unwrap();
    store.save().unwrap();

    Ok(())
}
