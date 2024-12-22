/// どれにしようかな
// メニューの数 n を入力し、「どれにしようかな」の文字数 21 を用いて選ばれたメニューを出力する関数を作成してください。
pub fn choice_of_god(n: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_choice_of_god {
    use super::*;

    #[test]
    fn test_choice_of_god() {
        assert_eq!(choice_of_god(4), 1);
        assert_eq!(choice_of_god(3), 3);
    }
}
