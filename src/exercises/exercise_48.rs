/// 調理計画
// 夕食の時間と調理時間から、調理開始時間を計算する関数を作成してください。
pub fn get_start_time_for_cook(s: &str, t: u32) -> String {
    let time: Vec<&str> = s.split(":").collect();
    let minutes = time[0].parse::<u32>().unwrap() * 60 + time[1].parse::<u32>().unwrap();
    let start_time = (1440 + minutes - t) % 1440;
    let start_hour = start_time / 60;
    let start_minute = start_time % 60;
    format!("{:02}:{:02}", start_hour, start_minute)
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
