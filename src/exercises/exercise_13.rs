/// 最大と最小
// 半角スペース区切りで与えられた 5 つの数字の最大値と最小値を出力する関数を作成してください。
pub fn get_max_and_min(s: &str) -> Option<String> {
    s.split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .fold(Some((i32::MIN, i32::MAX)), |acc, x| match acc {
            Some((max, min)) => Some((max.max(x), min.min(x))),
            None => None,
        })
        .map(|(max, min)| format!("{} {}", max, min))
}

#[cfg(test)]
mod tests_get_max_and_min {
    use super::*;

    #[test]
    fn test_get_max_and_min() {
        assert_eq!(get_max_and_min("9 12 3 6 10"), Some("12 3".to_string()));
        assert_eq!(get_max_and_min("1 1 2 2 3"), Some("3 1".to_string()));
        assert_eq!(get_max_and_min("5 9 -1 10 2"), Some("10 -1".to_string()));
    }
}
