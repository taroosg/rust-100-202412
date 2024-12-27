/// ピラミッドの作り方
// 段数 n のピラミッドを組むのに必要な人数を計算する関数を作成してください。
pub fn get_number_of_human(n: u32) -> u32 {
    (1 + n) * n / 2
}

#[cfg(test)]
mod tests_get_number_of_human {
    use super::*;

    #[test]
    fn test_get_number_of_human() {
        assert_eq!(get_number_of_human(4), 10);
        assert_eq!(get_number_of_human(10), 55);
        assert_eq!(get_number_of_human(50), 1275);
    }
}
