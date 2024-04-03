use crate::input::{get_float_input, get_input};

#[derive(Debug)]
pub struct Category {
    pub name: String,
    pub budgeted_amount: f64,
    pub spent_amount: f64,
}

pub fn create_category() -> Category {
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
