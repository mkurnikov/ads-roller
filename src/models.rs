#[derive(Queryable, Debug, PartialEq)]
pub struct Ad {
    pub id: i32,
    pub url: String,
    pub num_prepaid_shows: i32,
    pub categories: Vec<String>,
}
