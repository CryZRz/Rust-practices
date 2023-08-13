use std::{thread, time::Duration};
use std::sync::mpsc; //multiple producer single consumer

//Concurrency = concurrencia
//Programacion concurrente con Rust
//Paralelismo
//Chanels enviar mensajes entre threads

fn main() {
    //manejador de unir
    let nombre = String::from("Christian");

    println!("Hola {}", nombre);

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    //Hebra1
    let join_handle = thread::spawn(move || {
        //println!("{} se unio a la partida", nombre);
        //thread::sleep(Duration::from_secs(2));

        //Enviar mensaje a hebra principal
        for count in 0..3{
            let mut mensaje = String::from("Hola Jimena ");
            mensaje.push(char::from_digit(count, 10).unwrap());
            tx.send(mensaje).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

    });

    //Hebra2
    let join_handle = thread::spawn(move || {
        //println!("{} se unio a la partida", nombre);
        //thread::sleep(Duration::from_secs(2));

        //Enviar mensaje a hebra principal
        for count in ['a', 'b', 'c', 'd'].iter() {
            let mut mensaje = String::from("Hola Karla ");
            mensaje.push(*count);
            tx2.send(mensaje).unwrap();
            thread::sleep(Duration::from_secs(2));
        }

    });

   //join_handle.join().unwrap();  


    //Recibir el mensaje
    for mensaje_recibido in rx {
        println!("{}", mensaje_recibido)
    }
}
