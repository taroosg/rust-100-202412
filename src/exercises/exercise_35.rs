/// 天気の表示
// 降水確率に応じた天気を表示する関数を作成してください。
pub fn show_weather(n: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests_show_weather {
    use super::*;

    #[test]
    fn test_show_weather() {
        assert_eq!(show_weather(31), "cloudy");
        assert_eq!(show_weather(2), "sunny");
        assert_eq!(show_weather(71), "rainy");
    }
}
