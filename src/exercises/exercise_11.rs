/// カウントダウン
// 正整数 n が入力されるので、n から 1 まで 1 ずつカウントダウンする関数を作成してください。
pub fn count_down(n: u32) -> String {
    (1..=n)
        .rev()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests_count_down {
    use super::*;

    #[test]
    fn test_count_down() {
        assert_eq!(count_down(3), "3 2 1");
        assert_eq!(count_down(10), "10 9 8 7 6 5 4 3 2 1");
    }
}
