use diesel::insert_into;
use diesel::prelude::*;
use serde::Deserialize;

use ads_roller::db::get_connection;

const CONFIG_FPATH: &str = "resources/banners.csv";

#[derive(Deserialize)]
pub struct AdRecord {
    url: String,
    num_prepaid_shows: i32,
    category1: Option<String>,
    category2: Option<String>,
    category3: Option<String>,
    category4: Option<String>,
    category5: Option<String>,
}

impl AdRecord {
    pub fn get_categories(&self) -> Vec<String> {
        vec![
            &self.category1,
            &self.category2,
            &self.category3,
            &self.category4,
            &self.category5,
        ]
        .iter()
        .filter(|cat| cat.is_some())
        .map(|cat| cat.as_ref().unwrap().to_string())
        .collect()
    }
}

pub fn read_records_from_config() -> Vec<AdRecord> {
    let mut rdr = csv::Reader::from_path(CONFIG_FPATH).expect("Cannot read file");
    rdr.deserialize()
        .collect::<Result<Vec<AdRecord>, csv::Error>>()
        .unwrap()
}

pub fn main() {
    use ads_roller::schema::ads::dsl::*;

    let conn = get_connection();

    let records = read_records_from_config();
    for record in records.iter() {
        let record_categories = record.get_categories();
        insert_into(ads)
            .values((
                url.eq(&record.url),
                num_prepaid_shows.eq(record.num_prepaid_shows),
                categories.eq(record_categories.as_slice()),
            ))
            .execute(&conn)
            .unwrap();
    }
}
