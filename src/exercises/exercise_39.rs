/// 台風の間隔
// 台風の上陸日から間隔日数を計算する関数を作成してください。
pub fn get_interval(s: &str) -> String {
    s.split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .enumerate()
        .map(|(i, x)| match i {
            0 => x,
            _ => {
                x - s.split_whitespace().collect::<Vec<&str>>()[i - 1]
                    .parse::<u32>()
                    .unwrap()
            }
        })
        .enumerate()
        .filter(|&(i, _)| i != 0)
        .map(|x| format!("{}", x.1))
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests_get_interval {
    use super::*;

    #[test]
    fn test_get_interval() {
        assert_eq!(get_interval("5 8 19 25 31"), "3 11 6 6");
        assert_eq!(get_interval("2 3 7 9 28"), "1 4 2 19");
    }
}
