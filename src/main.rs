

fn main() {
   let name = String::from("John Doe");

   let copy = &name;

    assert_eq!("John Doe", name);
    assert_eq!("John Doe", *copy);

    println!("Name: {}", name);
}

