/// 調理計画
// 夕食の時間と調理時間から、調理開始時間を計算する関数を作成してください。
pub fn get_start_time_for_cook(s: &str, t: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests_get_start_time_for_cook {
    use super::*;

    #[test]
    fn test_get_start_time_for_cook() {
        assert_eq!(get_start_time_for_cook("19:00", 90), "17:30");
        assert_eq!(get_start_time_for_cook("20:00", 20), "19:40");
        assert_eq!(get_start_time_for_cook("20:30", 80), "19:10");
    }
}
