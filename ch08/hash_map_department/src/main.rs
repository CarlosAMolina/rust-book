use std::collections::HashMap;

fn main() {
    let mut departments = HashMap::new();

    departments.insert("Engineering", vec![]);
    departments.insert("Sales", vec![]);

    departments.get_mut("Engineering").unwrap().push("Sally");
    departments.get_mut("Sales").unwrap().push("Doe");
    departments.get_mut("Sales").unwrap().push("Amir");
    departments.get_mut("Sales").unwrap().sort();

    let department = "Games";
    let employee = "Carlos";
    departments
        .entry(department)
        .and_modify(|v| v.push(employee))
        .or_insert(vec![employee]);

    add_employee(&departments, &department, &employee);

    let mut departments_name: Vec<_> = vec![];
    for key in departments.keys() {
        departments_name.push(key);
    }
    departments_name.sort();
    for name in departments_name {
        println!("{:?}: {:?}", name, departments[name]);
    }
}

fn add_employee(departments: &HashMap<&str, Vec<&str>>, department: &str, employee: &str) {
    println!("{:?}", departments);
    println!("{:?}", department);
    println!("{:?}", employee);
}
