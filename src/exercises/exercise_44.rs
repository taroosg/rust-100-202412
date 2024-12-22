/// トリック・オア・トリート
// 入力文字列が "candy" または "chocolate" の場合に "Thanks!" を、それ以外の場合に "No!" を返す関数を作成してください。
pub fn candy_or_chocolate(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_candy_or_chocolate {
    use super::*;

    #[test]
    fn test_candy_or_chocolate() {
        assert_eq!(candy_or_chocolate("chocolate"), "Thanks!");
        assert_eq!(candy_or_chocolate("candy"), "Thanks!");
        assert_eq!(candy_or_chocolate("pannacotta"), "No!");
    }
}
