#[derive(Queryable, Debug, PartialEq)]
pub struct Ad {
    pub id: usize,
    pub url: String,
    pub num_prepaid_shows: usize,
    pub categories: Vec<String>,
}
