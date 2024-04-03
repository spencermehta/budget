pub fn get_input() -> String {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                break;
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    String::from(input.strip_suffix("\n").unwrap())
}

pub fn get_float_input() -> f64 {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                break;
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    let float_input = input.strip_suffix("\n").unwrap().parse::<f64>();
    float_input.unwrap()
}
