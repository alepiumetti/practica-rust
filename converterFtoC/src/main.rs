use std::io;

fn main() {
    println!("Converter Fahrenheit to Celcius!");

    loop {
        let mut temperature = String::new();

        println!("Input the value in Fahrenheit to convert: ");

        // Ingreso un valor
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        // parseo el valor string a signed 32
        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(error) => {
                println!("Error in parse: {error}");
                continue;
            }
        }; // shadow value

        println!("The value in Farenheit is: {temperature}");

        let temperature = ((temperature as f32) - 32.0) * 5.0 / 9.0;

        println!("The value in Celcius is: {temperature}");
    }
}
