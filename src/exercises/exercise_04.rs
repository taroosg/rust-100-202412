/// 西暦の計算
// 2つの西暦 a と b を入力し、a から b が何年間あるかを出力する関数を作成してください。
pub fn calculate_years(a: u32, b: u32) -> Option<u32> {
    match b - a {
        x if x < 0 => None,
        x => Some(x),
    }
}

#[cfg(test)]
mod tests_calculate_years {
    use super::*;

    #[test]
    fn test_calculate_years() {
        assert_eq!(calculate_years(1990, 2014), Some(24));
        assert_eq!(calculate_years(1999, 2000), Some(1));
        assert_eq!(calculate_years(794, 1192), Some(398));
    }
}
