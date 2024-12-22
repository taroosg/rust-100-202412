/// 連休の天気
// 降水確率が 30% 以下の日数を数える関数を作成してください。
pub fn days_of_go_out(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_days_of_go_out {
    use super::*;

    #[test]
    fn test_days_of_go_out() {
        assert_eq!(days_of_go_out("13 0 15 30 89 100 30"), 5);
        assert_eq!(days_of_go_out("0 14 18 22 0 2 4"), 7);
        assert_eq!(days_of_go_out("99 99 99 99 99 99 99"), 0);
    }
}
