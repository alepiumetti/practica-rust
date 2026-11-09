use std::{
    io::{self, Write},
    process,
};

#[derive(Debug, Default)]
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn nueva(&self, descripcion: String) {
        // self.descripcion = descripcion;
        // self.completada = false;
    }
    fn completar_tarea(&mut self) {
        self.completada = true;
    }
    fn es_completada(&self) -> bool {
        self.completada
    }
}
#[derive(Debug)]
enum Accion {
    Agregar,
    Listar,
    Completar,
    Salir,
    Eliminar,
}
// Función para borrar la pantalla
fn clear_screen() {
    process::Command::new("clear").status().unwrap();
}

// Pausa en espera de que se aprete un boton
fn pause() {
    print!("Aprete enter para continuar...");
    io::stdout().flush().unwrap();
    let mut number = String::new();
    let _ = io::stdin().read_line(&mut number);
}
// fn eliminar_tarea(lista: [Tarea], numero: u32) {}
fn agregar_tarea(lista: &mut [Tarea], cantidad: &mut usize, descripcion: String) {
    if *cantidad >= 10 {
        println!("Límite de tareas alcanzado.");
        pause();
        return;
    }
    println!("cantidad: {}", *cantidad);

    lista[*cantidad].descripcion = descripcion;
    // tarea.descripcion = descripcion;
    // tarea.completada = false;

    // &lista[*cantidad] = &mut tarea;
    *cantidad += 1;

    pause();
}
// fn completar_tarea(lista: &[Option<&mut Tarea>; 10], numero: usize) {
//     let tarea: &Option<&mut Tarea> = &lista[numero];
//     match tarea {
//         Some(x) => **x.completar_tarea(),
//         None => return, // None => println!("No es posible completar una tarea que no existe"),
//     };
// }

// Función para ver la lista de las tareas
// Recibe prestada la lista de tareas
fn ver_lista(lista: &[Tarea; 10]) {
    if lista.is_empty() {
        println!("La lista está vacía");
        pause();
        return;
    }

    for (index, item) in lista.iter().enumerate() {
        if item.es_completada() {
            print!("{} - ✅{}", index, item.descripcion);
        } else {
            if item.descripcion.is_empty() {
                println!("{} - ❎{}", index, item.descripcion);
            } else {
                print!("{} - ❎{}", index, item.descripcion);
            }
        };
        io::stdout().flush().unwrap();
    }
    pause();
}

// Función de menú
fn menu() -> Option<Accion> {
    println!("---ToDo---");
    println!("- (A)gregar\n- (L)istar\n- (C)ompletar\n- (E)liminar\n- (S)alir");
    print!("Ingrese una opción:");
    io::stdout().flush().unwrap();
    let mut accion = String::new();
    io::stdin()
        .read_line(&mut accion)
        .expect("Error al leer linea");

    let accion: Option<Accion> = match accion.trim().to_uppercase().as_str() {
        "A" => Some(Accion::Agregar),
        "L" => Some(Accion::Listar),
        "C" => Some(Accion::Completar),
        "E" => Some(Accion::Eliminar),
        "S" => Some(Accion::Salir),
        _ => None,
    };

    accion
}

fn main() {
    // Array con todas las tareas
    let mut lista: [Tarea; 10] = [
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
        Tarea {
            descripcion: String::new(),
            completada: false,
        },
    ];
    // cantidad de tareas agregadas
    let mut cantidad: usize = 0;

    loop {
        clear_screen(); // Helper de limpieza de pantalla
        let accion: Option<Accion> = menu(); // Abre el menú y en guarda la acción
        let accion: Accion = match accion {
            //Si la acción no es válida vuelve a empezar el loop
            Some(x) => x,
            None => {
                println!("Error al iniciarlizar acción");
                continue;
            }
        };
        match accion {
            // en base a la acción se ejecutan los métodos
            Accion::Salir => break,
            Accion::Eliminar => {
                println!("Eliminar tarea");
                // eliminar_tarea(lista, numero)
            }
            Accion::Completar => {
                println!("Completar tarea");
                // let tarea = &mut lista[0];
                // tarea.completar_tarea();

                // completar_tarea(&lista, 0);
            }
            Accion::Listar => {
                println!("Listar tareas");
                ver_lista(&lista)
            }
            Accion::Agregar => {
                println!("Agregar tarea");

                print!("Ingrese una descripción de la tarea:");
                io::stdout().flush().unwrap();
                let mut descripcion = String::new();
                io::stdin()
                    .read_line(&mut descripcion)
                    .expect("Error al leer descripcion");
                agregar_tarea(&mut lista, &mut cantidad, descripcion)
            }
        }
    }
}
