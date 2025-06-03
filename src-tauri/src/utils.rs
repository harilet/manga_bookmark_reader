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
                match j.get("url") {
                    Some(url_value) => {
                        let url = url_value.as_str().unwrap();
                        if url.contains("chapmanganato") {
                            let name = j.get("name").unwrap().as_str().unwrap();
                            let new_url = name_to_url(name);

                            index += 1;
                            result_data.push(BookmarkData {
                                name: j.get("name").unwrap().as_str().unwrap().to_string(),
                                url: new_url,
                                date_added: j
                                    .get("date_added")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                                index,
                            });
                        } else if url.contains("asuracomic")
                            || url.contains("natomanga")
                            || url.contains("manganato")
                            || url.contains("manhwaclan")
                            || url.contains("mangadex")
                        {
                            index += 1;
                            result_data.push(BookmarkData {
                                name: j.get("name").unwrap().as_str().unwrap().to_string(),
                                url: j.get("url").unwrap().as_str().unwrap().to_string(),
                                date_added: j
                                    .get("date_added")
                                    .unwrap()
                                    .as_str()
                                    .unwrap()
                                    .to_string(),
                                index,
                            });
                        }
                    }
                    None => {}
                }
            }
            break;
        }
    }
    return result_data.clone();
}

fn name_to_url(name: &str) -> String {
    let end_pos = name.find(" Chapter").unwrap();
    let res_name1_temp: &String = &name[..end_pos]
        .chars()
        .filter(|c| c.is_alphanumeric() || c == &'-' || c == &' ')
        .collect();
    let res_name1 = res_name1_temp.replace(" ", "-").to_ascii_lowercase();

    let name12 = &name[name.find("Chapter").unwrap()..];
    let temp: Vec<_> = name12.match_indices(" ").collect();
    let res_name2 = &name12[8..temp.get(1).unwrap().0]
        .replace(" ", "-")
        .replace(".", "-")
        .replace(":", "");

    return format!(
        "https://www.manganato.gg/manga/{}/chapter-{}",
        res_name1, res_name2
    );
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
