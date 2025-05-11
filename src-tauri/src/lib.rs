mod utils;

use serde_json::Value;
use std::sync::Arc;
use std::thread;
use tauri::async_runtime::block_on;
use tauri::window::{Effect, EffectsBuilder};
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
    error: bool,
}

#[derive(Clone, serde::Serialize)]
struct PayloadMangaSize {
    size: usize,
}

type CompleteQueue = deadqueue::limited::Queue<usize>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().to_owned();

            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();
            app.listen("start_scan", move |event| {
                let payload: Value = serde_json::from_str(event.payload()).unwrap();
                let worker_count = payload.get("worker_count").unwrap();
                spawn(get_manga(
                    app_handle.to_owned(),
                    worker_count.as_i64().unwrap().try_into().unwrap(),
                ));
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_manga(app_handle: AppHandle, worker_count: usize) {
    let data = read_bookmark();
    let mut index = 0;

    let task_count = data.len();

    let complete_queue = Arc::new(CompleteQueue::new(task_count));

    app_handle
        .emit("scan_data", PayloadMangaSize { size: data.len() })
        .unwrap();

    let mut prev_thread_count = 0;
    loop {
        if prev_thread_count != complete_queue.len() {
            prev_thread_count = complete_queue.len();
            app_handle
                .emit("threads", format!("{}", prev_thread_count))
                .unwrap();
        }
        while complete_queue.len() < worker_count && index < task_count {
            let manga_data = data[index].clone();

            complete_queue.try_push(manga_data.index).unwrap();
            app_handle
                .emit("thread_update", format!("+{}", manga_data.url.clone()))
                .unwrap();
            index += 1;
            app_handle
                .emit("threads", format!("{}", prev_thread_count))
                .unwrap();

            let _complete_queue = complete_queue.clone();
            let app_handle_2 = app_handle.app_handle().to_owned();
            thread::Builder::new()
                .stack_size(0)
                .spawn(move || {
                    block_on(async {
                        let response = get_html(&manga_data.url).await;
                        let dt = convert_date(manga_data.date_added.clone());
                        let mut payload = Payload {
                            index: manga_data.index,
                            name: manga_data.name.clone(),
                            date_time: dt.to_string(),
                            remaining_chapters: 0.to_string(),
                            url: manga_data.url.clone(),
                            error: false,
                        };
                        if manga_data.url.contains("natomanga")
                            || manga_data.url.contains("manganato")
                        {
                            if response.contains("PREV CHAPTER") {
                                if response.contains("class=\"back\"") {
                                    payload.remaining_chapters = get_remaining_chapters(
                                        response.clone(),
                                        "natomanga/manganato".to_owned(),
                                    );
                                }
                            } else {
                                payload.error = true;
                            }
                        } else if manga_data.url.contains("manhwaclan") {
                            if response.contains("class=\"btn prev_page\"") {
                                if response.contains("class=\"btn next_page\"") {
                                    payload.remaining_chapters = get_remaining_chapters(
                                        response.clone(),
                                        "manhwaclan".to_owned(),
                                    );
                                }
                            } else {
                                payload.error = true;
                            }
                        } else if manga_data.url.contains("asuracomic") {
                            if response.contains("rounded-full cursor-not-allowed") {
                                payload.remaining_chapters = "0".to_string();
                            } else {
                                payload.remaining_chapters = "1".to_string();
                            }
                        } else {
                            payload.error = true;
                        }
                        app_handle_2.emit("scan_data", payload).unwrap();
                        _complete_queue.pop().await;
                        app_handle_2
                            .emit("thread_update", format!("-{}", manga_data.url.clone()))
                            .unwrap();
                    })
                })
                .unwrap();
        }

        if index == task_count && complete_queue.len() == 0 {
            app_handle.emit("threads", format!("{}", 0)).unwrap();
            break;
        }
    }

    app_handle.emit("scan_data", "done").unwrap();
}
