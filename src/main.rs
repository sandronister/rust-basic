

fn main() {

 

    let result = divide(10 as f64, 0 as f64);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero")
    }
   
}


fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None
    }

    Some(a / b)
}

