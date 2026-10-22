fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }

    for number in (1..=4).rev() {
        println!("{number}");
    }
}
fn loop_with_while_fragil() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn loop_with_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("count= {count}");
        let mut remaining = 10;

        'counting_down: loop {
            println!("remaining= {remaining}");
            if remaining == 9 {
                break 'counting_down;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}
