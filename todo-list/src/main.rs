use std::{
    io::{self, Write},
    process,
};

#[derive(Debug)]
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn nueva(descripcion: String) {}
    fn completar_tarea(&mut self) {}
    fn es_completada(&self) {}
}
#[derive(Debug)]
enum Accion {
    Agregar,
    Listar,
    Completar,
    Salir,
    Eliminar,
}

fn clear_screen() {
    process::Command::new("clear").status().unwrap();
}

fn pause() {
    let mut number = String::new();
    let _ = io::stdin().read_line(&mut number);
}
fn eliminar_tarea(lista: Option<Tarea>, numero: u32) {}
fn agregar_tarea(lista: &mut [Option<Tarea>; 10], cantidad: &mut usize, descripcion: String) {
    if *cantidad >= 10 {
        return;
    }

    println!("cantidad: {}", *cantidad);

    let tarea = Tarea {
        descripcion: descripcion,
        completada: false,
    };
}
fn completar_tarea(lista: [Option<Tarea>; 10], numero: u32) {}
fn ver_lista(lista: &mut [Option<Tarea>; 10]) {}

fn menu() -> Accion {
    println!("---ToDo---");
    println!("- (A)gregar\n- (L)istar\n- (C)ompletar\n- (E)liminar\n- (S)alir");
    print!("Ingrese una opción:");
    io::stdout().flush().unwrap();
    let mut accion = String::new();
    io::stdin()
        .read_line(&mut accion)
        .expect("Error al leer linea");

    let accion: Accion = match accion.trim().to_uppercase().as_str() {
        "A" => Accion::Agregar,
        "L" => Accion::Listar,
        "C" => Accion::Completar,
        "E" => Accion::Eliminar,
        "S" => Accion::Salir,
        _ => Accion::Salir,
    };

    accion
}

fn main() {
    let mut lista: [Option<Tarea>; 10];
    let mut cantidad: usize = 0;

    loop {
        clear_screen();
        let accion: Accion = menu();

        match accion {
            Accion::Salir => break,
            Accion::Eliminar => {
                println!("Eliminar tarea");
                // eliminar_tarea(lista, numero)
            }
            Accion::Completar => {
                println!("Completar tarea");
                // completar_tarea(lista, numero)
            }
            Accion::Listar => {
                println!("Listar tareas");
                // ver_lista(&mut lista)
            }
            Accion::Agregar => {
                println!("Agregar tarea");

                let mut descripcion = String::new();
                io::stdin()
                    .read_line(&mut descripcion)
                    .expect("Error al leer descripcion");
                agregar_tarea(&mut lista, &mut cantidad, descripcion)
            }
            _ => println!("Error al inicializar acción"),
        }
    }
}
