/// 日付のデータ
// 西暦 y、月 m、日 d をスペース区切りで入力し、/ 区切りでフォーマットした文字列を出力する関数を作成してください。
pub fn format_ymd(s: &str) -> String {
    s.split(' ')
        // 4桁/2桁/2桁にする
        .enumerate()
        .map(|(i, x)| {
            let num: u32 = x.parse().unwrap();
            if i == 0 {
                format!("{:04}", num)
            } else {
                format!("{:02}", num)
            }
        })
        .collect::<Vec<String>>()
        .join("/")
}

#[cfg(test)]
mod tests_format_ymd {
    use super::*;

    #[test]
    fn test_format_ymd() {
        assert_eq!(format_ymd("2021 1 1"), "2021/01/01");
        assert_eq!(format_ymd("2020 10 10"), "2020/10/10");
        assert_eq!(format_ymd("794 11 29"), "0794/11/29");
    }
}
