use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Duration, TimeZone};
use html_parser::Dom;
use serde_json::Value;
use std::fs;

#[derive(Clone, Debug)]
pub(crate) struct BookmarkData {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) date_added: String,
    pub(crate) index: usize,
}

pub(crate) fn read_bookmark() -> Vec<BookmarkData> {
    let mut result_data: Vec<BookmarkData> = vec![];
    let contents = fs::read_to_string("C:\\Users\\Asus\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Bookmarks")
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
                        Some(url_str_data) => {
                           if url_str_data.contains("natomanga.com"){
                            url_str_data.replace("natomanga.com", "manganato.gg").to_string()
                           }else{
                            url_str_data.to_string()
                           }
                        },
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

pub(crate) async fn get_html(url: &str) -> String {
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(html_text) => {
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

pub(crate) fn get_remaining_chapters(response: String, site: String) -> String {
    if site == "natomanga/manganato".to_owned() {
        return natomanga_manganato_remaining_chapters(response);
    } else if site == "manhwaclan".to_owned() {
        return manhwaclan_remaining_chapters(response);
    } else if site == "asuracomic".to_owned() {
        return asuracomic_remaining_chapters(response);
    } else {
        return '0'.to_string();
    }
}

fn manhwaclan_remaining_chapters(response: String) -> String {
    let start_pos = response
        .find("<select class=\"selectpicker single-chapter-select\" >")
        .unwrap()
        + 52;

    let response_2 = &response[start_pos..];
    let end_bytes = response_2.find("</select>").unwrap();

    let response_3 = &response_2[..end_bytes];

    let dom = Dom::parse(response_3).unwrap();

    let mut selected_index = 0;

    for (i, option) in dom.children.iter().enumerate() {
        let ele = option.element().unwrap();

        if ele.attributes.contains_key("selected") {
            selected_index = i;
        }
    }

    return selected_index.to_string();
}

fn natomanga_manganato_remaining_chapters(response: String) -> String {
    let start_pos = response
        .find("<select class=\"navi-change-chapter\">")
        .unwrap_or(0)
        + 36;

    let response_2 = &response[start_pos..];
    let end_bytes = response_2.find("</select>").unwrap();

    let response_3 = &response_2[..end_bytes];

    let dom = Dom::parse(response_3).unwrap();

    let mut selected_index = 0;

    for (i, option) in dom.children.iter().enumerate() {
        let ele = option.element().unwrap();

        if ele.attributes.contains_key("selected") {
            selected_index = i;
        }
    }

    return selected_index.to_string();
}

fn asuracomic_remaining_chapters(response: String) -> String {
    let start_pos = response.find("selectedChapter").unwrap_or(0) - 3;
    let response_2 = &response[start_pos..];

    let end_bytes = response_2.find("}]").unwrap() + 2;

    let response_3 = &response_2[..end_bytes];
    let mut response_4 = response_3.replace("\\\"", "\"");
    response_4.push('}');

    let chapter_obj: Value = serde_json::from_str(&response_4).unwrap();

    let current_chapter = chapter_obj
        .get("selectedChapter")
        .unwrap()
        .as_str()
        .unwrap();

    let chapter_list = chapter_obj
        .get("chapterMapData")
        .unwrap()
        .as_array()
        .unwrap();

    let index = chapter_list
        .iter()
        .position(|r| r.get("label").unwrap() == current_chapter)
        .unwrap();

    return index.to_string();
}
