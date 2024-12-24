/// 文字列から N 番目
// 半角アルファベット文字列 s と整数 n を入力し、s の n 文字目を出力する関数を作成してください。
pub fn get_nth_char(s: &str, n: usize) -> Option<char> {
    s.chars().nth(n - 1)
}

#[cfg(test)]
mod tests_get_nth_char {
    use super::*;

    #[test]
    fn test_get_nth_char() {
        assert_eq!(get_nth_char("gsacademy", 3), Some('a'));
        assert_eq!(get_nth_char("abcdefg", 5), Some('e'));
        assert_eq!(get_nth_char("qwertyu", 1), Some('q'));
    }
}
