use std::collections::HashMap;
use std::io;

fn main() {
    println!("Welcome to the departments menu!");

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut user_input = String::new();
        //let mut user_input_v: Vec<&str> = vec![];

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");


        let user_input: String = match user_input.trim().parse() {
            Ok(user_input) => user_input,
            Err(_) => {
                println!("\nInvalid input :(");
                continue;
            }
        };

        println!("User input : {:?}", user_input);


        //user_input_v = user_input.split(' ').collect();

        if user_input.starts_with("Add") {

	    let spaces_index = blank_spaces_index(&user_input);
            println!("spaces index: {:?}", spaces_index);

            let employee = user_input[spaces_index[0] + 1..spaces_index[1]].to_string();
            let department = &user_input[spaces_index[2] + 1..user_input.len()];

            println!("employee: {:?}", employee);
            //println!("department: {:?}", department);

            //let department = user_input_v[3].to_string();
            //let employee = user_input_v[1].to_string();
            //departments.insert(employee, vec![]);
            //departments.get_mut("Engineering").unwrap().push("Sally".to_string());
            departments
                    .entry(department.to_string())
                    .and_modify(|v| v.push(employee.to_string()))
                    .or_insert(vec![employee.to_string()]);
            println!("{:?}", departments);
        }
    }

    /*

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
            .and_modify(|v| v.push(employee.to_string()))
            .or_insert(vec![employee.to_string()]);

    //    add_employee(&departments, &department, &employee);

        let mut departments_name: Vec<_> = vec![];
        for key in departments.keys() {
            departments_name.push(key);
        }
        departments_name.sort();
        for name in departments_name {
            println!("{:?}: {:?}", name, departments[name]);
        }
        */
}

fn add_employee(departments: &HashMap<&str, Vec<&str>>, department: &str, employee: &str) {
    println!("{:?}", departments);
    println!("{:?}", department);
    println!("{:?}", employee);
}

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
