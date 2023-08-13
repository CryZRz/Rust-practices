//Struts y Enums
struct Person {
    nombre: String,
    email: String,
    age: i32,
    active: bool,
}

impl Person {
    fn edad(&self) -> i32{
        self.age
    }
}

fn main() {
    let mut person = Person{
        nombre: "Christian".to_string(),
        email: String::from("Chris@mail.com"),
        age: 25,
        active: true
    };
    
    println!("nombre {} email: {} edad: {} activo: {}", person.nombre, person.email, person.age, person.active);

    person.nombre = "Jimena".to_string();

    let person2 = nuevo_usuario(String::from("Jimena"), String::from("jimena@gmail.com"));

    let person3 = Person{
        nombre: String::from("Jimena"),
        email: String::from("jimena@gmail.com"),
        ..person
    };
    println!("nombre {} email: {} edad: {} activo: {}", person3.nombre, person3.email, person3.age, person3.active);

    //tuple structs
    struct Point(i32, i32, i32);
    let pointA = Point(12, 15, 16);

    let get_edad = person3.edad();
    println!("la edad es: {}", get_edad);
}

fn nuevo_usuario(nombre: String, email: String) -> Person{
    return Person {
        nombre,
        email,
        age: 15,
        active: true
    };
}