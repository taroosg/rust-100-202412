/// リストラ計画
// 現在の従業員リストから解雇する従業員を除外したリストを返す関数を作成してください。
pub fn fire_employees(employees: &str, fired: &str) -> String {
    let employees: Vec<&str> = employees.split(" ").collect();
    let fired: Vec<&str> = fired.split(" ").collect();
    employees
        .iter()
        .filter(|&e| !fired.contains(e)).copied()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests_fire_employees {
    use super::*;

    #[test]
    fn test_fire_employees() {
        assert_eq!(
            fire_employees(
                "Jotaro Joseph Avdol Polnareff Tenmei Iggy",
                "Avdol Tenmei Iggy"
            ),
            "Jotaro Joseph Polnareff"
        );
        assert_eq!(
            fire_employees(
                "Giorno Bucciarati Abbacchio Mista Narancia Fugo Trish",
                "Bucciarati Abbacchio Narancia"
            ),
            "Giorno Mista Fugo Trish"
        );
        assert_eq!(
            fire_employees(
                "Jolyne Ermes Emporio F・F Weather Anasui",
                "Jolyne Ermes F・F Weather Anasui"
            ),
            "Emporio"
        );
    }
}
