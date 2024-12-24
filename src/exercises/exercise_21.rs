/// n までの和
// ある 2 以上の整数 n が与えられるので、1 から n までの和を出力する関数を作成してください。
pub fn get_sum(n: u32) -> u32 {
    (1 + n) * n / 2
}

#[cfg(test)]
mod tests_get_sum {
    use super::*;

    #[test]
    fn test_get_sum() {
        assert_eq!(get_sum(10), 55);
        assert_eq!(get_sum(99), 4950);
    }
}
