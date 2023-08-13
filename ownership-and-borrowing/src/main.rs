//Ownership = propiedad, que es dueño de
//Borrowing = pedir prestado

//Rust no posee garbage collector
//Cada data tiene 1 un dueño (owner)

//Stack vs Heap

//Stack
/*
    Implementada como stack (estructura pila)
    rapida, basta con mover el puntero
    tamaño fijo
    es liberada cuando se alcanza el fin del scope
*/

//Heap
/*
    Flexible
    Costoso de asignar y recuperar datos
    Es liberada cuando no tiene dueños (owners)
*/

use std::f32::consts::PI;

const PI_VALUE: f32 = PI;

fn main() {

   //let name = String::from("Chris");
   //let name_copy = name;
   //println!("{}", name);

   let mut edad = 20;
    aumentar_edad(&mut edad);
    println!("{}", edad);

    let mut nombre = String::from("Christian");
    enviar_nombre(&mut nombre);
    println!("{}", nombre);

    //LifeTimes = tiempo de vide de las referencias
    /*
        Lifestimes son una forma de asegurar que un pedazo 
        de memoria es aun valida para una referencia 
    */

    let a: &i32;
    {
        let b = 10;
        a = &b; //ownership
    };
    println!("{}", a);
}

//fn hace_algo<'a>(param: &'a i32) -> &'a i32{
//    param
//}

fn dame_ref() -> &i32{
    let a = 1;
    &a
}

fn hace_algo(param: &i32) -> &i32{
    param
}

fn enviar_nombre(nombre2: &mut String){
    nombre2.push_str(" Ramos");
    println!("eviando.... {}", nombre2);
}

fn aumentar_edad( edad_copy: &mut i32) {
    *edad_copy += 1;
}