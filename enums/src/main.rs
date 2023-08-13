//Enums Enumarations
enum Countrys {
    MEXICO,
    ALEMANIA,
    GRECIA,
    ARGENTINA
}

enum WebSite {
    URL(String),
    INSTAGRAM(String),
    FACEBOOK(String),
    TWITTER(String),
}

struct User {
    name: String,
    age: i64,
    country: Countrys,
    website: WebSite
}

//option
//enum Option<T>{
//    Some(T),
//    None,
//}

fn main() {

    let country = Countrys::ARGENTINA;

    let user = User {
        name: "Jimena".to_string(),
        age: 19,
        country: Countrys::MEXICO,
        website: WebSite::INSTAGRAM(String::from("jimena"))
    };

    is_mexico(user.country);

    let nombre : Option<String> = None;

    match nombre {
        None => println!("Nombre no indicado"),
        Some(nombre) => println!("{}", nombre)
    }

    let nuevo = UserTwo {
        name: "Uriel".to_string(),
        age: Some(19)
    };

    let age_uriel = nuevo.get_age();
    match age_uriel {
        Some(age_uriel) => println!("{}", age_uriel),
        _ => (),
    }

}

fn is_mexico(country: Countrys) -> bool{
    match country {
        Countrys::MEXICO => true,
        Countrys::ALEMANIA => false,
        Countrys::GRECIA => false,
        Countrys::ARGENTINA => false,
    }
}

struct UserTwo {
    name: String,
    age: Option<i32>
}

impl UserTwo {
    fn get_age(&self) -> Option<i32>{
        self.age
    }
}