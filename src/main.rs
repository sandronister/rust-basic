

fn main() {

    let resultado = std::panic::catch_unwind(|| {
        let values =function_with_panic(32);

        Ok::<i32,&str>(values)
    });

    match resultado {
        Ok(valor ) => println!("Tudo certo {}", valor.unwrap()),
        Err(_) => println!("Deu ruim")   
    }

    let result_div = divide(10, 2);

    match result_div {
        Ok(value) => println!("Resultado: {}", value),
        Err(err) => println!("Erro: {}", err)
    }
   
}

fn function_with_panic(valor: i32) -> i32 {
    if valor == 0 { 
        panic!("Valor não pode ser negativo");
    }

    valor
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Divisão por zero".to_string());
    }

    Ok(a / b)
}
