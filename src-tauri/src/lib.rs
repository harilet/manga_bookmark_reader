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

const WORKER_COUNT: usize = 8;

type CompleteQueue = deadqueue::limited::Queue<usize>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().to_owned();
            app.listen("get_threads", move |_event| {
                app_handle
                    .emit("threads", format!("{}/{}", 0, WORKER_COUNT))
                    .unwrap();
            });

            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();
            let app_handle2 = app.handle().to_owned();
            app.listen("start_scan", move |event| {
                let payload: Value = serde_json::from_str(event.payload()).unwrap();
                if payload.get("the_message").unwrap() == "read_manga" {
                    spawn(get_manga(app_handle2.to_owned()));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_manga(app_handle: AppHandle) {
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
                .emit("threads", format!("{}/{}", prev_thread_count, WORKER_COUNT))
                .unwrap();
        }
        while complete_queue.len() < WORKER_COUNT && index < task_count {
            let i = data[index].clone();

            complete_queue.try_push(i.index).unwrap();
            index += 1;
            app_handle
                .emit("threads", format!("{}/{}", prev_thread_count, WORKER_COUNT))
                .unwrap();

            let _complete_queue = complete_queue.clone();
            let app_handle_2 = app_handle.app_handle().to_owned();
            thread::Builder::new()
                .stack_size(0)
                .spawn(move || {
                    block_on(async {
                        let response = get_html(&i.url).await;
                        let dt = convert_date(i.date_added.clone());
                        let mut payload = Payload {
                            index: i.index,
                            name: i.name.clone(),
                            date_time: dt.to_string(),
                            remaining_chapters: 0.to_string(),
                            url: i.url.clone(),
                            error: false,
                        };
                        if response.contains("navi-change-chapter-btn-prev") {
                            if response.contains("navi-change-chapter-btn-next") {
                                payload.remaining_chapters =
                                    get_remaining_chapters(response.clone());
                            }
                        } else if response.contains("class=\"next\"") {
                            if response.contains("class=\"back\"") {
                                payload.remaining_chapters =
                                    get_remaining_chapters(response.clone());
                            }
                        } else {
                            payload.error = true;
                        }
                        app_handle_2.emit("scan_data", payload).unwrap();
                        _complete_queue.pop().await;
                    })
                })
                .unwrap();
        }
        if index == task_count && complete_queue.len() == 0 {
            app_handle
                .emit("threads", format!("{}/{}", 0, WORKER_COUNT))
                .unwrap();
            break;
        }
    }

    app_handle.emit("scan_data", "done").unwrap();
}
