/// 不思議なタマゴ
// タマゴをすべて孵化させるための最低歩行距離を計算する関数を作成してください。
pub fn get_number_of_steps(s: &str) -> u32 {
    s.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests_get_number_of_steps {
    use super::*;

    #[test]
    fn test_get_number_of_steps() {
        assert_eq!(get_number_of_steps("1 2 5"), 5);
        assert_eq!(get_number_of_steps("2 5 5"), 5);
        assert_eq!(get_number_of_steps("1 2 1"), 2);
    }
}
