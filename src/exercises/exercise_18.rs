/// 三角形の内角の和
// 平面上の三角形の 2 つの角度 a, b を入力し、残りの 1 つの角の角度を出力する関数を作成してください。
pub fn get_angle(a: u32, b: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_get_angle {
    use super::*;

    #[test]
    fn test_get_angle() {
        assert_eq!(get_angle(30, 90), 60);
        assert_eq!(get_angle(45, 45), 90);
    }
}
