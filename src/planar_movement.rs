use crate::stocks::get_user_number;

pub fn solve_planar_movement() {
    println!("||| PLANAR MOVEMENT |||");
    //Get our two vectors
    let acceleration:f64 = get_user_number(format!("What's the acceleration?"));
    let velocity:f64 = get_user_number(format!("What's the initial velocity?"));
    let table_amount:i32 = get_user_number(format!("How many rows of data do you want?"));

    for i in 0..table_amount{
        println!("{:7}s | velocity = {:7}m/s | displacement = {:7}m", i, velocity + acceleration * i as f64, i as f64*(2.0*velocity + acceleration*i as f64)/2.0);
    }
        
}

