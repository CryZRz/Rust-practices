//Traits = rasgo
struct Humano;
struct Gato;

//Derive
#[derive(Debug)]
struct User {
    name: String,
    age: i64
}

#[derive(Default, Debug)]
struct UserTwo {
    name: String,
    age: i64,
    role: UserRole
}

#[derive(Default, Debug)]
enum UserRole{
    #[default]
    BASIC,
    ADMIN
}

trait Hablar {
    fn di_hola(&self) -> String;
    fn idioma() -> String {
        "No tengo idioma".to_string()
    }
}

impl Hablar for Humano {
    fn di_hola(&self) -> String {
        "Hola :)".to_string()
    }
    fn idioma() -> String {
        "Hablo humano".to_string()
    }
}

impl Hablar for Gato {
    fn di_hola(&self) -> String {
        "MIAU".to_string()
    }
    fn idioma() -> String {
        "Hanlo gatuno".to_string()
    }
}

//impl std::fmt::Debug for User{
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        f.debug_struct("User").field("name", &self.name).field("age", &self.age).finish();
//        write!(f, "Usuario {}, tine {} años",  self.name, self.age)
//    }
//}

fn main() {
    let christian = Humano;
    println!("{}", christian.di_hola());
    let pelusa = Gato;
    println!("{}", pelusa.di_hola());
    println!("{}", Humano::idioma());

    let edad: Option<i32> = Some(15);
    if edad.es_mayor_de_edad(){
        println!("es mayor de edad")
    }else{
        println!("edad no identificada")
    }

    let user = User {
        name: "Chris".to_string(),
        age: 25
    };

    println!("{:?}", user);

    let user2 = UserTwo::default();
    println!("{:?}", user2)
}

trait LicenciaConducir {
    fn es_mayor_de_edad(&self) -> bool;
}

impl LicenciaConducir for Option<i32>{
    fn es_mayor_de_edad(&self) -> bool {
        match self{
            Some(edad) => *edad > 18,
            None => false
        }
    }
}
