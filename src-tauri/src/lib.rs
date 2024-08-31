mod utils;

use serde_json::Value;
use tauri::window::{Effect, EffectsBuilder};
use std::thread::{self, JoinHandle};
use tauri::async_runtime::block_on;
use tauri::{async_runtime::spawn, AppHandle};
use tauri::{Emitter, Listener, Manager};
use utils::{convert_date, get_html, get_remaining_chapters, read_bookmark};

#[derive(Clone, serde::Serialize)]
struct Payload {
    index: usize,
    name: String,
    date_time: String,
    remaining_chapters: String,
    url: String,
}

#[derive(Clone, serde::Serialize)]
struct PayloadMangaSize {
    size: usize,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_effects(
                EffectsBuilder::new()
                .effect(Effect::Acrylic)
                .build(),
            ).unwrap();
            let app_handle = app.handle().to_owned();
            app.listen("start_scan", move |event| {
                let payload: Value = serde_json::from_str(event.payload()).unwrap();
                if payload.get("the_message").unwrap() == "read_manga" {
                    spawn(get_manga(app_handle.to_owned()));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_manga(app_handle: AppHandle) {
    let mut index = 0;
    let data = read_bookmark();
    let mut threads:Vec<JoinHandle<()>> = vec![];
    app_handle
        .emit("scan_data", PayloadMangaSize { size: data.len() })
        .unwrap();
    for i in data {
        let app_handle_2 = app_handle.app_handle().to_owned();
        let temp_thread=thread::spawn(move || {
            block_on(async {
                let response = get_html(&i.url).await;
                let dt = convert_date(i.date_added.clone());
                let mut payload = Payload {
                    index,
                    name: i.name.clone(),
                    date_time: dt.to_string(),
                    remaining_chapters: 0.to_string(),
                    url: i.url.clone(),
                };
                if response.contains("navi-change-chapter-btn-next") {
                    payload.remaining_chapters = get_remaining_chapters(response.clone());
                }
                app_handle_2.emit("scan_data", payload).unwrap();
            })
        });
        threads.push(temp_thread);
        index += 1;
    }
    for thread in threads{
        thread.join().unwrap();
    }
    app_handle.emit("scan_data", "done").unwrap();
}
