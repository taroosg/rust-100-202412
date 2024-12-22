/// 掛け算のリスト
// 正の整数 1 から 9 に整数 n を掛けた数を半角スペース区切りで出力する関数を作成してください。
pub fn create_list(n: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests_create_list {
    use super::*;

    #[test]
    fn test_create_list() {
        assert_eq!(create_list(2), "2 4 6 8 10 12 14 16 18");
        assert_eq!(create_list(3), "3 6 9 12 15 18 21 24 27");
    }
}
