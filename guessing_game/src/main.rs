use std::cmp::Ordering;
use std::io;

use rand::Rng; // import the random number generator

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin() // ask for input
            .read_line(&mut guess)
            .expect("Failed to read line"); //Check if the input is the expected form

        let guess: u32 = match guess.trim().parse() {
            // calbacks
            Ok(num) => num,     // Si el resultado es Ok, retorna num
            Err(_) => continue, //Si el resultado es Err con lo que haya como parámetro, continua
        }; //shadow value 

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
