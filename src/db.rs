use std::env;

use diesel::expression::dsl::count;
use diesel::prelude::*;

use dotenv::dotenv;

use crate::models::Ad;

pub fn get_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn select_matching_ads(queried_categories: Vec<String>, conn: &PgConnection) -> Vec<Ad> {
    use crate::schema::ads::dsl::*;

    let categories_not_provided = queried_categories.is_empty();
    let query = ads.filter(
        num_prepaid_shows.gt(0).and(
            categories
                .contains(&queried_categories)
                .or(categories_not_provided),
        ),
    );
    query.load::<Ad>(conn).unwrap()
}

pub fn count_ads(conn: &PgConnection) -> usize {
    use crate::schema::ads::dsl::*;
    ads.select(count(id)).first::<i64>(conn).unwrap() as usize
}

pub fn decrement_ad_prepaid_shows(ad: &Ad, conn: &PgConnection) {
    use crate::schema::ads::dsl::*;

    let affected_rows = diesel::update(ads.filter(id.eq(ad.id)))
        .set(num_prepaid_shows.eq(num_prepaid_shows - 1))
        .execute(conn)
        .unwrap();
    assert_eq!(affected_rows, 1)
}
