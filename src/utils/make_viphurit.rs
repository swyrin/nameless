use std::collections::HashMap;
use std::sync::LazyLock;

/// vip
static VIPHURIT: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
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

    // https://discord.com/channels/1025394048204275732/1099318720259690599/1535290571390984222
    input
        .split_whitespace()
        .map(|mut word| {
            let lower = word.to_lowercase();
            let lower = lower.as_str();

            if let Some(replace) = VIPHURIT.get(lower) {
                word = *replace;
            }

            word
        })
        .collect::<Vec<&str>>()
        .join(" ")
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
        assert_eq!(make_viphurit("Vip(hurit) la nhat"), "Vip(hurit) la nhat");
    }

    #[test]
    fn vip_prevent_overlaps() {
        assert_ne!(make_viphurit("Shyloadcc"), "cccc");
    }

    #[test]
    fn vip_case_insensitive() {
        assert_eq!(make_viphurit("vip"), "Vip(hurit)");
        assert_eq!(make_viphurit("Vip"), "Vip(hurit)");
    }
}
