use crate::stocks::get_user_number;

mod vector_addition;
mod planar_movement;
mod stocks;

const APPLICATIONS: &'static [&'static str] = &["1. Vector Addition", "2. Planar Movement"];

fn main() {
    loop{
        println!("\nList of Applications");
        for key in APPLICATIONS{
            println!("{}", key);
        }
        let num:i32 = get_user_number("".to_string());
        match num {
            1 => vector_addition::solve_vector_addition(),
            2 => planar_movement::solve_planar_movement(),
            _ => println!("Try an actual selection this time."),
        }
    }
}