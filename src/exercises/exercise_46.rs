/// ワインのキャッチコピー
// 入力文字列を "Best in " に結合して出力する関数を作成してください。
pub fn best_copy(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_best_copy {
    use super::*;

    #[test]
    fn test_best_copy() {
        assert_eq!(best_copy("a decade"), "Best in a decade");
        assert_eq!(best_copy("the world"), "Best in the world");
        assert_eq!(best_copy("history ever"), "Best in history ever");
    }
}
