fn get_num(num: i32) -> i32{
   return num; 
}

fn seleccion_numero(num: i32){
    println!("numero es:, {}", num);
}

fn num_refer(num: &mut i32){
    *num = *num*2;
}

fn main() {
    let mut num_new = 2;
    println!("{}", num_new);
    num_refer(&mut num_new);
    println!("{}", num_new);
    let num = get_num(25);
    println!("Hello, world!, {}", num);
    seleccion_numero(20)
}