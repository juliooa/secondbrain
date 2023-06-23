use std::process::Command;

use crate::localstore;

#[derive(serde::Deserialize)]
struct ConfigModels {
    models: Vec<ConfigLanguageModel>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConfigLanguageModel {
    pub name: String,
    pub filename: String,
    pub arquitecture: String,
    pub url: String,
    pub image: String,
    pub prompt_template: String,
    pub size: String,
}

pub fn get_config_language_models(app_handle: &tauri::AppHandle) -> Vec<ConfigLanguageModel> {
    let resource_path = app_handle
        .path_resolver()
        .resolve_resource("configs/models.json")
        .expect("failed to resolve resource");

    let models_file = std::fs::File::open(&resource_path).expect(&format!(
        "failed to open file: {} ",
        resource_path.to_str().unwrap()
    ));

    //FIX ME: if deserialization fails because models.json format, do something else than panic!
    let config_models: ConfigModels = serde_json::from_reader(models_file).unwrap();

    return config_models.models;
}

#[tauri::command]
pub fn show_in_folder(path: String) {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", &path]) // The comma after select is not a typo
            .spawn()
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs::metadata;
        use std::path::PathBuf;
        if path.contains(",") {
            // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
            let new_path = match metadata(&path).unwrap().is_dir() {
                true => path,
                false => {
                    let mut path2 = PathBuf::from(path);
                    path2.pop();
                    path2.into_os_string().into_string().unwrap()
                }
            };
            Command::new("xdg-open").arg(&new_path).spawn().unwrap();
        } else {
            Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.FileManager1",
                    "--type=method_call",
                    "/org/freedesktop/FileManager1",
                    "org.freedesktop.FileManager1.ShowItems",
                    format!("array:string:\"file://{path}\"").as_str(),
                    "string:\"\"",
                ])
                .spawn()
                .unwrap();
        }
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open").args(["-R", &path]).spawn().unwrap();
    }
}

#[tauri::command]
pub async fn choose_directory(app_handle: tauri::AppHandle) -> Result<String, String> {
    use tauri::api::dialog::blocking::FileDialogBuilder;

    let dialog_result = FileDialogBuilder::new().pick_folder();

    match dialog_result {
        Some(path) => {
            localstore::save_models_folder(app_handle, path.to_str().unwrap().to_string()).unwrap();
            return Ok(path.to_str().unwrap().to_string());
        }
        None => {
            return Err("No path selected".to_string());
        }
    }
}

#[tauri::command]
pub async fn get_models_folder(app_handle: tauri::AppHandle) -> Result<String, String> {
    match localstore::get_models_folder(app_handle) {
        Some(path) => {
            return Ok(path);
        }
        None => {
            return Err("No models folder found".to_string());
        }
    }
}
