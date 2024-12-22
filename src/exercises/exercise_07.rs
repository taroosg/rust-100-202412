/// 割り算
// 整数 m と n を入力し、m を n で割り算した商と余りを半角スペース区切りの文字列で出力する関数を作成してください。
pub fn get_quotient_and_surplus(m: i32, n: i32) -> Option<String> {
    Some(format!("{} {}", m / n, m % n))
}

#[cfg(test)]
mod tests_get_quotient_and_surplus {
    use super::*;

    #[test]
    fn test_get_quotient_and_surplus() {
        assert_eq!(get_quotient_and_surplus(10, 3), Some("3 1".to_string()));
        assert_eq!(get_quotient_and_surplus(12, 14), Some("0 12".to_string()));
    }
}
