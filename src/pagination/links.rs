use crate::requests::get_photos_request::GetPhotosRequest;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Links {
    pub first: String,
    pub next: String,
    pub current: String,
    pub previous: String,
    pub last: String,
}

impl Links {
    pub fn new(req: &GetPhotosRequest, total_pages: i64) -> Self {
        let current_page = req.get_page();

        let current_page = if current_page <= 0 {
            1
        } else if current_page >= total_pages {
            total_pages
        } else {
            current_page
        };

        let previous_page = if current_page <= 0 {
            1
        } else {
            current_page - 1
        };

        let (first_link, previous_link) = if current_page == 1 {
            ("".to_string(), "".to_string())
        } else {
            (build_link(1, req), build_link(previous_page, req))
        };

        let (next_link, last_link) = if current_page >= total_pages {
            ("".to_string(), "".to_string())
        } else {
            (
                build_link(current_page + 1, req),
                build_link(total_pages, req),
            )
        };

        let current_link = build_link(current_page, req);

        Links {
            current: current_link,
            first: first_link,
            previous: previous_link,
            next: next_link,
            last: last_link,
        }
    }

    pub fn default() -> Links {
        Links {
            current: String::from(""),
            first: String::from(""),
            next: String::from(""),
            previous: String::from(""),
            last: String::from(""),
        }
    }
}

fn build_link(page: i64, req: &GetPhotosRequest) -> String {
    let mut url = build_host_url();

    let page_size = req.get_page_size();
    let sort_by = req.get_sort_by();
    let folder = &req.get_folder();

    url.query_pairs_mut()
        .append_pair("page", format!("{}", page).as_str())
        .append_pair("page_size", format!("{}", page_size).as_str());

    if let Some(sort_by) = sort_by {
        url.query_pairs_mut()
            .append_pair("sort_by", &sort_by.join(","));
    }

    //    if req.ineligible_wallpaper.is_some() {
    //        url.query_pairs_mut().append_pair(
    //            "ineligible_wallpaper",
    //            &req.ineligible_wallpaper.unwrap().to_string(),
    //        );
    //    }

    //    if req.hidden.is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("hidden", &req.hidden.unwrap().to_string());
    //    }
    //
    url.query_pairs_mut().append_pair("folder", folder);
    //
    //    if req.get_raw_ignore_folders().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("ignore_folders", &req.get_raw_ignore_folders().unwrap());
    //    }
    //
    //    if req.get_raw_tags().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("tags", &req.get_raw_tags().unwrap());
    //    }
    //
    //    if req.get_raw_ignore_tags().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("ignore_tags", &req.get_raw_ignore_tags().unwrap());
    //    }
    //
    //    if req.get_raw_people().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("people", &req.get_raw_people().unwrap());
    //    }
    //
    //    if req.get_raw_ignore_people().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("ignore_people", &req.get_raw_ignore_people().unwrap());
    //    }

    url.into_string()
}

fn build_host_url() -> Url {
    let host =
        env::var("SCARLETT_HOSTNAME").expect("SCARLETT_HOSTNAME environment variable not set");
    Url::parse(format!("https://{}/photos", host).as_str()).unwrap()
}
