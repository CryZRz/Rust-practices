fn main() {
    //clousures
    let sum2 = |num: i32| -> i32 {
        num+5
    };

    let sum3 = |num1, num2| -> i32 {
        num1+num2
    };

    let sum = sumar_uno;
    println!("{}", sum(15));
    println!("{}", sum2(15));
    println!("{}", sum3(15, 15));

    let mut counter = 1;
    let mut counter2 = 1;

    let mut incrementar = || {
        counter += 1;
    };

    let incrementar_two = move || {
        counter2 += 1;
    };

    incrementar();

    let variable = &counter; //borrowing pedir prestada
}

fn sumar_uno(num: i32) -> i32{
    num + 1
}
