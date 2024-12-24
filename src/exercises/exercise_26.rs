/// 頭文字
// 苗字と名前がスペース区切りで与えられるので、各頭文字をドット区切りで出力する関数を作成してください。
pub fn get_initial_from_name(s: &str) -> String {
    s.split(" ")
        .map(|x| x.chars().next().unwrap().to_uppercase().to_string())
        .collect::<Vec<String>>()
        .join(".")
}

#[cfg(test)]
mod tests_get_initial_from_name {
    use super::*;

    #[test]
    fn test_get_initial_from_name() {
        assert_eq!(get_initial_from_name("Dio Brando"), "D.B");
        assert_eq!(get_initial_from_name("Pannacotta Fugo"), "P.F");
        assert_eq!(get_initial_from_name("Gyro Zeppeli"), "G.Z");
    }
}
