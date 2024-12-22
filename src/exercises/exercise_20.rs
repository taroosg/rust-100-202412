/// 一週間の予定
// 半角スペース区切りで 7 日間の休み情報（yes/no）が入力され、有給申請が必要な日数を出力する関数を作成してください。
pub fn get_paid_holidays(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_get_paid_holidays {
    use super::*;

    #[test]
    fn test_get_paid_holidays() {
        assert_eq!(get_paid_holidays("yes yes yes yes no no yes"), 2);
        assert_eq!(get_paid_holidays("yes no no no no no yes"), 5);
    }
}
