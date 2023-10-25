use serde::Deserialize;
use toml::Table;

#[derive(Deserialize)]
pub struct FrontMatter { 
    author: String,
    description: String, 
    keywords: Vec<String>,
    title: String,
    stylesheets: Vec<String>,
    scripts: Vec<String>
}

#[derive(Deserialize)]
pub struct Header { 
    front: FrontMatter,
    meta: Table
}

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