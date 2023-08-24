use serde::Deserialize;


#[derive(Deserialize)]
pub struct Publication {
    pub id: String,
    pub order: usize,
    pub title: String,
    pub authors: Vec<String>,
    pub year: usize,
    pub venue: String,
    pub abstr: Option<String>,
    pub arxiv: Option<String>,
    pub doi: Option<String>,
    pub code: Option<String>,
    pub datasets: Option<String>,
}