enum Fruta {
    Banana,
    Laranja,
    Morango,
    Maca,
    Acai
}

enum Coordenadas{
    Ponto2D(i32, i32),
    Ponto3D(i32, i32, i32)
}

struct Person {
    name: String,
    age: u8,
    height: f32
}

impl Person {
    fn new(name: String, age: u8, height: f32) -> Person {
        Person {
            name,
            age,
            height
        }
    }
}

trait Area {
    fn area(&self) -> u32;
}

trait Factory {
    fn new(width: u32, height: u32) -> Self;
}

struct Retangulo {
    width: u32,
    height: u32
}

impl Area for Retangulo {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Factory for Retangulo {
    fn new(width: u32, height: u32) -> Self {
        Retangulo {
            width,
            height
        }
    }
}

impl Retangulo {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    fruits(Fruta::Banana);
    let ponto2d = Coordenadas::Ponto2D(1, 2);
    let ponto3d = Coordenadas::Ponto3D(1, 2, 3);
    let person = Person::new("John".to_string(), 25, 1.75);
    let retangulo = Retangulo::new(10, 10);

    coords(ponto2d);
    coords(ponto3d);
    print_person(person);
    println!("Area: {}", retangulo.area());
    println!("Is square: {}", retangulo.is_square());
}

fn fruits(fruta:Fruta) {
    match fruta {
        Fruta::Banana => println!("Banana"),
        Fruta::Laranja => println!("Laranja"),
        Fruta::Morango => println!("Morango"),
        Fruta::Maca => println!("Maca"),
        Fruta::Acai => println!("Acai")
    }
}

fn coords(coordenadas:Coordenadas) {
    match coordenadas {
        Coordenadas::Ponto2D(x, y) => println!("Ponto 2D: {}, {}", x, y),
        Coordenadas::Ponto3D(x, y, z) => println!("Ponto 3D: {}, {}, {}", x, y, z)
    }
}

fn print_person(person: Person) {
    println!("Name: {}, Age: {}, Height: {}", person.name, person.age, person.height);
}
