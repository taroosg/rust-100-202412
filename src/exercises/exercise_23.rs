/// サイコロの裏面
// 一般的な 6 面サイコロの目を表す整数 n に対し、その裏側にある目を出力する関数を作成してください。
pub fn get_back_side(n: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_get_back_side {
    use super::*;

    #[test]
    fn test_get_back_side() {
        assert_eq!(get_back_side(1), 6);
        assert_eq!(get_back_side(3), 4);
    }
}
