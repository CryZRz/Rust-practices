//Mutexes: permiten el acceso al mismo dato desde distintas hebras por turnos

use std::{sync::{Mutex, Arc}, thread, time::Duration};

fn main() {
    //Mutex("hola")
    //lock = bloqueador
    //"lock("hola")
    //liberar lock
    //thread safe

    //Arc similar a Rc, pero es thread safe, es seguro de usarlo en situciones concurrentes
    //Arc: Atomic Reference counted (smart pointer)
    //atomic: son primitivos seguros de compartir en disitintas hebras

    let id = Arc::new(Mutex::new(99));
    let mut handles = vec![];
    
    for _ in 0..3 {
        let num_clone = Arc::clone(&id);

        let handlel1 = thread::spawn(move || {
            let mut numero = num_clone.lock().unwrap();
            *numero += 1;
        });

        handles.push(handlel1);
    }

    for hanlde in handles{
        hanlde.join().unwrap()
    }


    println!("{:?}", id)
}
