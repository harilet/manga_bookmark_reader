mod utils;

use serde_json::Value;
use std::sync::Arc;
use std::thread;
use tauri::async_runtime::block_on;
use tauri::window::{Effect, EffectsBuilder};
use tauri::{async_runtime::spawn, AppHandle};
use tauri::{Emitter, Listener, Manager};
use utils::{convert_date, get_html, get_remaining_chapters, read_bookmark, BookmarkData};

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

const WORKER_COUNT: usize = 8;

type TaskQueue = deadqueue::limited::Queue<BookmarkData>;
type CompleteQueue = deadqueue::limited::Queue<usize>;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();
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
    let data = read_bookmark();
    let mut index = 0;

    let task_count = data.len();

    let queue = Arc::new(TaskQueue::new(task_count));
    let complete_queue = Arc::new(CompleteQueue::new(task_count));

    app_handle
        .emit("scan_data", PayloadMangaSize { size: data.len() })
        .unwrap();

    loop {
        while complete_queue.len() < WORKER_COUNT
            && queue.len() < WORKER_COUNT
            && index < task_count
        {
            queue.try_push(data[index].clone()).unwrap();
            complete_queue.try_push(index).unwrap();
            index += 1;
        }
        while queue.len() > 0 && complete_queue.len() > 0 {
            let queue = queue.clone();
            let complete_queue = complete_queue.clone();
            let app_handle_2 = app_handle.app_handle().to_owned();
            thread::spawn(move || {
                block_on(async {
                    loop {
                        let i = queue.pop().await;

                        let response = get_html(&i.url).await;
                        let dt = convert_date(i.date_added.clone());
                        let mut payload = Payload {
                            index: i.index,
                            name: i.name.clone(),
                            date_time: dt.to_string(),
                            remaining_chapters: 0.to_string(),
                            url: i.url.clone(),
                        };
                        if response.contains("navi-change-chapter-btn-next") {
                            payload.remaining_chapters = get_remaining_chapters(response.clone());
                        }
                        app_handle_2.emit("scan_data", payload).unwrap();
                        complete_queue.pop().await;
                    }
                })
            });
        }
        if index == task_count && queue.len() == 0 && complete_queue.len() == 0 {
            break;
        }
    }

    app_handle.emit("scan_data", "done").unwrap();
}
