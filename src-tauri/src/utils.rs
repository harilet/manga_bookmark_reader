use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{Duration, TimeZone};
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
                let url = j.get("url").unwrap().as_str().unwrap();
                if url.contains("chapmanganato") {
                    let name = j.get("name").unwrap().as_str().unwrap();
                    let new_url = name_to_url(name);

                    index += 1;
                    result_data.push(BookmarkData {
                        name: j.get("name").unwrap().as_str().unwrap().to_string(),
                        url: new_url,
                        date_added: j.get("date_added").unwrap().as_str().unwrap().to_string(),
                        index,
                    });
                } else if url.contains("chapmanganato")
                    || url.contains("natomanga")
                    || url.contains("manganato")
                {
                    index += 1;
                    result_data.push(BookmarkData {
                        name: j.get("name").unwrap().as_str().unwrap().to_string(),
                        url: j.get("url").unwrap().as_str().unwrap().to_string(),
                        date_added: j.get("date_added").unwrap().as_str().unwrap().to_string(),
                        index,
                    });
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
                println!("{:#?}",e);
                return "".to_owned();
            }
        },
        Err(e) => {
            println!("{:#?}",e);
            return "".to_owned();
        }
    }
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
            Some(index) => response_4.push(chapter_name[0..index].trim().to_owned()),
            None => response_4.push(chapter_name.to_owned()),
        }
    });

    response_4.reverse();

    return response_4;
}

pub(crate) fn get_current_chapter(response: String) -> String {
    if response.contains("navi-change-chapter-btn") {
        // for chapmanganato
        let start_pos = response.find("selected >").unwrap_or(0) + 10;
        let response_2 = &response[start_pos..response.len()];
        let end_bytes = response_2.find("</").unwrap_or(response.len());
        let response_3 = &response_2[0..end_bytes];

        return response_3.trim().to_owned();
    } else if response.contains("btn-navigation-chap") {
        // for natomanga
        let start_pos = response.find("selected>").unwrap_or(0) + 10;
        let response_2 = &response[start_pos..response.len()];
        let end_bytes = response_2.find("</").unwrap_or(response.len());
        let response_3 = &response_2[0..end_bytes];

        return response_3.trim().to_owned();
    } else {
        return "".to_owned();
    }
}
