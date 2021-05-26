use std::collections::HashMap;

fn main() {
    let mut departments = HashMap::new();
    departments.insert("Engineering", "Sally");
    departments.insert("Sales", "Amir");
    println!("{:?}", departments);

    let mut result_expected = HashMap::new();
    result_expected.insert("Engineering", "Sally");
    result_expected.insert("Sales", "Amir");
    assert_eq!(result_expected, departments);
}

