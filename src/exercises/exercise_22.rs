/// 数字の桁数
// ある 10 進数の正の整数 n が与えられるので、n が何桁かを数字で出力する関数を作成してください。
pub fn get_number_of_digits(n: u32) -> usize {
    n.to_string().len()
}

#[cfg(test)]
mod tests_get_number_of_digits {
    use super::*;

    #[test]
    fn test_get_number_of_digits() {
        assert_eq!(get_number_of_digits(100), 3);
        assert_eq!(get_number_of_digits(114514), 6);
    }
}
