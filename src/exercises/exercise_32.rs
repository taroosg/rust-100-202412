/// 試合の回数
// 総当り対戦での対戦回数を計算する関数を作成してください。
pub fn calculate_number_of_games(n: u32) -> u32 {
    (n * (n - 1)) / 2
}

#[cfg(test)]
mod tests_calculate_number_of_games {
    use super::*;

    #[test]
    fn test_calculate_number_of_games() {
        assert_eq!(calculate_number_of_games(4), 6);
        assert_eq!(calculate_number_of_games(10), 45);
    }
}
