/// A の個数
// 半角アルファベットで構成された文字列 s に含まれる A の数を出力する関数を作成してください。
pub fn get_number_of_a(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_get_number_of_a {
    use super::*;

    #[test]
    fn test_get_number_of_a() {
        assert_eq!(get_number_of_a("GSACADEMY"), 2);
        assert_eq!(get_number_of_a("aAaAaA"), 3);
        assert_eq!(get_number_of_a("JavaScript"), 0);
    }
}
