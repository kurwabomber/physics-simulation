use crate::stocks::get_user_number;
pub fn solve_planar_movement() {
    println!("||| PLANAR MOVEMENT |||\nInsert -1 to solve algebraically.");
    //$WAG
    let mut acceleration:f64 = get_user_number(format!("What's the acceleration?"));
    let mut velocity:f64 = get_user_number(format!("What's the initial velocity?"));
    let mut time:f64 = get_user_number(format!("How long is the duration?"));
    let mut displacement:f64 = get_user_number(format!("How far did it travel?"));

    if acceleration == -1.0
    {
        acceleration = ((displacement*2.0/time)-velocity*2.0)/time;
    }
    else if velocity == -1.0
    {
        velocity = ((displacement*2.0/time)-time*acceleration)/2.0;
    }
    else if time == -1.0
    {
        time = ((-2.0*velocity) - f64::sqrt((velocity*2.0).powf(2.0) - (4.0*acceleration) * (displacement * -2.0)))/(2.0*acceleration);
    }
    else
    {
        let table_amount:i32 = get_user_number(format!("How many rows of data do you want?"));
        for i in 0..table_amount{
            println!("{:7}s | velocity = {:7}m/s | displacement = {:7}m", i, velocity + acceleration * i as f64, i as f64*(2.0*velocity + acceleration*i as f64)/2.0);
        }

        displacement = (2.0*velocity + time*acceleration)/2.0 * 3.0;
    }

    println!("||| Solution |||\nTime = {}s\nVelocity = {}m/s\nAcceleration = {}m/s^2\nDisplacement = {}m", time, velocity, acceleration, displacement);
}

pub fn solve_kinematics() {
    println!("||| KINEMATICS |||\nInsert -1 to solve algebraically.");
    //Based on the missing variables, get the other values.
    let mut acceleration:f64 = get_user_number(format!("What's the acceleration?"));
    let mut velocity:f64 = get_user_number(format!("What's the initial velocity?"));
    let mut fvelocity:f64 = get_user_number(format!("What's the final velocity?"));
    let mut time:f64 = get_user_number(format!("How long is the duration?"));
    let mut displacement:f64 = get_user_number(format!("How far did it travel?"));

    //equation one, vf = v0 + at
    if fvelocity == -1.0 && velocity != -1.0 && acceleration != -1.0 && time != -1.0
    {
        fvelocity = velocity + acceleration * time;
    }
    else if velocity == -1.0 && fvelocity != -1.0 && acceleration != -1.0 && time != -1.0
    {
        velocity = fvelocity - acceleration * time;
    }
    else if acceleration == -1.0 && fvelocity != -1.0 && velocity != -1.0 && time != -1.0
    {
        acceleration = (fvelocity-velocity)/time;
    }
    else if time == -1.0 && fvelocity != -1.0 && acceleration != -1.0 && velocity != -1.0
    {
        time = (fvelocity-velocity)/acceleration;
    }
    //equation two, displacement = ((vf+v0)/2)t
    if displacement == -1.0 && fvelocity != -1.0 && velocity != -1.0 && time != -1.0
    {
        displacement = ((fvelocity+velocity)/2.0)*time;
    }
    else if fvelocity == -1.0 && displacement != -1.0 && velocity != -1.0 && time != -1.0
    {
        fvelocity = ((displacement*2.0)/time)-velocity;
    }
    else if velocity == -1.0 && displacement != -1.0 && fvelocity != -1.0 && time != -1.0
    {
        velocity = ((displacement*2.0)/time)-fvelocity;
    }
    else if time == -1.0 && displacement != -1.0 && fvelocity != -1.0 && velocity != -1.0
    {
        time = displacement/((fvelocity+velocity)/2.0);
    }
    //equation three, displacement = v0*t + (acceleration*time^2)
    if displacement == -1.0 && fvelocity != -1.0 && time != -1.0 && acceleration != 1.0
    {
        displacement = (fvelocity*time) + (acceleration*f64::powf(time, 2.0))/2.0;
    }
    else if fvelocity == -1.0 && displacement != -1.0 && time != -1.0 && acceleration != 1.0
    {
        fvelocity = (displacement - (acceleration*f64::powf(time, 2.0))/2.0 )/time;
    }
    else if acceleration == -1.0 && displacement != -1.0 && time != -1.0 && fvelocity != 1.0
    {
        acceleration = 2.0*(displacement - (fvelocity * time))/f64::powf(time, 2.0);
    }
    else if time == -1.0 && displacement != -1.0 && acceleration != -1.0 && fvelocity != 1.0
    {
        let d_vel:f64 = 2.0*fvelocity;
        time = (-(d_vel) + f64::sqrt( (d_vel*d_vel) - 4.0 * (d_vel*-displacement*2.0)))/(d_vel*2.0);
    }
    //equation four, vf^2 = v0^2 + 2*acceleration*displacement
    if fvelocity == -1.0 && velocity != -1.0 && acceleration != -1.0 && displacement != -1.0
    {
        fvelocity = f64::sqrt(f64::powf(velocity, 2.0) + (2.0*acceleration*displacement));
    }

    println!("||| Solution |||\nTime = {}s\nInitial Velocity = {}m/s\nFinal Velocity = {}m/s\nAcceleration = {}m/s^2\nDisplacement = {}m", time, velocity, fvelocity, acceleration, displacement);
}