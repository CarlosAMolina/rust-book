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
                for departmentname in sort_hash_map_keys(&departments){
                    println!(
                        "{:?}: {:?}",
                        departmentname,
                        sort_hash_map_values_by_key(&departments, &departmentname)
                    );
                }
            } else {
                println!(
                    "{:?}: {:?}",
                    what_to_show,
                    sort_hash_map_values_by_key(&departments, &what_to_show.to_string())
                );
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

fn sort_hash_map_values_by_key(
    hash_map: &HashMap<String, Vec<String>>,
    key: &String,
) -> Vec<String> {
    let mut values: Vec<String> = vec![];
    for v in &hash_map[key] {
        values.push(v.to_string());
    }
    values.sort();
    values
}

fn sort_hash_map_keys(hash_map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut keys: Vec<String> = vec![];
    for key in hash_map.keys() {
        keys.push(key.to_string());
    }
    keys.sort();
    keys
}

fn print_invalid_input() {
    println!("Invalid input :(");
}
