/// お月見だんご
// 各子供の要求しただんごの数から、最大5個までしか与えない場合の合計を計算する関数を作成してください。
pub fn get_number_of_dango(s: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_get_number_of_dango {
    use super::*;

    #[test]
    fn test_get_number_of_dango() {
        assert_eq!(get_number_of_dango("3 50"), 8);
        assert_eq!(get_number_of_dango("100 500"), 10);
        assert_eq!(get_number_of_dango("2 4"), 6);
    }
}
