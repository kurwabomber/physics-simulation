use crate::stocks::get_user_number;
pub fn solve_planar_movement() {
    println!("||| PLANAR MOVEMENT |||\nInsert -1 to solve algebraically.");
    //Get our two vectors
    let mut acceleration:f64 = get_user_number(format!("What's the acceleration?"));
    let mut velocity:f64 = get_user_number(format!("What's the initial velocity?"));
    let mut time:f64 = get_user_number(format!("How long is the duration?"));
    let mut displacement:f64 = get_user_number(format!("How far did it travel?"));

    if acceleration == -1.0
    {
        acceleration = ((displacement*2.0/time)-velocity*2.0)/time;
        println!("||| Solution |||\nTime = {}s\nVelocity = {}m/s\nAcceleration = {}m/s^2\nDisplacement = {}m", time, velocity, acceleration, displacement);
    }
    else if velocity == -1.0
    {
        velocity = ((displacement*2.0/time)-time*acceleration)/2.0;
        println!("||| Solution |||\nTime = {}s\nVelocity = {}m/s\nAcceleration = {}m/s^2\nDisplacement = {}m", time, velocity, acceleration, displacement);
    }
    else if time == -1.0
    {
        time = ((-2.0*velocity) - f64::sqrt((velocity*2.0).powf(2.0) - (4.0*acceleration) * (displacement * -2.0)))/(2.0*acceleration);
        println!("||| Solution |||\nTime = {}s\nVelocity = {}m/s\nAcceleration = {}m/s^2\nDisplacement = {}m", time, velocity, acceleration, displacement);
    }
    else
    {
        let table_amount:i32 = get_user_number(format!("How many rows of data do you want?"));
        for i in 0..table_amount{
            println!("{:7}s | velocity = {:7}m/s | displacement = {:7}m", i, velocity + acceleration * i as f64, i as f64*(2.0*velocity + acceleration*i as f64)/2.0);
        }

        displacement = (2.0*velocity + time*acceleration)/2.0 * 3.0;
        println!("||| Solution |||\nTime = {}s\nVelocity = {}m/s\nAcceleration = {}m/s^2\nDisplacement = {}m", time, velocity, acceleration, displacement);
    }
}