/// 数の比較
// 整数の入力値 a と b を比較し、大きい方の値を出力する関数を作成してください。
// a と b が等しい場合は "eq" を出力してください。
pub fn compare_numbers(a: i32, b: i32) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compare_numbers() {
        assert_eq!(compare_numbers(2, 5), "5");
        assert_eq!(compare_numbers(10, 9), "10");
        assert_eq!(compare_numbers(2, 2), "eq");
    }
}
