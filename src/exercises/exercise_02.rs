/// 単位の計算
// 距離 n とその単位 s が入力されるので、すべての距離を mm に換算する関数を作成してください。
// 入力される単位は "km", "m", "cm" の 3 種類です。
pub fn convert_to_mm(n: f64, s: &str) -> Option<f64> {
    todo!()
}

#[cfg(test)]
mod tests_convert_to_mm {
    use super::*;

    #[test]
    fn test_convert_to_mm() {
        assert_eq!(convert_to_mm(1.0, "km"), Some(1_000_000.0));
        assert_eq!(convert_to_mm(54.0, "km"), Some(54_000_000.0));
        assert_eq!(convert_to_mm(2.0, "cm"), Some(20.0));
        assert_eq!(convert_to_mm(1.0, "mm"), None); // 無効な単位
    }
}
