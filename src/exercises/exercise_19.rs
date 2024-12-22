/// 数字の出力
// 正の整数 n（最大 2 桁）を 0 埋めして 3 桁で出力する関数を作成してください。
pub fn zero_padding3(n: u32) -> String {
    todo!()
}

#[cfg(test)]
mod tests_zero_padding3 {
    use super::*;

    #[test]
    fn test_zero_padding3() {
        assert_eq!(zero_padding3(98), "098");
        assert_eq!(zero_padding3(2), "002");
    }
}
