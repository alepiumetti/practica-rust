const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;

    println!("The value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing con mutación de valores

    let spaces = "    ";
    println!("spaces value: {spaces}");
    let spaces = spaces.len();
    println!("spaces value: {spaces}");

    // Código no funcional
    // let mut mutable_spaces = "    ";
    // println!("spaces value: {mutable_spaces}");
    // mutable_spaces = mutable_spaces.len();
    // println!("spaces value: {mutable_spaces}");
}
