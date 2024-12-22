/// 通知票
// 数字の成績をアルファベットに変換する関数を作成してください。
pub fn convert_number_to_alphabet(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_convert_number_to_alphabet {
    use super::*;

    #[test]
    fn test_convert_number_to_alphabet() {
        assert_eq!(convert_number_to_alphabet("53342"), "ACCBD");
        assert_eq!(convert_number_to_alphabet("22222"), "DDDDD");
        assert_eq!(convert_number_to_alphabet("51111"), "AEEEE");
    }
}
