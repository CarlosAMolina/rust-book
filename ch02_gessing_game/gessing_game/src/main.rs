use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hola! Adivina el número!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret numberer is: {}", secret_number);

    loop {

        println!("Por favor, escribe un número y pulsa la tecla enter:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nDebes escribir un número.");
                continue;
            }
        };

        println!("\nHas escrito el número {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Es muy pequeño!"),
            Ordering::Greater => println!("Es muy grande!"),
            Ordering::Equal => {
                println!("Acertaste! :)");
                break;
            }
        }

    }

}
