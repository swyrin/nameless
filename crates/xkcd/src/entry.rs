use serde::Deserialize;

/// An XKCD entry, from raw JSON.
#[derive(Deserialize, Debug)]
pub struct XkcdEntry {
    /// The entry number.
    pub num: u64,

    /// The day this entry is published.
    #[serde(deserialize_with = "string_to_u64")]
    pub day: u64,

    /// The month this entry is published.
    #[serde(deserialize_with = "string_to_u64")]
    pub month: u64,

    /// The year this entry is published.
    #[serde(deserialize_with = "string_to_u64")]
    pub year: u64,

    /// The link pointing to this XKCD entry.
    pub link: String,

    /// The announcement, think like when Randall
    /// advertises What If?
    pub news: String,

    /// The image, read out.
    /// For some reason newer XKCD entries does not have this.
    pub transcript: String,

    /// The text when hovering on the image.
    pub alt: String,

    /// Image URL.
    pub img: String,

    /// Entry title.
    pub title: String,

    /// The title, but with special characters escaped.
    pub safe_title: String,

    /// Extra data that I have no idea.
    pub extra_parts: Option<serde_json::Value>,
}

fn string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u64>().map_err(serde::de::Error::custom)
}
