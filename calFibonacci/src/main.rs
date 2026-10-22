use std::io;

fn main() {
    loop {
        println!("Input the index for calculate the Fibbonacci number:");
        println!("Press 0 to exit");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i64 = match number.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error in parse: {error}");
                break; //unica forma de salir del programa
            }
        };

        if number < 0 {
            println!("You can only input positive numbers");
            println!("-----------------------");
            continue;
        }

        if number == 0 {
            println!("The number is: {number}");
            break;
        }

        let number: i64 = cal_fibonacci(number);

        println!("The number is: {number}");
        println!("-----------------------")
    }
}

fn cal_fibonacci(number: i64) -> i64 {
    let number = match number {
        0 => 0,
        1 => 1,
        _ => {
            let mut n = [0, 1];
            let mut fibonacci = 1;

            let mut index = 1;
            while index < number {
                index += 1;
                fibonacci = n[0] + n[1];
                n[0] = n[1];
                n[1] = fibonacci;
            }
            fibonacci
        }
    };
    number
}
