#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rand::distributions::Distribution;
use rand::thread_rng;
use rocket::http::RawStr;
use serde::Serialize;

use ads_roller::ads::get_distribution;
use ads_roller::db::{count_ads, decrement_ad_prepaid_shows, get_connection, select_matching_ads};

#[derive(Serialize)]
struct AdUrl {
    url: String,
}

#[get("/?<category1>&<category2>&<category3>")]
fn fetch_ad(
    category1: Option<&RawStr>,
    category2: Option<&RawStr>,
    category3: Option<&RawStr>,
) -> String {
    let selected_categories: Vec<String> = vec![category1, category2, category3]
        .iter()
        .filter(|cat| cat.is_some())
        .map(|cat| cat.unwrap().to_string())
        .collect();
    let conn = get_connection();

    let ads = select_matching_ads(selected_categories, &conn);
    let total_ads = count_ads(&conn);

    let dist = get_distribution(&ads, total_ads, None);
    let mut rng = thread_rng();

    let ad_index = dist.sample(&mut rng);
    let selected_ad = ads.get(ad_index).unwrap();
    decrement_ad_prepaid_shows(selected_ad, &conn);

    let ad_url = AdUrl {
        url: selected_ad.url.clone(),
    };
    serde_json::to_string(&ad_url).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![fetch_ad]).launch();
}
