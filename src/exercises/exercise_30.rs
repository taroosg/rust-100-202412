/// 花粉症でつらい
// ティッシュ 1 箱が空になるまでの期間 m 日、花粉症の季節の残り日数 n 日から必要なティッシュ箱の数を求める関数を作成してください。
pub fn get_need_box(m: u32, n: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_get_need_box {
    use super::*;

    #[test]
    fn test_get_need_box() {
        assert_eq!(get_need_box(7, 30), 5);
        assert_eq!(get_need_box(3, 100), 34);
        assert_eq!(get_need_box(7, 5), 1);
    }
}
