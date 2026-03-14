mod utils;

use dashmap::DashMap;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use tauri::async_runtime::block_on;
use tauri::window::{Effect, EffectsBuilder};
use tauri::AppHandle;
use tauri::{Emitter, Manager};
use tokio::sync::Semaphore;
use url::Url;
use utils::{convert_date, get_html, get_remaining_chapters, read_bookmark};

use crate::utils::BookmarkData;

#[derive(Clone, serde::Serialize)]
struct Payload {
    index: usize,
    name: String,
    date_time: String,
    remaining_chapters: String,
    total_chapters: String,
    url: String,
    error: bool,
}

#[derive(Clone, serde::Serialize)]
struct PayloadMangaSize {
    size: usize,
}

#[derive(Default)]
struct AppData {
    thread: usize,
}

#[derive(Debug, Deserialize, Clone)]
struct Rule {
    site_match: Vec<String>,
    page_check: Option<String>,
    main_check: Option<String>,
    #[serde(rename = "type")]
    rule_type: String,
    value: Option<String>,
}

#[derive(Debug, Deserialize, Clone, serde::Serialize)]
struct ThreadViewData {
    url: String,
    added: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(Mutex::new(AppData::default()));

            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .set_effects(EffectsBuilder::new().effect(Effect::Acrylic).build())
                .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_manga, update_thread_count,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn update_thread_count(app_handle: AppHandle, new_thread_count: u64) {
    let state = app_handle.state::<Mutex<AppData>>().to_owned();

    // Lock the mutex to get mutable access:
    let mut state = state.lock().unwrap();

    // Modify the state:
    state.thread = new_thread_count as usize;
}

fn create_http_client() -> Client {
    Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/145.0.0.0 Safari/537.36")
        .pool_idle_timeout(tokio::time::Duration::from_secs(30))
        .pool_max_idle_per_host(4)
        .cookie_store(true)
        .build()
        .unwrap()
}

#[tauri::command]
async fn get_manga(app_handle: AppHandle) {
    let state = app_handle.state::<Mutex<AppData>>().to_owned();
    let worker_count = state.lock().unwrap().thread;
    let data = read_bookmark();
    app_handle
        .emit("scan_data", PayloadMangaSize { size: data.len() })
        .unwrap();
    let exe_path = get_exe_dir().unwrap();
    let rules_file =
        exe_path.join("/home/cyber/Documents/tauri/manga_bookmark_reader_old/config/rules.json");
    let json_data = fs::read_to_string(rules_file).unwrap();
    let rules: Vec<Rule> = serde_json::from_str(&json_data).unwrap();
    let client = create_http_client();

    // Per-host semaphores: each host gets its own Semaphore(worker_count)
    let host_semaphores: Arc<DashMap<String, Arc<Semaphore>>> = Arc::new(DashMap::new());

    for (_, manga_data) in data.clone().into_iter().enumerate() {
        // Extract host from URL, fall back to full URL if parsing fails
        let host = Url::parse(&manga_data.url)
            .ok()
            .and_then(|u| u.host_str().map(|h| h.to_string()))
            .unwrap_or_else(|| manga_data.url.clone());

        // Get or create a Semaphore(worker_count) for this host
        let semaphore = host_semaphores
            .entry(host)
            .or_insert_with(|| Arc::new(Semaphore::new(worker_count)))
            .clone();

        let app_handle_2 = app_handle.app_handle().to_owned();
        let temp_rules = rules.clone();
        let client = client.clone();

        thread::Builder::new()
            .spawn(move || {
                block_on(async {
                    let permit = semaphore.acquire_owned().await.unwrap();
                    let thread_data = ThreadViewData {
                        url: manga_data.url.clone(),
                        added: true,
                    };
                    app_handle_2.emit("thread_update", thread_data).unwrap();
                    let payload =
                        bookmark_check(client, manga_data.clone(), temp_rules.clone()).await;
                    let thread_data = ThreadViewData {
                        url: manga_data.url.clone(),
                        added: false,
                    };
                    app_handle_2.emit("thread_update", thread_data).unwrap();
                    app_handle_2.emit("scan_data", payload).unwrap();
                    drop(permit);
                })
            })
            .unwrap();
    }

    app_handle.emit("scan_data", "done").unwrap();
}

async fn bookmark_check(client: Client, manga_data: BookmarkData, rules: Vec<Rule>) -> Payload {
    let response: String = get_html(client, &manga_data.url).await;
    // let response = "".to_string();

    let dt = convert_date(manga_data.date_added.clone());
    let mut payload = Payload {
        index: manga_data.index,
        name: manga_data.name.clone(),
        date_time: dt.to_string(),
        remaining_chapters: 0.to_string(),
        total_chapters: 1.to_string(),
        url: manga_data.url.clone(),
        error: true,
    };

    for rule in rules {
        if !rule.site_match.iter().any(|s| manga_data.url.contains(s)) {
            continue;
        }

        match rule.page_check {
            Some(page_check_data) => {
                if !response.contains(&page_check_data) {
                    return payload;
                }
            }
            None => {}
        }

        match rule.rule_type.as_str() {
            "static" => {
                let static_remaining = match rule.value {
                    Some(value) => value,
                    None => "1".to_string(),
                };
                payload.error = false;
                payload.remaining_chapters = static_remaining;
            }
            "auto" => match rule.main_check {
                Some(main_rule) => {
                    if response.contains(&main_rule) {
                        let remaining_chapters =
                            get_remaining_chapters(response.clone(), rule.site_match.join("/"));
                        payload.remaining_chapters = remaining_chapters.remaining.to_string();
                        payload.total_chapters = remaining_chapters.total.to_string();
                    }
                    payload.error = false;
                }
                None => {
                    return payload;
                }
            },
            "inverse_auto" => match rule.main_check {
                Some(main_rule) => {
                    if !response.contains(&main_rule) {
                        let remaining_chapters =
                            get_remaining_chapters(response.clone(), rule.site_match.join("/"));
                        payload.remaining_chapters = remaining_chapters.remaining.to_string();
                        payload.total_chapters = remaining_chapters.total.to_string();
                    }
                    payload.error = false;
                }
                None => {
                    return payload;
                }
            },
            "toggle" => match rule.main_check {
                Some(main_rule) => {
                    if response.contains(&main_rule) {
                        payload.remaining_chapters = "1".to_string();
                    }
                    payload.error = false;
                }
                None => {
                    return payload;
                }
            },
            "inverse_toggle" => match rule.main_check {
                Some(main_rule) => {
                    if !response.contains(&main_rule) {
                        payload.remaining_chapters = "1".to_string();
                    }
                    payload.error = false;
                }
                None => {
                    return payload;
                }
            },
            _ => {
                return payload;
            }
        }
    }

    return payload;
}

fn get_exe_dir() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    return Ok(exe_path
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf());
}
