#[derive(serde::Deserialize)]
struct ConfigModels {
    models: Vec<ConfigLanguageModel>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConfigLanguageModel {
    pub id: u32,
    pub name: String,
    pub filename: String,
    pub url: String,
    pub image: String,
}

pub fn get_config_language_models(app_handle: &tauri::AppHandle) -> Vec<ConfigLanguageModel> {
    //1. get the models from the config file
    let resource_path = app_handle
        .path_resolver()
        .resolve_resource("configs/models.json")
        .expect("failed to resolve resource");
    //TODO return some error to frontend, instead of panicking
    let models_file = std::fs::File::open(&resource_path).expect(&format!(
        "failed to open file: {} ",
        resource_path.to_str().unwrap()
    ));
    let config_models: ConfigModels = serde_json::from_reader(models_file).unwrap();

    return config_models.models;
}
