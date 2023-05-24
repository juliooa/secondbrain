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
    pub prompt_base: String,
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
