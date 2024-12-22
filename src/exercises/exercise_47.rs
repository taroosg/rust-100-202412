/// ガチャ期待値計算
// 当選確率と試行回数から、1回以上当選する確率を計算する関数を作成してください。
pub fn probability_get_ssr(n: f64, t: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_probability_get_ssr {
    use super::*;

    #[test]
    fn test_probability_get_ssr() {
        assert_eq!(probability_get_ssr(1.0, 100), 63);
        assert_eq!(probability_get_ssr(1.5, 70), 65);
        assert_eq!(probability_get_ssr(1.0, 200), 87);
    }
}
