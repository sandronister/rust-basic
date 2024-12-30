

fn main() {

    let resultado = std::panic::catch_unwind(|| {
        let values =function_with_panic(32);

        Ok::<i32,&str>(values)
    });

    match resultado {
        Ok(_) => println!("Tudo certo"),
        Err(_) => println!("Deu ruim")   
    }
   
}

fn function_with_panic(valor: i32) -> i32 {
    if valor == 0 { 
        panic!("Valor não pode ser negativo");
    }

    valor
}
