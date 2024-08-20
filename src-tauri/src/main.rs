// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clipboard_rs::{
    common::ContentData, Clipboard, ClipboardContext, ClipboardHandler, ClipboardWatcher,
    ClipboardWatcherContext, ContentFormat, WatcherShutdown,
};
use serde::Serialize;
use std::{sync::Mutex, thread};
use tauri::{AppHandle, Manager};

struct ClipboardManager {
    ctx: ClipboardContext,
    app: AppHandle,
}

impl ClipboardManager {
    pub fn new(app: AppHandle) -> Self {
        let ctx = ClipboardContext::new().unwrap();
        ClipboardManager { ctx, app }
    }
}

impl ClipboardHandler for ClipboardManager {
    fn on_clipboard_change(&mut self) {
        let _ = self.app.emit_all(
            "clipboard_change",
            Some(self.ctx.available_formats().unwrap_or(vec![])),
        );
    }
}

struct MyState {
    ctx: Mutex<ClipboardContext>,
    stop_channel: Mutex<Option<WatcherShutdown>>,
}

#[derive(Serialize)]
struct Content {
    format: String,
    data: String,
}

#[tauri::command]
async fn read(state: tauri::State<'_, MyState>) -> Result<Vec<Content>, String> {
    let locked = state
        .ctx
        .lock()
        .map_err(|e| format!("lock failed: {:?}", e))?;
    let contents = locked
        .get(&[
            ContentFormat::Text,
            ContentFormat::Html,
            ContentFormat::Rtf,
            ContentFormat::Files,
        ])
        .unwrap_or(vec![]);
    let mut res = vec![];
    for content in contents {
        let format = match content.get_format() {
            ContentFormat::Text => "text/plain".to_string(),
            ContentFormat::Html => "text/html".to_string(),
            ContentFormat::Rtf => "text/rtf".to_string(),
            ContentFormat::Files => "text/uri-list".to_string(),
            ContentFormat::Image => "image".to_string(),
            ContentFormat::Other(format) => format.clone(),
        };
        let data = match content.as_str() {
            Ok(data) => data.to_string(),
            Err(_) => "".to_string(),
        };
        res.push(Content { format, data });
    }
    Ok(res)
}

#[tauri::command]
async fn has_image(state: tauri::State<'_, MyState>) -> Result<bool, String> {
    let locked = state
        .ctx
        .lock()
        .map_err(|e| format!("lock failed: {:?}", e))?;
    Ok(locked.has(ContentFormat::Image))
}

#[tauri::command]
async fn watch(app: tauri::AppHandle, state: tauri::State<'_, MyState>) -> Result<(), String> {
    let mut stop_channel = state.stop_channel.lock().expect("lock failed");
    if stop_channel.is_some() {
        println!("watcher already started");
        return Err("watcher already started".to_string());
    }
    // 你也可以只使用这个 watcher，这个 watcher 会在剪贴板变动时回调你，我这只是一个简单的 demo，你可以丰富你的 manager 去做更多事情
    let manager = ClipboardManager::new(app);
    let mut watcher = ClipboardWatcherContext::new().expect("watcher init failed");
    let shutdown_channel = watcher.add_handler(manager).get_shutdown_channel();
    stop_channel.replace(shutdown_channel);
    thread::spawn(move || {
        println!("start watch");
        watcher.start_watch();
    });
    Ok(())
}

#[tauri::command]
async fn stop_watch(state: tauri::State<'_, MyState>) -> Result<(), String> {
    // state.stop_channel.lock().unwrap().take().unwrap().stop();
    let mut shutdown_channel = state.stop_channel.lock().expect("lock failed");
    if let Some(shutdown_channel) = shutdown_channel.take() {
        println!("stop watch");
        shutdown_channel.stop();
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read, watch, stop_watch, has_image])
        .setup(|app| {
            // 你可以只使用这个 ctx，在你觉得想要使用的时候使用
            let ctx = ClipboardContext::new().expect("clipboard init failed");
            app.manage(MyState {
                ctx: Mutex::new(ctx),
                stop_channel: Mutex::new(None),
            });
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
