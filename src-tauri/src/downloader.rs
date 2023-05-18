use futures_util::TryStreamExt;
use serde::{ser::Serializer, Serialize};
use tauri::{command, Runtime, Window};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use read_progress_stream::ReadProgressStream;

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
    id: u32,
    progress: u64,
    total: u64,
}

type Callback = Box<dyn FnOnce() + Send + 'static>;

pub(crate) async fn download<R: Runtime>(
    window: Window<R>,
    id: u32,
    url: &str,
    file_path: &str,
    headers: HashMap<String, String>,
    finish_download_notice: &str,
    callback: Callback,
) -> Result<u32> {
    println!("Downloading modell! {}", url);
    let client = reqwest::Client::new();

    let mut request = client.get(url);
    // // Loop trought the headers keys and values
    // // and add them to the request object.
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
                        id,
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

#[command]
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
                    id,
                    progress,
                    total,
                },
            );
        }),
    ))
}
