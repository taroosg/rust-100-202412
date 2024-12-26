/// ◯◯ の秋
// アンケートから「◯◯」の部分を取り出す関数を作成してください。
pub fn get_autumn(s: &str) -> String {
    s.split("noaki").collect::<Vec<&str>>()[0].to_string()
}

#[cfg(test)]
mod tests_get_autumn {
    use super::*;

    #[test]
    fn test_get_autumn() {
        assert_eq!(get_autumn("codenoaki"), "code");
        assert_eq!(get_autumn("dokusyo"), "dokusyo");
        assert_eq!(get_autumn("javascriptnoaki"), "javascript");
    }
}
