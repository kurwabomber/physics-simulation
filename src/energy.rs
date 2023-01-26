use crate::stocks::get_user_number;

pub fn solve_work(){
    println!("There are multiple formulas for work.");
    println!("1. W=FDcos(degree)");
    println!("2. W=Change in KE");
    println!("3. W=P/T");
    println!("4. KE= MV^2 / 2");
    let option:i32 = get_user_number(format!("Which one do you want to choose?"));
    println!("To select a variable to solve for, input -1 for a varaible.");
    loop{
        match option{
            1 => {
                let mut work:f64 = get_user_number(format!("How much work was done?"));
                let mut force:f64 = get_user_number(format!("How much force was applied? (mass x distance)"));
                let mut distance:f64 = get_user_number(format!("How much displacement on the object was applied?"));
                let mut angle:f64 = get_user_number(format!("At what angle was the force applied? (in degrees to normal, 0 for direct.)"));

                if work == -1.0 && force != -1.0 && distance != -1.0 && angle != -1.0{
                    work = force*distance*angle.to_radians().cos();
                }else if force == -1.0  && work != -1.0 && distance != -1.0 && angle != -1.0{
                    force = work/(distance*angle.to_radians().cos());
                }else if distance == -1.0 && work != -1.0 && force != -1.0 && angle != -1.0{
                    distance = work/(force*angle.to_radians().cos());
                }else if angle == -1.0 && work != -1.0 && force != -1.0 && distance != -1.0{
                    angle = (work/(force*distance)).to_radians().acos();
                }
                println!("Work: {}\nForce: {}\nDistance: {}\nAngle: {}", work, force, distance, angle);
                break;
            },
            2 => {
                break;
            },
            3 => {
                break;
            },
            4 => {
                break;
            },
            _ => println!("Choose again."),
        }
    }
}