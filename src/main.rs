use crate::stocks::get_user_number;

mod vector_addition;
mod stocks;

fn main() {
    loop{
        let num:i32 = get_user_number(format!("\nList of Applications (type to play):\n1. Vector Addition"));
        match num {
            1 => vector_addition::solve_vector_addition(),
            _ => println!("Try an actual selection this time."),
        }
    }
}