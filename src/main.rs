mod category;
mod input;
mod transaction;

use category::{create_category, Category};
use input::get_input;

fn main() {
    let mut categories: Vec<Category> = Vec::new();

    loop {
        println!("\nSelect an option:\nq: Quit\n1: Create category\n2: Show categories");
        let choice = get_input();
        match choice.as_str() {
            "q" => break,
            "1" => {
                let category = create_category();
                println!("Created category {:?}", category);
                categories.push(category);
            }
            "2" => {
                for category in &categories {
                    println!("{:?}", category);
                }
            }
            _ => {}
        }
    }
}
