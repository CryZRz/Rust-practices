//Test -> Unit test = test unitarios

fn main() {
    //assert = asegurar
    //assert!(expresion)
    //assert_eq!(left, right)
    //assert_ne!(left, right)
}

fn codigo_solo_numeros(codigo: &str) -> bool{
    codigo.chars().all(char::is_numeric)
}

#[test]
fn check_codigo_con_numeros(){
    let result = codigo_solo_numeros("121523");
    assert!(result)
}

#[test]
fn check_codigo_con_letras(){
    let result = codigo_solo_numeros("121HOLA523");
    assert!(!result)
}

fn sumar(a: i32, b: i32) -> i32{
    a + b
}

#[test]
fn sumar_bien(){
    assert_eq!(sumar(2, 2), 4)
}