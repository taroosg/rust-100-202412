/// 11/11
// 入力された文字列の "1" の本数が11以上の場合に "OK" を、それ以外の場合に不足本数を返す関数を作成してください。
pub fn can_party(s: &str) -> String {
    let len = s.chars().filter(|&x| x == '1').count();
    match len {
        x if x >= 11 => "OK".to_string(),
        _ => (11 - len).to_string(),
    }
}

#[cfg(test)]
mod tests_can_party {
    use super::*;

    #[test]
    fn test_can_party() {
        assert_eq!(can_party("11111111111"), "OK");
        assert_eq!(can_party("1111"), "7");
        assert_eq!(can_party("1111111111111111"), "OK");
    }
}
