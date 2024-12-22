/// アットマーク
// 文字列 s の中に含まれる "at" を全て "@" に置換して出力する関数を作成してください。
pub fn convert_at_to_at_mark(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_convert_at_to_at_mark {
    use super::*;

    #[test]
    fn test_convert_at_to_at_mark() {
        assert_eq!(
            convert_at_to_at_mark("gsatcodeatquantity"),
            "gs@code@quantity"
        );
        assert_eq!(convert_at_to_at_mark("atatatjojoatatat"), "@@@jojo@@@");
    }
}
