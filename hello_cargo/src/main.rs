use std::f32::consts::{E, PI};

fn main() {
    //tipado estatico

    //variables
    let x = 8;
    let mut y: u8 = 16;
    println!("varuable: {}",y);
    y = 10;
    println!("varuable: {}",x);
    println!("varuable: {}",y);

    //shadowing
    let hello = 25;
    println!("varuable: {}", hello);
    let hello: u16 = 30;
    println!("varuable: {}", hello);

    //constantes
    const PINUM: f32 = PI;
    const EULER: f32 = E;
    println!("constante: {}", PINUM);
    println!("constante: {}", EULER);

    //tipos de datos
    let mi_numero_con_signo: i32 = -120;
    println!("mi Numero Con Signo: {}", mi_numero_con_signo);

    let mi_numero_sin_signo: u32 = 120;
    println!("mi Numero sin Signo: {}", mi_numero_sin_signo);

    //integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    //float
    let float1: f32 = 5.32;

    //boolean
    let verdadero: bool = true;
    let falso: bool = false;

    //character
    let caracter: char = 'a';
    let emoji: char = '🌵';
    println!("mi emoji: {}", emoji);

    //compuned types

    //Tuples
    let tupla = ('a', 25, -25, 10.23, '👒');
    let tupla2: (char, f32) = ('🌵', 2.35);
    let (cactus, float_tupla) = tupla2;
    println!("mi emoji2: {}", cactus);

    println!("ultimo valor de la tupla: {}", tupla.4);

    //array
    let mut arreglo: [i32; 5] = [1, 2, 3, 4, 5];
    arreglo[0] = 25;
    println!("primer valor de la tupla: {}", arreglo[0]);

    //strings
    let nombre: &str = "Christian";
    println!("mi nombre es: {}", nombre);

    //string slice
    let apellido: String = "Ramos".to_string();
    let mut domicilio = String::new();
    domicilio = "La condesa".to_string();
    println!("mi nombre es: {} {} y vivo en {}", nombre, apellido, domicilio);
}