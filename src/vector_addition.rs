struct MathVector{
    scalar: f64,
    angle: f64,
}
impl MathVector{
    fn add_vector(&self, vec2:MathVector){
        let result_base:f64 = (self.angle.to_radians().cos() * self.scalar) + (vec2.angle.to_radians().cos() * vec2.scalar);
        let result_height:f64 = (self.angle.to_radians().sin() * self.scalar) + (vec2.angle.to_radians().sin() * vec2.scalar);
        let result_hypotenuse:f64 = (result_base.powf(2.0) + result_height.powf(2.0)).sqrt();
        let result_angle:f64 = (result_height/result_hypotenuse).asin().to_degrees();

        println!("The resultant vector is: {} at {} degrees.", result_hypotenuse, result_angle);
    }
}

pub fn solve_vector_addition() {
    println!("||| VECTOR ADDITION |||");

    let mut i = 0;
    let mut vec1 = MathVector{
        scalar: 0.0,
        angle: 0.0,
    };
    let mut vec2 = MathVector{
        scalar: 0.0,
        angle: 0.0,
    };

    //Get our two vectors
    loop{
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("What's the scalar value of vector #{}?", i+1);
        match std::io::stdin().read_line(&mut input1){
            Err(error) => {
                println!("that's not a string Error: {}", error);
                continue;
            }
            Ok(_) => println!(""),
        }
        
        let value:f64 = match input1.trim().parse(){
            Err(error) => {
                println!("Hey bitch! that's not a number! Error: {}", error);
                continue;
            }
            Ok(value) => value,
        };

        println!("What's the angle of vector #{}?", i+1);
        match std::io::stdin().read_line(&mut input2){
            Err(error) => {
                println!("ay lmao!!!! that's not a string! Error: {}", error);
                continue;
            }
            Ok(_) => println!(""),
        }
        
        let angle:f64 = match input2.trim().parse(){
            Err(error) => {
                println!("Hey bitch! that's not a number! Error: {}", error);
                continue;
            }
            Ok(angle) => angle,
        };

        if i == 0 {
            vec1.scalar = value;
            vec1.angle = angle;
        }else{
            vec2.scalar = value;
            vec2.angle = angle;
        }

        i += 1;
        if i >= 2{
            break;
        }
    }
    vec1.add_vector(vec2);
}

