/// 本棚選び
// 本の幅を合計したもの n(cm)、本棚の段数 d、1 段に収まる幅 e(cm) を入力し、
// 本が収まるかを OK または NG で出力する関数を作成してください。
pub fn can_store_books(s: &str) -> String {
    let v: Vec<u32> = s.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = v[0];
    let d = v[1];
    let e = v[2];
    match n / e {
        0 => "OK".to_string(),
        _ => match n / (d * e) {
            0 => "OK".to_string(),
            _ => "NG".to_string(),
        },
    }
}

#[cfg(test)]
mod tests_can_store_books {
    use super::*;

    #[test]
    fn test_can_store_books() {
        assert_eq!(can_store_books("400 5 85"), "OK");
        assert_eq!(can_store_books("500 4 70"), "NG");
    }
}
