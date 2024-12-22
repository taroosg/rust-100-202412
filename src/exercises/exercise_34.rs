/// 正三角形かどうか
// 三角形が正三角形かを判定する関数を作成してください。
pub fn is_equilateral_triangle(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_is_equilateral_triangle {
    use super::*;

    #[test]
    fn test_is_equilateral_triangle() {
        assert_eq!(is_equilateral_triangle("10 10 10"), "yes");
        assert_eq!(is_equilateral_triangle("3 4 5"), "No");
    }
}
