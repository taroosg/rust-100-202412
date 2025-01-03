/// しかたないから全角にしてやるか
// 入力された7桁の郵便番号を全角に変換する関数を作成してください。
pub fn convert_to_full_width(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '0' => '０',
            '1' => '１',
            '2' => '２',
            '3' => '３',
            '4' => '４',
            '5' => '５',
            '6' => '６',
            '7' => '７',
            '8' => '８',
            '9' => '９',
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests_convert_to_full_width {
    use super::*;

    #[test]
    fn test_convert_to_full_width() {
        assert_eq!(convert_to_full_width("1234567"), "１２３４５６７");
        assert_eq!(convert_to_full_width("1111111"), "１１１１１１１");
        assert_eq!(convert_to_full_width("1145140"), "１１４５１４０");
    }
}
