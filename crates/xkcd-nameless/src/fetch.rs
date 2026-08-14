use rand::random_range;
use reqwest::get;

use crate::entry::XkcdEntry;

/// Construct XKCD url.
fn get_xkcd_url(number: Option<u32>) -> String {
    match number {
        Some(x) => format!("https://xkcd.com/{}/info.0.json", x),
        None => String::from("https://xkcd.com/info.0.json"),
    }
}

/// Generic fetch function of an XKCD entry.
async fn fetch(number: Option<u32>) -> anyhow::Result<XkcdEntry> {
    let url = get_xkcd_url(number);
    let resp = get(url.clone()).await?.error_for_status();

    match resp {
        Ok(response) => {
            let text = response.text().await?;
            let data: XkcdEntry = serde_json::from_str(&text).unwrap();

            Ok(data)
        }
        Err(_) => anyhow::bail!(format!("Unable to fetch XKCD url {}", url.clone())),
    }
}

/// Fetch a specific XKCD entry.
pub async fn fetch_number(number: u32) -> anyhow::Result<XkcdEntry> {
    match fetch(Some(number)).await {
        Ok(x) => Ok(x),
        Err(e) => Err(e),
    }
}

/// Fetch the lastest XKCD entry.
pub async fn fetch_latest() -> anyhow::Result<XkcdEntry> {
    match fetch(None).await {
        Ok(x) => Ok(x),
        Err(e) => Err(e),
    }
}

/// Fetch a random XKCD entry.
pub async fn fetch_random() -> anyhow::Result<XkcdEntry> {
    let latest_entry = fetch_latest().await;

    let latest_entry = match latest_entry {
        Ok(entry) => entry,
        Err(_) => panic!("Should be unreachable."),
    };

    let upper_bound = latest_entry.num;

    let range = 1..=upper_bound;

    // https://xkcd.com/221/
    let the_chosen_one = random_range(range);

    match fetch(Some(the_chosen_one)).await {
        Ok(x) => Ok(x),
        Err(err) => Err(err),
    }
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
