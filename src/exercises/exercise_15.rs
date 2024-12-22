/// 文字列の一致
// 2 つの文字列 s と t が完全一致している場合は "Yes"、それ以外は "No" を出力する関数を作成してください。
pub fn is_equal(s: &str, t: &str) -> String {
    match s == t {
        true => "Yes".to_string(),
        false => "No".to_string(),
    }
}

#[cfg(test)]
mod tests_is_equal {
    use super::*;

    #[test]
    fn test_is_equal() {
        assert_eq!(is_equal("gsacademy", "gsacademy"), "Yes");
        assert_eq!(is_equal("JavaScript", "ジャバスク"), "No");
        assert_eq!(is_equal("aaaaa", "aaaaaa"), "No");
    }
}
