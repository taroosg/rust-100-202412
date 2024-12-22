/// 奇数か偶数か
// 正の整数 n が入力されるので、n が奇数なら "odd"、偶数なら "even" を出力する関数を作成してください。
pub fn odd_or_even(n: u32) -> String {
    match n % 2 {
        0 => "even".to_string(),
        _ => "odd".to_string(),
    }
}

#[cfg(test)]
mod tests_odd_or_even {
    use super::*;

    #[test]
    fn test_odd_or_even() {
        assert_eq!(odd_or_even(4), "even");
        assert_eq!(odd_or_even(5), "odd");
        assert_eq!(odd_or_even(2), "even");
    }
}
