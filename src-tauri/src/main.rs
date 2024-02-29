// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Mutex, thread};

use clipboard_rs::{Clipboard, ClipboardContext, ClipboardWatcher, ClipboardWatcherContext};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct MyState {
  clipboard_ctx: std::sync::Mutex<ClipboardContext>,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn read(state: tauri::State<'_, MyState>) -> Result<String, String> {
  let res = state.clipboard_ctx.lock().unwrap().get_text();
  Ok(res.unwrap())
}

#[tauri::command]
async fn watch(   app: tauri::AppHandle) -> Result<(), String> {
    let mut wathcer = ClipboardWatcherContext::new().unwrap();
    wathcer.add_handler(Box::new(move||{
        println!("clipboard change");
        let _=app.emit_all("clipboard-change", true);
    }));
    thread::spawn(move||{
        wathcer.start_watch();
    });
  Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,read,watch])
        .setup(|app|{
            let ctx = ClipboardContext::new().unwrap();
            app.manage(MyState{
                clipboard_ctx:Mutex::new(ctx)
            });
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
