use std::env;

use crate::models::Ad;
use diesel::prelude::*;
use dotenv::dotenv;

pub fn get_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


pub fn select_matching_ads(queried_categories: Vec<String>, conn: &PgConnection) -> Vec<Ad> {
    use schema::ads::dsl::*;

    ads.filter(
        categories
            .overlaps_with(queried_categories)
            .and(num_prepaid_shows.gt(0)),
    )
    .load::<Ad>(conn)
}
