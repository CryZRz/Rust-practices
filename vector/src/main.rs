use std::collections::{HashSet, HashMap};

fn main() {
    //Collections

    //Vector
    //String
    //hasMap

    //Vector
    let mut vec: Vec<i32> = Vec::new();
    let mut vec_two = vec![1, 2, 3];

    vec.push(9);
    vec.push(10);
    vec.push(3);

    let tercer = vec[2];
    println!("{}", tercer);

    let segundo = vec.get(1);
    match segundo {
        Some(value) => println!("{}", value),
        None => panic!("number is invalid")
    }
    
    for i in &vec{
        println!("{}", i);
    }

    for x in &mut vec_two{
        *x *= 2
    }

    for i in &vec_two{
        println!("{}", i);
    }

    enum Mensajes {
        MENSAJE(String),
        ERROR(i32)
    }

    let mensajes = vec![Mensajes::MENSAJE("hola".to_string()), Mensajes::ERROR(404)];

    for m in &mensajes {
        match m {
            Mensajes::MENSAJE(texto) => println!("{}", texto),
            Mensajes::ERROR(err) => println!("{}", err),
        }
    }

    //Strings
    //String vs stringSlice
    //slice: referencia a una contigua secuencia de elemento de un collection
    
    //String se guardar en el heap
    //string slide: stack, referencia al heap, string literal incrustado en el codigo binario

    //Hashset
    let mut user_ids = HashSet::new();
    user_ids.insert(&12);
    user_ids.insert(&125);
    user_ids.insert(&3);
    user_ids.insert(&500);
    user_ids.insert(&500);
    user_ids.remove(&125);

    for id in user_ids.iter() {
        println!("{}", id);
    }

    let mut backup_users = HashSet::new();
    backup_users.insert(&100);
    backup_users.insert(&23);
    backup_users.insert(&9);

    for d in user_ids.difference(&backup_users){
        println!("{}", d)
    }

    let mut puntajes: HashMap<String, i32> = HashMap::new();
    puntajes.insert("Azul".to_string(), 20);
    puntajes.insert("Rojo".to_string(), 30);

    let puntos_azul = puntajes.get("Rojo");

    for (key, value) in &puntajes {
        println!("{}, {}", key, value)
    }
}