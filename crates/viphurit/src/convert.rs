use crate::dictionary;
use aho_corasick::AhoCorasick;

#[must_use]
pub fn make_viphurit(input: impl ToString) -> String {
    let input = input.to_string();

    let patterns = dictionary::VIPHURIT.keys().collect::<Vec<_>>();
    let replaces = dictionary::VIPHURIT.values().collect::<Vec<_>>();

    let ac = AhoCorasick::new(patterns).unwrap();

    // https://discord.com/channels/1025394048204275732/1099318720259690599/1535290571390984222
    ac.replace_all(&input, &replaces)
}

#[cfg(test)]
mod test {
    use crate::convert::make_viphurit;

    #[test]
    fn vip() {
        assert_eq!(make_viphurit("phum vip"), "phum Vip(hurit)");
        assert_eq!(make_viphurit("cac"), "Shyload");
        assert_eq!(
            make_viphurit("nhìn tưởng a9 nhưng lại gần thì lại là lưu trữ genshjit"),
            "nhìn tưởng game rác nhưng lại gần thì lại là lưu trữ genshjit"
        );
    }

    #[test]
    fn vip_preserve_og() {
        assert_eq!(make_viphurit("Vip(hurit)"), "Vip(hurit)");
    }
}
