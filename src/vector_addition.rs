use crate::stocks::get_user_number;

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
        let value:f64 = get_user_number(format!("What's the magnitude of vector #{}?", i+1));
        let angle:f64 = get_user_number(format!("What's the angle of vector #{}?", i+1));

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

