use crate::stocks::get_user_number;

mod vector_addition;
mod planar_movement;
mod stocks;

const APPLICATIONS: &'static [&'static str] = &["1. Vector Addition", "2. Planar Movement", "3. Kinematic Equations", "4. Friction", "5. Work, Energy, & Power", "6. Rotation", "7. Springs", "8. Waves", "9. Light", "10. Electric Fields/Forces", "11. Electrical Energy", "12. Circuits"];

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
            4 => 
            _ => println!("Not implemented."),
        }
    }
}