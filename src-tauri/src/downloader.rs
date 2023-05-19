use futures_util::TryStreamExt;
use read_progress_stream::ReadProgressStream;
use serde::{ser::Serializer, Serialize};
use tauri::{Manager, Runtime, Window};
use tokio::task::JoinHandle;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tokio_util::codec::{BytesCodec, FramedRead};

use std::{collections::HashMap, sync::Mutex};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("{0}")]
    ContentLength(String),
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
    modelId: u32,
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
    model_id: u32,
    file_name: String,
    finish_download_notice: String,
    window: tauri::Window,
    app_handle: tauri::AppHandle,
) -> std::result::Result<u32, String> {
    let mut download_path = app_handle
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .join("models");
    std::fs::create_dir_all(&download_path).unwrap();
    download_path.push(file_name);

    println!("Downloading model to {}", download_path.to_str().unwrap());
    let callback = Box::new(|| {
        println!("Callback called!");
    });

    let handler = tokio::task::spawn(async move {
        let result = download(
            window,
            model_id,
            &url,
            download_path.to_str().unwrap(),
            HashMap::new(),
            &finish_download_notice,
            callback,
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

    Ok(model_id)
}
#[tauri::command]
pub async fn cancel_download(
    model_id: u32,
    app_handle: tauri::AppHandle,
) -> std::result::Result<u32, String> {
    let app_state = app_handle.state::<DownloadState>();
    let lock = app_state.tokio_handle.lock().unwrap();
    if let Some(handler) = &*lock {
        handler.abort();
    }
    //TODO delete the file here instead of calling from frontend
    Ok(model_id)
}
fn on_finish_download() {
    // set the model as downloaded
}

pub(crate) async fn download<R: Runtime>(
    window: Window<R>,
    id: u32,
    url: &str,
    file_path: &str,
    headers: HashMap<String, String>,
    finish_download_notice: &str,
    callback: Callback,
) -> Result<u32> {
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

    let file = File::create(file_path).await;
    match file {
        Ok(mut f) => {
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.try_next().await? {
                let result_write = f.write_all(&chunk).await;
                if let Err(err) = result_write {
                    println!("Error writing to file: {}", err);
                    return Err(Error::Io(err));
                }
                let _ = window.emit(
                    "progress_download",
                    ProgressPayload {
                        modelId: id,
                        progress: chunk.len() as u64,
                        total,
                    },
                );
            }
            callback();
            let _ = window.emit("finish_download", finish_download_notice);
        }
        Err(e) => {
            println!("Error creating file: {}", e);
        }
    }

    Ok(id)
}

async fn upload<R: Runtime>(
    window: Window<R>,
    id: u32,
    url: &str,
    file_path: &str,
    headers: HashMap<String, String>,
) -> Result<serde_json::Value> {
    // Read the file
    let file = File::open(file_path).await?;

    // Create the request and attach the file to the body
    let client = reqwest::Client::new();
    let mut request = client.post(url).body(file_to_body(id, window, file));

    // Loop trought the headers keys and values
    // and add them to the request object.
    for (key, value) in headers {
        request = request.header(&key, value);
    }

    let response = request.send().await?;

    response.json().await.map_err(Into::into)
}

fn file_to_body<R: Runtime>(id: u32, window: Window<R>, file: File) -> reqwest::Body {
    let stream = FramedRead::new(file, BytesCodec::new()).map_ok(|r| r.freeze());
    let window = Mutex::new(window);
    reqwest::Body::wrap_stream(ReadProgressStream::new(
        stream,
        Box::new(move |progress, total| {
            let _ = window.lock().unwrap().emit(
                "upload://progress",
                ProgressPayload {
                    modelId: id,
                    progress,
                    total,
                },
            );
        }),
    ))
}

pub(crate) async fn delete(file_path: &str, callback: Callback) -> std::result::Result<(), String> {
    println!("Deleting file! {}", file_path);

    match fs::remove_file(file_path).await {
        Ok(()) => {
            println!("File removed successfully");
            callback();
            Ok(())
        }
        Err(err) => {
            eprintln!("Failed to remove file: {}", err);
            Err(err.to_string())
        }
    }
}
