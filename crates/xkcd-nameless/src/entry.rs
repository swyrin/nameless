use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct XkcdEntry {
    pub month: String,
    pub num: u32,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub extra_parts: Option<serde_json::Value>,
    pub day: String,
}
