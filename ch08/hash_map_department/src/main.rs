use std::collections::HashMap;

fn main() {
    let mut departments = HashMap::new();
    departments.insert("Engineering", "Sally");
    departments.insert("Sales", "Amir");
    departments.insert("Delete", "Foo");
    println!("{:?}", departments);
    departments.remove("Delete");

    let mut result_expected = HashMap::new();
    result_expected.insert("Engineering", "Sally");
    result_expected.insert("Sales", "Amir");
    assert_eq!(result_expected, departments);

    let mut v: Vec<String> = vec!["a".to_string(),
                                  "d".to_string(),
                                  "e".to_string(),
                                  "d".to_string(),
                                  "b".to_string()];
    v.push("a".to_string());
    println!("{:?}", v);
    let mut departments_b = HashMap::new();
    departments_b.insert("vector".to_string(), v);
    println!("{:?}", departments_b["vector"]);
    //TODO departments_b["vector"].push("x".to_string());

}

