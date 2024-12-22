/// N 文字目まで出力
// 半角アルファベットで構成された文字列 s と正の整数 n を入力し、1 文字目から n 文字目までを出力する関数を作成してください。
pub fn get_one_to_n(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

#[cfg(test)]
mod tests_get_one_to_n {
    use super::*;

    #[test]
    fn test_get_one_to_n() {
        assert_eq!(get_one_to_n("aabbccdd", 5), "aabbc");
        assert_eq!(get_one_to_n("gsacademyfukuoka", 9), "gsacademy");
    }
}
