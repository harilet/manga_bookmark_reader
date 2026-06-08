use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Duration, TimeZone};
use html_parser::Dom;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub(crate) struct BookmarkData {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) date_added: String,
    pub(crate) index: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct RemainingChapters {
    pub(crate) total: usize,
    pub(crate) remaining: usize,
}

fn default_order() -> String {
    "ascending".to_string()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct SiteParser {
    pub name: String,
    #[serde(rename = "type")]
    pub parser_type: String,
    pub selector: String,
    #[serde(default)]
    pub offset: usize,
    #[serde(default)]
    pub json_key: String,
    #[serde(default)]
    pub json_chapter_list: String,
    #[serde(default)]
    pub json_label: String,
    #[serde(default = "default_order")]
    pub order: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct SiteParserConfig {
    pub sites: Vec<SiteParser>,
}

pub(crate) fn read_bookmark() -> Vec<BookmarkData> {
    let mut result_data: Vec<BookmarkData> = vec![];
    let contents =
        fs::read_to_string("/home/cyber/.config/BraveSoftware/Brave-Browser/Default/Bookmarks")
            .expect("Should have been able to read the file");

    let object: Value = serde_json::from_str(&contents).unwrap();

    let data = object
        .get("roots")
        .unwrap()
        .get("bookmark_bar")
        .unwrap()
        .get("children")
        .unwrap();

    for value in data.as_array().unwrap() {
        if value.get("name").unwrap() == "anime" {
            let all_anime = value.get("children").unwrap().as_array().unwrap();
            let mut index = 0;

            for j in all_anime.iter() {
                let name = match j.get("name") {
                    Some(name_data) => match name_data.as_str() {
                        Some(name_str_data) => name_str_data,
                        None => "",
                    },
                    None => "",
                };
                if name == "series" {
                    continue;
                }
                index += 1;
                let url = match j.get("url") {
                    Some(url_data) => match url_data.as_str() {
                        Some(url_str_data) => url_str_data.to_string(),
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                };
                result_data.push(BookmarkData {
                    name: j.get("name").unwrap().as_str().unwrap().to_string(),
                    url: url,
                    date_added: j.get("date_added").unwrap().as_str().unwrap().to_string(),
                    index,
                });
            }
            break;
        }
    }
    return result_data.clone();
}

pub(crate) fn convert_date(j: String) -> DelayedFormat<StrftimeItems<'static>> {
    let date_format_start = chrono::Local.with_ymd_and_hms(1601, 1, 1, 0, 0, 0).unwrap();

    return (date_format_start + Duration::microseconds(j.parse::<i64>().unwrap()))
        .format("%d-%m-%Y");
}

pub(crate) async fn get_html(client:Client, url: &str) -> String {
    // let mut rng = rand::random_range(0..3);
    // tokio::time::sleep(tokio::time::Duration::from_secs(rng)).await;

    // return "".to_string();
    match client.get(url).send().await {
        Ok(response) => match response.text().await {
            Ok(html_text) => {
                if is_bot_block(html_text.as_str(), 200){
                    println!("bot site");
                }
                return html_text;
            }
            Err(e) => {
                println!("{:#?}", e);
                return "".to_owned();
            }
        },
        Err(e) => {
            println!("{:#?}", e);
            return "".to_owned();
        }
    }
}

fn is_bot_block(html: &str, status: u16) -> bool {
    // 1. Obvious text markers
    let text_match = html.contains("Enable JavaScript and cookies") ||
                     html.contains("Just a moment") ||
                     html.contains("Checking your browser") ||
                     html.contains("cf-browser-verification");

    // 2. Very small / suspicious HTML
    let suspicious_size = html.len() < 5000;

    // 3. Missing expected content (you KNOW what should be there)
    let missing_target = !html.contains("expected-product-title-or-selector");

    // 4. HTTP status anomalies
    let bad_status = status == 403 || status == 503;

    text_match || (suspicious_size && missing_target) || bad_status
}

pub(crate) fn load_site_parsers(site_parser_rules_file: PathBuf) -> SiteParserConfig {
    let contents = fs::read_to_string(site_parser_rules_file)
        .unwrap_or_else(|_| {
            eprintln!("Failed to read site_parsers.json, using default config");
            r#"{"sites": []}"#.to_string()
        });
    
    serde_json::from_str(&contents)
        .unwrap_or_else(|_| SiteParserConfig { sites: vec![] })
}

pub(crate) fn get_remaining_chapters(response: String, site: String, site_parser_rules_file: PathBuf) -> RemainingChapters {
    let config = load_site_parsers(site_parser_rules_file);
    
    // Find the matching site parser in config
    let parser = config.sites.iter()
        .find(|p| p.name == site);
    
    match parser {
        Some(p) => {
            println!("Found parser for site: {}", site);
            match p.parser_type.as_str() {
                "html_select" => parse_html_select(&response, &p),
                "json" => parse_json(&response, &p),
                _ => RemainingChapters {
                    total: 0,
                    remaining: 0,
                },
            }
        }
        None => {
            println!("No parser found for site: {}\n({:#?})", site, parser);
            RemainingChapters {
                total: 0,
                remaining: 0,
            }
        }
    }
}

fn parse_html_select(response: &str, parser: &SiteParser) -> RemainingChapters {
    let start_pos = match response.find(&parser.selector) {
        Some(pos) => pos + parser.offset,
        None => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let response_2 = &response[start_pos..];
    let end_bytes = match response_2.find("</select>") {
        Some(pos) => pos,
        None => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let response_3 = &response_2[..end_bytes];

    let dom = match Dom::parse(response_3) {
        Ok(d) => d,
        Err(_) => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let mut selected_index = 0;

    for (i, option) in dom.children.iter().enumerate() {
        if let Some(ele) = option.element() {
            if ele.attributes.contains_key("selected") {
                selected_index = i;
            }
        }
    }

    let total = dom.children.len();
    let remaining = if parser.order.to_lowercase() == "descending" {
        selected_index
    } else {
        total.saturating_sub(selected_index + 1)
    };

    RemainingChapters {
        total,
        remaining,
    }
}

fn parse_json(response: &str, parser: &SiteParser) -> RemainingChapters {
    let start_pos = match response.find(&parser.json_key) {
        Some(pos) => {
            if pos >= 3 { pos - 3 } else { 0 }
        },
        None => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let response_2 = &response[start_pos..];
    let end_bytes = match response_2.find("}]") {
        Some(pos) => pos + 2,
        None => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let response_3 = &response_2[..end_bytes];
    let mut response_4 = response_3.replace("\\\"", "\"");
    response_4.push('}');

    let chapter_obj: Value = match serde_json::from_str(&response_4) {
        Ok(obj) => obj,
        Err(_) => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let current_chapter = match chapter_obj.get(&parser.json_key)
        .and_then(|v| v.as_str()) {
        Some(ch) => ch,
        None => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let chapter_list = match chapter_obj.get(&parser.json_chapter_list)
        .and_then(|v| v.as_array()) {
        Some(list) => list,
        None => return RemainingChapters {
            total: 0,
            remaining: 0,
        },
    };

    let index = chapter_list
        .iter()
        .position(|r| r.get(&parser.json_label).map_or(false, |v| v == current_chapter))
        .unwrap_or(0);

    let total = chapter_list.len();
    let remaining = if parser.order.to_lowercase() == "descending" {
        index
    } else {
        total.saturating_sub(index + 1)
    };

    RemainingChapters {
        total,
        remaining,
    }
}
