/// 絶対値
// -100 から 100 までの整数 n が与えられるので、絶対値を出力する関数を作成してください。
pub fn get_abs(n: i32) -> i32 {
    match n {
        n if n < 0 => n * -1,
        _ => n,
    }
}

#[cfg(test)]
mod tests_get_abs {
    use super::*;

    #[test]
    fn test_get_abs() {
        assert_eq!(get_abs(-10), 10);
        assert_eq!(get_abs(15), 15);
        assert_eq!(get_abs(0), 0);
    }
}
