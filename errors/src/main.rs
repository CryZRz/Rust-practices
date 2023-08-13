use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //Manejo de errores
    //Recuperables: abrir un archivo donde el path es incorrecto
    //Result <T, E>

    //No-recuperables: tratar de acceder a un arreglo mas alla de su limite
    //panic!
    
    let file = File::open("/hola/hola.txt");

    match file {
        Ok(archivo) => leer_archivo(archivo),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("archivo no encontrado"),
            other_error => println!("Error desconido {}", other_error)
        }
    }

    let file2 = File::open("/hola/hola.txt").expect("No se encontro el archivo");

}

fn leer_archivo(file: File){

}