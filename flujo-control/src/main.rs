//flujos de control

fn main() {

    let number = 5;
    if number == 5 {
        println!("es cinco")
    }
    else if number == 3 {
        println!("es tres")
    }
    else{
        println!("no es cinco ni tres")
    }

    let resultado = if number > 3 {1000} else {0};
    println!("resultado: {}", resultado);

    //Loops
    let mut i = 0;
    let result = loop {
        if i > 5 {
            break i;
        }
        println!("te amo");
        i += 1;
    };

    println!("{}", result);

    //While
    let mut counter = 0;
    while  counter < 10{
        println!("jimena");
        counter+=1
    }

    //for
    let arreglo = [1, 2, 3, 4, 5];
    for num in arreglo.iter() {
        println!("{}", num)
    }

    for x in 1..10 {
        println!("{}", x)
    } 

    //if let
    let edad : Option<i32> = Some(20);

    if let Some(value) = edad {
        println!("edad: {}", value)
    }

    let mut mensajes_no_lreido = Some(20);

    while let Some(value) = mensajes_no_lreido {
        if value > 0 {
            println!("Tienes mensajes no leidas");
            mensajes_no_lreido = Some(value-1);
        }else{
            println!("No hay mensajes nuevos");
            mensajes_no_lreido = None;
        }
    }

    //let-else rust 1.65
    let algun_numero: Option<i32> = Some(19);

    let Some(numero) = algun_numero else{
        //va a caer si no puede hacer match
        panic!("el numero no es valido")
    };
    println!("Numero valido: {}", numero);

    //lebeled blocks
    let algun_numero_2: Option<i32> = Some(20);
    let result_dd: i32 = 'procesamiento_a: {
        'primer_loop: loop{
            let Some(num) = algun_numero_2 else{
                break 'procesamiento_a 0;
            };

            if num == 3{
                println!("Numero valido: {}", num);
                break 'primer_loop 3;
            }else{
                println!("Numero valido: {}", num);
                break 'primer_loop num;
            }
        }

    };

    println!("el resultado es: {}", result_dd);
}
