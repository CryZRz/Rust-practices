//Generics
struct Point<T>{
    x: T,
    y: T
}

fn main() {
    let pointA = Point {
        x: 12,
        y: 15
    };

    let pointB = Point {
        x: 12.32,
        y: 15.25
    };

    calcular_area(pointA, pointB);
}

fn calcular_area<T, V>(pointA: Point<T>, pointB: Point<V>){

}