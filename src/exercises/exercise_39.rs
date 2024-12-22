/// 台風の間隔
// 台風の上陸日から間隔日数を計算する関数を作成してください。
pub fn get_interval(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_get_interval {
    use super::*;

    #[test]
    fn test_get_interval() {
        assert_eq!(get_interval("5 8 19 25 31"), "3 11 6 6");
        assert_eq!(get_interval("2 3 7 9 28"), "1 4 2 19");
    }
}
