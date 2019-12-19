#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::RawStr;
use serde::Serialize;
use ads_roller::db::get_connection;
use ads_roller::ads::get_distribution;

struct AdParams {
    categories: Vec<String>,
}

#[derive(Serialize)]
struct Ad {
    url: String,
}

#[get("/?<category1>&<category2>&<category3>")]
fn fetch_ad(
    category1: Option<&RawStr>,
    category2: Option<&RawStr>,
    category3: Option<&RawStr>,
) -> &str {
    let selected_categories = vec![category1, category2, category3]
        .iter()
        .filter(|cat| cat.is_some())
        .collect();
    let conn = get_connection();
    //    let ad_as_str = serde_json::to_string(&ad).unwrap();
    //    ad_as_str
}

fn main() {
    rocket::ignite().mount("/", routes![fetch_ad]).launch();
}
