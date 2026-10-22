fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is {guess}");

    let x = 2.0; //f64
    let y: f32 = 3.0;

    println!("The value of x is {x} and the value of y is {y}");

    // resto de una división

    let remainder = 21 % 4;

    println!("Remainder value is: {remainder}");

    // Booleanos
    let t = true; // implicito
    let f: bool = false; //Explicito

    println!("True is {t} and false is {f}");

    // Caracteres de 4 bytes en unicode

    let c = 'z';
    let z: char = 'Z'; //Explicit type annotation
    let heart_eyed_cat = '😻';

    println!("This three values are char: c = {c} | z = {z} | heart_eyed_cat = {heart_eyed_cat}");

    // Compound types
    //
    // Tuplas

    let tup: (i32, f64, u8) = (500, 6.4, 1); // Puede tener diferentes types

    let (x, y, z) = tup; // pattern matching para tomar los valores de una Tupla con una
    // destructuración
    println!("Value of x={x}, y={y} and z={z}");

    // otra forma de acceder a las tuplas
    println!("The first value of tup is: {}", tup.0);

    // arrays
    // A diferencia de las tuplas solo puede tener un type
    let arr: [i32; 6] = [1, 2, 3, 4, 5, 6]; // tiene tamaño fijo  Guarda los datos en el stack
    //  [type; lenght] = inicialización

    let arrInicializado = [3; 5]; // array de 5 elementos que valen 3 [3,3,3,3,3]
    let arrFirst = arr[0];
    let arrInicializadoSecond = arrInicializado[1];

    println!("First: {arrFirst}, Second: {arrInicializadoSecond}");
}
