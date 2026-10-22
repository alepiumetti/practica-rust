use rand::Rng;
use std::io;
use std::process;

fn clear_screen() {
    process::Command::new("clear").status().unwrap();
}

fn pause() {
    let mut number = String::new();
    let _ = io::stdin().read_line(&mut number);
}

#[derive(Debug)]
enum Jugada {
    Piedra,
    Papel,
    Tijera,
}

impl Jugada {
    fn gana_a(&self, otra: &Jugada) -> bool {
        match (self, otra) {
            (Jugada::Piedra, Jugada::Tijera) => true,
            (Jugada::Papel, Jugada::Piedra) => true,
            (Jugada::Tijera, Jugada::Papel) => true,
            (_, _) => false,
        }
    }
}

fn muestra_marcador(marcador: &[u32; 3]) {
    println!("--Marcador--");
    println!(
        "Jugador: {} | IA: {} | Empates: {}",
        marcador[0], marcador[1], marcador[2]
    );
}

fn leer_numero() -> u32 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Error al leer una linea");

    let number: u32 = match number.trim().parse() {
        Ok(num) => {
            if num > 0 && num < 3 {
                num
            } else {
                println!("Valor inválido");
                0
            }
        }
        Err(_) => {
            println!("Error al leer valor");
            0
        }
    };

    number
}

fn main() {
    let mut marcador: [u32; 3] = [0, 0, 0];

    loop {
        clear_screen();
        println!("Piedra | Papel | Tijera");
        println!("1 - Empezar");
        println!("2 - Salir");
        println!("Ingrese un valor:");
        let opcion = leer_numero();

        if opcion == 0 {
            continue;
        } else if opcion == 2 {
            break;
        }

        loop {
            let mut jugada = String::new();
            let jugada_ia = rand::rng().random_range(1..=100);

            let jugada_ia: Jugada = match jugada_ia {
                0..=33 => Jugada::Piedra,
                34..=66 => Jugada::Papel,
                67..=100 => Jugada::Tijera,
                _ => {
                    println!("Jugada IA es: {jugada_ia}");
                    Jugada::Tijera
                }
            };
            clear_screen();
            muestra_marcador(&marcador);
            println!("Ingrese su jugada (Piedra,Papel o Tijera)");
            println!("Puede salir escribiendo Salir");
            io::stdin()
                .read_line(&mut jugada)
                .expect("Error al leer una linea");

            let jugada: Jugada = match jugada.trim().to_uppercase().as_str() {
                "PIEDRA" => Jugada::Piedra,
                "PAPEL" => Jugada::Papel,
                "TIJERA" => Jugada::Tijera,
                "SALIR" => {
                    println!("Fin del juego");
                    return;
                }
                _ => {
                    println!("Jugada inválida, vuelva a intentarlo");
                    continue;
                }
            };

            println!("Vos: {jugada:?} vs IA: {jugada_ia:?}");

            let mut mano: (bool, bool) = (false, false);

            if jugada.gana_a(&jugada_ia) {
                mano.0 = true;
            };

            if jugada_ia.gana_a(&jugada) {
                mano.1 = true;
            };

            if !mano.0 && !mano.1 {
                println!("Empate");
                marcador[2] += 1;
            } else if mano.0 && !mano.1 {
                println!("Jugador gana la mano");
                marcador[0] += 1;
            } else {
                println!("IA gana la mano");
                marcador[1] += 1;
            };

            if marcador[0] == 3 {
                clear_screen();
                muestra_marcador(&marcador);
                println!("JUGADOR GANA LA PARTIDA");
                pause();
                break;
            } else if marcador[1] == 3 {
                clear_screen();
                muestra_marcador(&marcador);
                println!("IA GANA LA PARTIDA");
                pause();
                break;
            };

            pause();
        }
    }
}
