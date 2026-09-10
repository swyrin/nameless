use reqwest::get;

use crate::extra::xkcd::{
    entry::XkcdEntry,
    util::{get_xkcd_url, pick},
};

/// Generic fetch function of an XKCD entry.
async fn fetch(number: Option<u64>) -> Result<XkcdEntry, reqwest::Error> {
    let url = get_xkcd_url(number);
    let resp = get(url.clone()).await?.error_for_status();

    match resp {
        Ok(response) => {
            let text = response.text().await?;
            let data: XkcdEntry = serde_json::from_str(&text).unwrap();

            Ok(data)
        }
        Err(err) => Err(err),
    }
}

/// Fetch a specific XKCD entry.
pub async fn fetch_number(number: u64) -> Result<XkcdEntry, reqwest::Error> {
    fetch(Some(number)).await
}

/// Fetch the lastest XKCD entry.
pub async fn fetch_latest() -> Result<XkcdEntry, reqwest::Error> {
    fetch(None).await
}

/// Fetch a random XKCD entry.
pub async fn fetch_random() -> Result<XkcdEntry, reqwest::Error> {
    let latest_entry = fetch_latest().await?;

    fetch(Some(pick(1..=latest_entry.num))).await
}

#[cfg(test)]
mod test {
    // would you like a 0th of Jan
    // when we kissed under a mistletoe at 25:71 PM?
    use super::*;

    #[tokio::test]
    async fn test_xkcd_get_url() {
        assert_eq!(
            get_xkcd_url(Some(2928)),
            "https://xkcd.com/2928/info.0.json"
        );

        assert_eq!(get_xkcd_url(None), "https://xkcd.com/info.0.json");
    }

    #[tokio::test]
    async fn test_xkcd_fetch_specific() {
        // at the time of development
        // I found another one: https://crates.io/crates/xkcd
        //
        // https://xkcd.com/927/
        assert!(fetch_number(927).await.is_ok());
    }

    #[tokio::test]
    async fn test_xkcd_fetch_non_existent() {
        assert!(fetch_number(0).await.is_err());
    }

    #[tokio::test]
    async fn test_xkcd_fetch_guaranteed() {
        assert!(fetch_number(1053).await.is_ok());
        assert!(fetch_random().await.is_ok());
    }
}
