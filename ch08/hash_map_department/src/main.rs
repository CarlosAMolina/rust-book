use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the departments menu!");
    println!("  Usage example:");
    println!("  - Add Carlos to Dev");
    println!("  - Show people in Dev");
    println!("  - Show people in company");

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: String = match user_input.trim().parse() {
            Ok(user_input) => user_input,
            Err(_) => {
                print_invalid_input();
                continue;
            }
        };

        println!("User input: {:?}", user_input);

        if user_input.starts_with("Add") {
            let spaces_index = blank_spaces_index(&user_input);
            println!("Spaces index: {:?}", spaces_index);

            let employee = user_input[spaces_index[0] + 1..spaces_index[1]].to_string();
            let department = &user_input[spaces_index[2] + 1..user_input.len()];

            println!("employee / department: {:?} / {:?}", employee, department);

            departments
                .entry(department.to_string())
                .and_modify(|v| v.push(employee.to_string()))
                .or_insert(vec![employee.to_string()]);
            println!("{:?}", departments);
        } else if user_input.starts_with("Show") {
            let spaces_index = blank_spaces_index(&user_input);
            let what_to_show = &user_input[spaces_index[2] + 1..user_input.len()];
            println!("{:?}", what_to_show);
            // TODO sort names
            if what_to_show == "company" {
                let mut departments_name: Vec<_> = vec![];
                for key in departments.keys() {
                    departments_name.push(key);
                }
                departments_name.sort();
                for name in departments_name {
                    println!("{:?}: {:?}", name, departments[name]);
                }
            } else {
                println!("{:?}: {:?}", what_to_show, departments[what_to_show]);
            }
        } else {
            print_invalid_input();
        }
    }
}

//fn add_employee(departments: &HashMap<&str, Vec<&str>>, department: &str, employee: &str) {
//    println!("{:?}", departments);
//    println!("{:?}", department);
//    println!("{:?}", employee);
//}

fn blank_spaces_index(s: &str) -> Vec<usize> {
    let bytes = s.as_bytes();
    let mut indexes = vec![];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            indexes.push(i);
        }
    }
    indexes
}

fn print_invalid_input() {
    println!("Invalid input :(");
}
