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
    // fn nueva(&self, descripcion: String) {
    //     // self.descripcion = descripcion;
    //     // self.completada = false;
    // }
    fn completar_tarea(&mut self) {
        self.completada = true;
    }
    fn es_completada(&self) -> bool {
        self.completada
    }
}

enum VistaTareas {
    Completadas,
    Todas,
    Incompletas,
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
    lista[*cantidad].descripcion = descripcion;
    *cantidad += 1;

    pause();
}
fn completar_tarea(lista: &mut [Tarea; 10]) {
    loop {
        clear_screen();
        ver_lista(lista, VistaTareas::Incompletas);
        io::stdout().flush().unwrap();
        print!("Ingrese la tarea que quiere completar:");
        let mut tarea = String::new();
        io::stdin()
            .read_line(&mut tarea)
            .expect("Error al leer linea");

        let tarea: usize = match tarea.trim().parse() {
            Ok(i) => i,
            Err(..) => {
                println!("Debe ingresar un número entero");
                continue;
            }
        };
        if !(0..10).contains(&tarea) {
            println!("Debe ingresar un número entre 0 y 9 que son las tareas disponibles");
            continue;
        };

        if lista[tarea].descripcion.is_empty() {
            println!("Seleccione una tarea inicializada");
            continue;
        }

        if lista[tarea].es_completada() {
            println!("La tarea ya está completa");
            continue;
        } else {
            let _ = &lista[tarea].completar_tarea();
            println!("Tarea {tarea} completada");
            pause();
            break;
        }
    }
}

fn eliminar_tarea(lista: &mut [Tarea; 10]) {
    loop {
        clear_screen();
        ver_lista(lista, VistaTareas::Todas);
        io::stdout().flush().unwrap();
        print!("Ingrese la tarea que quiere eliminar:");
        let mut tarea = String::new();
        io::stdin()
            .read_line(&mut tarea)
            .expect("Error al leer linea");

        let tarea: usize = match tarea.trim().parse() {
            Ok(i) => i,
            Err(..) => {
                println!("Debe ingresar un número entero");
                pause();
                continue;
            }
        };
        if !(0..10).contains(&tarea) {
            println!("Debe ingresar un número entre 0 y 9 que son las tareas disponibles");
            pause();
            continue;
        };

        if lista[tarea].descripcion.is_empty() {
            println!("Debe ingresar una tarea que esté ingresada");
            pause();
            continue;
        } else {
            lista[tarea].descripcion = String::new();
            lista[tarea].completada = false;
            println!("Tarea {tarea} eliminada");
            pause();
            break;
        }
    }
}

// Función para ver la lista de las tareas
// Recibe prestada la lista de tareas
fn ver_lista(lista: &[Tarea; 10], tipo_vista: VistaTareas) {
    if lista.is_empty() {
        println!("La lista está vacía");
        pause();
        return;
    }

    match tipo_vista {
        VistaTareas::Completadas => {
            for (index, item) in lista.iter().enumerate() {
                if item.es_completada() {
                    print!("{} - ✅{}", index, item.descripcion);
                }
                io::stdout().flush().unwrap();
            }
        }
        VistaTareas::Incompletas => {
            for (index, item) in lista.iter().enumerate() {
                if !item.es_completada() {
                    if item.descripcion.is_empty() {
                        println!("{} - ❎{}", index, item.descripcion);
                    } else {
                        print!("{} - ❎{}", index, item.descripcion);
                    }
                };
                io::stdout().flush().unwrap();
            }
        }
        VistaTareas::Todas => {
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
        }
    }
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
            Accion::Eliminar => eliminar_tarea(&mut lista),
            Accion::Completar => {
                completar_tarea(&mut lista);
            }
            Accion::Listar => {
                ver_lista(&lista, VistaTareas::Todas);
                pause();
            }
            Accion::Agregar => {
                clear_screen();
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
