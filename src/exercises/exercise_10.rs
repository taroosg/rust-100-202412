/// 等差数列
// 初項 m と公差 n を入力し、10番目までの数字をスペース区切りで出力する関数を作成してください。
pub fn create_sequence(m: i32, n: i32) -> String {
    [m; 10]
        .iter()
        .enumerate()
        .map(|(i, x)| (i as i32 * n + x).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests_create_sequence {
    use super::*;

    #[test]
    fn test_create_sequence() {
        assert_eq!(create_sequence(3, 3), "3 6 9 12 15 18 21 24 27 30");
        assert_eq!(create_sequence(5, 10), "5 15 25 35 45 55 65 75 85 95");
    }
}
