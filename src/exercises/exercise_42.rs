/// 衣替え
// 与えられた服の情報から夏物が5着以上あるかを判定する関数を作成してください。
pub fn is_ready_for_summer(s: &str) -> String {
    let summer_count = s.split(" ").filter(|&x| x == "S").count();
    match summer_count {
        5..=10 => "OK".to_string(),
        _ => "NG".to_string(),
    }
}

#[cfg(test)]
mod tests_is_ready_for_summer {
    use super::*;

    #[test]
    fn test_is_ready_for_summer() {
        assert_eq!(is_ready_for_summer("W W W W W S S S S S"), "OK");
        assert_eq!(is_ready_for_summer("S S W W S W W W W W"), "NG");
        assert_eq!(is_ready_for_summer("W S S S S S S S S W"), "OK");
    }
}
