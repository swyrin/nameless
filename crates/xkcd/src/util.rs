use rand::random_range;
use std::ops::RangeInclusive;

/// Construct XKCD url.
pub(crate) fn get_xkcd_url(number: Option<u64>) -> String {
    match number {
        Some(x) => format!("https://xkcd.com/{}/info.0.json", x),
        None => String::from("https://xkcd.com/info.0.json"),
    }
}

/// Pick a number in range.
///
// https://xkcd.com/221/
pub(crate) fn pick(range: RangeInclusive<u64>) -> u64 {
    random_range(range)
}
