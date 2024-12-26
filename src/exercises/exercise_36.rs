/// はじめまして
// 名前と性別から挨拶を出力する関数を作成してください。
pub fn greet(name: &str, gender: &str) -> String {
    match gender {
        "M" => format!("Hi, Mr.{}", name),
        "F" => format!("Hi, Ms.{}", name),
        _ => format!("Hi, {}", name),
    }
}

#[cfg(test)]
mod tests_greet {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Stroheim", "M"), "Hi, Mr.Stroheim");
        assert_eq!(greet("Trish", "F"), "Hi, Ms.Trish");
    }
}
