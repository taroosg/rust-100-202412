/// 単位の変換
// n 分から秒へ変換する関数を作成してください。
pub fn convert_to_seconds(n: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_convert_to_seconds {
    use super::*;

    #[test]
    fn test_convert_to_seconds() {
        assert_eq!(convert_to_seconds(16), 960);
        assert_eq!(convert_to_seconds(3), 180);
    }
}
