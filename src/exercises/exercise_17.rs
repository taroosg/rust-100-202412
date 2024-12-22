/// 表面積の計算
// 一辺の長さ a を入力し、立方体の表面積を出力する関数を作成してください。
pub fn get_surface_area(a: u32) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests_get_surface_area {
    use super::*;

    #[test]
    fn test_get_surface_area() {
        assert_eq!(get_surface_area(4), 96);
        assert_eq!(get_surface_area(1), 6);
    }
}
