use crate::stocks::get_user_number;

mod vector_addition;
mod planar_movement;
mod stocks;
mod energy;

const APPLICATIONS: &'static [&'static str] = &["1. Vector Addition", "2. Planar Movement", "3. Kinematic Equations", "4. Work, Energy, & Power", "5. Rotation", "6. Springs", "7. Waves", "8. Light", "9. Electric Fields/Forces", "10. Electrical Energy", "11. Circuits"];

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
            3 => planar_movement::solve_kinematics(),
            4 => energy::solve_work(),
            _ => println!("Not implemented."),
        }
    }
}