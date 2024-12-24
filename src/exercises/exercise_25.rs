/// 充電時間
// 現在のスマートフォンのバッテリーの残量 n% から 100% まで充電するのに必要な時間を出力する関数を作成してください。
pub fn get_time_until_full(n: u32) -> u32 {
    100 - n
}

#[cfg(test)]
mod tests_get_time_until_full {
    use super::*;

    #[test]
    fn test_get_time_until_full() {
        assert_eq!(get_time_until_full(70), 30);
        assert_eq!(get_time_until_full(25), 75);
    }
}
