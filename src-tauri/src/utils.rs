use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Duration, TimeZone};
use serde_json::Value;
use std::fs;

#[derive(Clone, Debug)]
pub(crate) struct BookmarkData {
    pub(crate) url: String,
    pub(crate) name: String,
    pub(crate) date_added: String,
}

pub(crate) fn read_bookmark() -> Vec<BookmarkData> {
    let mut result_data: Vec<BookmarkData> = vec![];
    let contents = fs::read_to_string("C:\\Users\\mymai\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\Bookmarks")
    .expect("Should have been able to read the file");

    let object: Value = serde_json::from_str(&contents).unwrap();

    let data = object
        .get("roots")
        .unwrap()
        .get("bookmark_bar")
        .unwrap()
        .get("children")
        .unwrap();

    for i in data.as_array().unwrap() {
        if i.get("name").unwrap() == "anime" {
            let all_anime = i.get("children").unwrap().as_array().unwrap();

            for j in all_anime.iter() {
                if j.get("url")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .contains("chapmanganato")
                {
                    result_data.push(BookmarkData {
                        name: j.get("name").unwrap().as_str().unwrap().to_string(),
                        url: j.get("url").unwrap().as_str().unwrap().to_string(),
                        date_added: j.get("date_added").unwrap().as_str().unwrap().to_string(),
                    })
                }
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
    return reqwest::get(url).await.unwrap().text().await.unwrap();
}

pub(crate) fn get_remaining_chapters(response: String) -> String {
    let all_chapters = get_all_chapters(response.clone());
    let current_chapter = get_current_chapter(response.clone());

    if all_chapters.contains(&current_chapter) {
        return ((all_chapters.len() - 1)
            - all_chapters
                .iter()
                .position(|r| r.eq(&current_chapter))
                .unwrap())
        .to_string();
    }
    return '0'.to_string();
}

fn get_all_chapters(response: String) -> Vec<String> {
    let start_pos = response
        .find("<select class=\"navi-change-chapter\">")
        .unwrap_or(0)
        + 36;
    let end_bytes = response.find("</select>").unwrap_or(response.len());

    let response_2: Vec<_> = response[start_pos..end_bytes].split(">").collect();

    let response_3 = response_2
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0);

    let mut response_4: Vec<String> = vec![];

    response_3.clone().for_each(|(_i, chapter_name)| {
        let str_end_index = chapter_name.find("<");
        match str_end_index {
            Some(index) => response_4.push(chapter_name[0..index].to_owned()),
            None => response_4.push(chapter_name.to_owned()),
        }
    });

    response_4.reverse();

    return response_4;
}

pub(crate) fn get_current_chapter(response: String) -> String {
    let start_pos = response.find("selected >").unwrap_or(0) + 10;

    let response_2 = &response[start_pos..response.len()];
    let end_bytes = response_2.find("</").unwrap_or(response.len());
    let response_3 = &response_2[0..end_bytes];

    return response_3.to_owned();
}
