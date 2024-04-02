use std::io;

fn main() {
    loop {
        println!("\nSelect an option:\nq: Quit\n1: Create category");
        let choice = get_input();
        match choice.as_str() {
            "q" => break,
            "1" => {
                let category = create_category();
                println!("Created category {:?}", category);
            }
            _ => {}
        }
    }
}

fn create_category() -> Category {
    println!("Please enter category name:");
    let name = get_input();
    println!("Please enter category budgeted amount:");
    let budgeted_amount = get_float_input();
    println!("Please enter category spent amount:");
    let spent_amount = get_float_input();

    Category {
        name,
        budgeted_amount,
        spent_amount,
    }
}

fn get_input() -> String {
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

fn get_float_input() -> f64 {
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

#[derive(Debug)]
struct Category {
    name: String,
    budgeted_amount: f64,
    spent_amount: f64,
}

#[cfg(test)]
mod tests {
    use crate::create_category;

    #[test]
    fn test_create_category() {}
}
