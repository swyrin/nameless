use aho_corasick::AhoCorasick;

use std::{collections::HashMap, sync::LazyLock};

/// vip
pub static VIPHURIT: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    HashMap::from([
        ("v1p", "Vip(hurit)"),
        ("vip", "Vip(hurit)"),
        ("víp", "Vip(hurit)"),
        // -------------------------
        ("job", "j*b"),
        // -------------------------
        ("cc", "Shyload"),
        ("cac", "Shyload"),
        ("concac", "Shyload"),
        ("cặc", "Shyload"),
        ("cằc", "Shyload"),
        // -------------------------
        ("cl", "Shyloap"),
        ("cai lon", "Shyloap"),
        // -------------------------
        ("gay vai lon", "ShyLoas"),
        // -------------------------
        ("clear", "còn Cypher"),
        // -------------------------
        ("nắc", "Kaiser"),
        ("cái đéo má", "Kaiser"),
        ("cai deo ma", "Kaiser"),
        ("cais deos mas", "Kaiser"),
        // -------------------------
        ("dâm", "K"),
        ("fudi", "K"),
        // -------------------------
        ("tsm", "gay"),
        // -------------------------
        ("công việc", "c*ng vi*c"),
        // -------------------------
        ("<@&1151518720141832342>", "mấy con ckó ngu"),
        ("<@&1534192990464114758>", "mấy con ckó ngu"),
        // -------------------------
        ("bán mikè", "@chị G"),
        ("bán mike", "@chị G"),
        ("ban mike", "@chị G"),
        ("bán mice", "@chị G"),
        ("bán <@455951762470338560>", "@chị G"),
        // -------------------------
        ("valorant", "game rác"),
        ("valoran", "game rác"),
        ("viliran", "game rác"),
        ("a9", "game rác"),
        ("arknights", "game rác"),
        ("arknight", "game rác"),
        ("uma", "game rác"),
        // -------------------------
        ("main", "trong site"),
        // -------------------------
        ("r6s", "Rainbow Six Sex"),
        ("siege", "Rainbow Six Sex"),
        ("r6", "Rainbow Six Sex"),
        // -------------------------
        ("rrq", "BBQ"),
        // -------------------------
        ("t1", "TLiệt"),
        // -------------------------
        ("aririn", "mẹ Aririn"),
        // -------------------------
        ("adi", "Thánh Mẫu Adi"),
        // -------------------------
        ("làm", "l*m"),
        // -------------------------
        ("work", "w*rk"),
        // -------------------------
        ("việc", "v*ệc"),
        // -------------------------
    ])
});

#[must_use]
pub fn make_viphurit(input: &(impl ToString + ?Sized)) -> String {
    let input = input.to_string();

    let patterns = VIPHURIT.keys().collect::<Vec<_>>();
    let replaces = VIPHURIT.values().collect::<Vec<_>>();

    let ac = AhoCorasick::new(patterns).expect("Malformed dictionary.");

    // https://discord.com/channels/1025394048204275732/1099318720259690599/1535290571390984222
    ac.replace_all(&input, &replaces)
}

#[cfg(test)]
mod test {
    use super::make_viphurit;

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
