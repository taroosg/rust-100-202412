/// 何番目？
// A〜Z の中から1文字を入力し、アルファベット順で何番目かを出力する関数を作成してください。
pub fn judge_th(c: char) -> Option<u32> {
    todo!()
}

#[cfg(test)]
mod tests_judge_th {
    use super::*;

    #[test]
    fn test_judge_th() {
        assert_eq!(judge_th('C'), Some(3));
        assert_eq!(judge_th('X'), Some(24));
    }
}
