// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use llm::load_progress_callback_stdout as load_callback;
use llm::InferenceRequest;
use std::collections::HashMap;
use std::sync::Mutex;
use std::{convert::Infallible, io::Write, path::Path};
use tauri::App;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Window;

mod dummy_model;
struct AppState {
    system_message: Mutex<String>,
    messages: Mutex<Vec<String>>,
}
fn main() {
    tauri::Builder::default()
        .manage(AppState {
            messages: Mutex::from(vec![]),
            system_message: Mutex::from("".to_string()),
        })
        .invoke_handler(tauri::generate_handler![infere])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn infere(message: String, window: Window, app_handle: tauri::AppHandle) -> String {
    let model = load_model();
    let mut session = model.start_session(Default::default());
    let app_state = app_handle.state::<AppState>();
    let mut conversation = app_state.inner().messages.lock().unwrap();
    conversation.push(format!("Human: {message}"));
    let system_message = app_state.inner().system_message.lock().unwrap();
    let mut prompt = format!("{}\n\n", system_message);
    for (_, message) in conversation.iter().enumerate() {
        prompt.push_str(&format!("{}\n", message));
    }
    prompt.push_str(&format!("Assistant: "));

    let mut answer: String = "".to_string();
    let res = session.infer::<Infallible>(
        model.as_ref(),
        &mut rand::thread_rng(),
        &InferenceRequest {
            prompt: prompt.as_str(),
            play_back_previous_tokens: false,
            ..Default::default()
        },
        // OutputRequest
        &mut Default::default(),
        |t| {
            print!("{t}");
            std::io::stdout().flush().unwrap();
            answer.push_str(t);
            window
                .emit(
                    "new_token",
                    Payload {
                        message: t.to_string(),
                    },
                )
                .unwrap();
            Ok(())
        },
    );

    let conversation_string = conversation.join("");
    let clean_answer = answer.replace(&conversation_string, "");
    conversation.push(clean_answer.clone());
    println!("-----");
    for (_, message) in conversation.iter().enumerate() {
        println!("{}\n", message);
        println!("+++");
    }
    println!("-----");

    match res {
        Ok(_) => format!("{}", clean_answer),
        Err(err) => format!("\n{err}"),
    }
}

fn load_model() -> Box<dyn llm::Model> {
    let model = llm::load_dynamic(
        "llama".parse().unwrap_or_else(|e| panic!("{e}")),
        Path::new("/Users/julioandres/Development/llm-models/wizardLM-7B.ggml.q4_2.bin"),
        Default::default(),
        load_callback,
    )
    .unwrap_or_else(|err| panic!("Failed to load  model from : {err}"));

    let now = std::time::Instant::now();
    println!(
        "Model fully loaded! Elapsed: {}ms",
        now.elapsed().as_millis()
    );
    model
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}
