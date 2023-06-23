use futures_util::TryStreamExt;
use serde::{ser::Serializer, Serialize};
use tauri::{Manager, Runtime, Window};
use tokio::task::JoinHandle;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use std::path::PathBuf;
use std::{collections::HashMap, sync::Mutex};

use crate::localstore;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Clone, Serialize)]
struct ProgressPayload {
    model_filename: String,
    progress: u64,
    total: u64,
}

type Callback = Box<dyn FnOnce() + Send + 'static>;
pub struct DownloadState {
    pub tokio_handle: Mutex<Option<JoinHandle<()>>>,
}

#[tauri::command]
pub async fn download_model(
    url: String,
    model_filename: String,
    finish_download_notice: String,
    window: tauri::Window,
    app_handle: tauri::AppHandle,
) -> std::result::Result<String, String> {
    let models_path = localstore::get_models_folder(app_handle.clone()).unwrap();
    let mut download_path = PathBuf::from(&models_path);
    std::fs::create_dir_all(&download_path).unwrap();
    download_path.push(&model_filename);

    println!("Downloading model to {}", download_path.to_str().unwrap());
    let finish_download_callback = Box::new(|| {
        //TODO: maybe send finish event here
    });

    let filename_clone = model_filename.clone();
    let handler = tokio::task::spawn(async move {
        let result = download(
            window,
            &filename_clone,
            &url,
            download_path.to_str().unwrap(),
            HashMap::new(),
            &finish_download_notice,
            finish_download_callback,
        )
        .await;

        match result {
            Ok(_) => println!("Download finished!"),
            Err(err) => println!("Error downloading model: {}", err),
        }
    });

    let app_state = app_handle.state::<DownloadState>();
    let mut lock = app_state.tokio_handle.lock().unwrap();
    (*lock).replace(handler);

    Ok(model_filename)
}

#[tauri::command]
pub async fn cancel_download(
    model_filename: String,
    app_handle: tauri::AppHandle,
) -> std::result::Result<String, String> {
    let app_state = app_handle.state::<DownloadState>();
    let lock = app_state.tokio_handle.lock().unwrap();
    if let Some(handler) = &*lock {
        handler.abort();
    }
    //TODO delete the file here instead of calling from frontend
    Ok(model_filename)
}

pub(crate) async fn download<R: Runtime>(
    window: Window<R>,
    filename: &str,
    url: &str,
    file_path: &str,
    headers: HashMap<String, String>,
    finish_download_notice: &str,
    finish_download_callback: Callback,
) -> Result<String> {
    let client = reqwest::Client::new();
    let mut request = client.get(url);
    for (key, value) in headers {
        request = request.header(&key, value);
    }

    let response = request.send().await;
    let response = match response {
        Ok(r) => r,
        Err(err) => {
            println!("Error downloading model: {}", err);
            return Err(Error::Request(err));
        }
    };
    let total = response.content_length().unwrap_or(0);
    let mut progress: u64 = 0;

    let file = File::create(file_path).await;
    match file {
        Ok(mut f) => {
            let mut stream = response.bytes_stream();
            println!("Starting streaming of: {}", filename);
            while let Some(chunk) = stream.try_next().await? {
                let result_write = f.write_all(&chunk).await;
                if let Err(err) = result_write {
                    println!("Error writing to file: {}", err);
                    return Err(Error::Io(err));
                }
                progress += chunk.len() as u64;
                let _ = window.emit(
                    "progress_download",
                    ProgressPayload {
                        model_filename: filename.to_string(),
                        progress: progress,
                        total,
                    },
                );
            }
            finish_download_callback();
            let _ = window.emit("finish_download", finish_download_notice);
        }
        Err(e) => {
            println!("Error creating file: {}", e);
        }
    }

    Ok(filename.to_string())
}

pub(crate) async fn delete(
    file_path: &str,
    delete_done_callback: Callback,
) -> std::result::Result<(), String> {
    println!("Deleting file! {}", file_path);

    match fs::remove_file(file_path).await {
        Ok(()) => {
            println!("File removed successfully");
            delete_done_callback();
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to remove file: {}", err);
            Err(err.to_string())
        }
    }
}
