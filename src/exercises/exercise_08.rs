/// 小文字を大文字に
// 半角アルファベットの小文字で構成された文字列 s を入力し、大文字に変換する関数を作成してください。
pub fn convert_to_uppercase(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_convert_to_uppercase {
    use super::*;

    #[test]
    fn test_convert_to_uppercase() {
        assert_eq!(convert_to_uppercase("gsacademy"), "GSACADEMY");
        assert_eq!(
            convert_to_uppercase("abcdefghijklmnopqrstuvwxyz"),
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        );
    }
}
