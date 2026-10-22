use std::io;

enum Operacion {
    Suma(f64, f64),
    Resta(f64, f64),
    Multiplicacion(f64, f64),
    Division(f64, f64),
}

impl Operacion {
    fn calcular(self: &Self) -> f64 {
        match *self {
            Operacion::Suma(a, b) => a + b,
            Operacion::Resta(a, b) => a - b,
            Operacion::Division(a, b) => a / b,
            Operacion::Multiplicacion(a, b) => a * b,
        }
    }
}

fn leer_numero() -> String {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Error al leer la linea");

    number
}

fn main() {
    loop {
        println!("1-Suma\n2-Resta\n3-Multiplicación\n4-División\n5-Salir");

        println!("Ingrese una opción:");
        let opcion = leer_numero();

        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => {
                if num > 0 && num < 6 {
                    num
                } else {
                    println!("Opcion inválida");
                    continue;
                }
            }
            Err(_) => continue,
        };

        if opcion == 5 {
            println!("Programa terminado");
            break;
        }

        println!("Ingrese el primer operador:");
        let x = leer_numero();
        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ingrese el segundo operador:");
        let y = leer_numero();

        let y: f64 = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let opcion: Operacion = match opcion {
            1 => Operacion::Suma(x, y),
            2 => Operacion::Resta(x, y),
            3 => Operacion::Multiplicacion(x, y),
            4 => {
                if y < 1.0 {
                    println!("Error al dividir por: {}", y);
                    continue;
                };
                Operacion::Division(x, y)
            }
            _ => continue,
        };

        println!("El resultado es: {}", opcion.calcular());
        continue;
    }
}
