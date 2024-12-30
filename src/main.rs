use std::io::Read;
use std::fs::File;



fn main() {

    let file = read_file("sample.txt");

    match file {
        Ok(contents) => {
            println!("File contents: {}", contents);
        },
        Err(e) => {
            println!("Error reading file: {}", e);
        }
    }
   
}

fn read_file(path : &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;

    let mut contents = String::new();

    file.read_to_string( &mut contents)?;

    Ok(contents)
}


