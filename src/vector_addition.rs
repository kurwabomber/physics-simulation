use crate::stocks::get_user_number;

struct MathVector{
    scalar: f64,
    angle: f64,
}
impl MathVector{
    fn add_vector(&self, vec2:MathVector) -> MathVector{
        let result_base:f64 = (self.angle.to_radians().cos() * self.scalar) + (vec2.angle.to_radians().cos() * vec2.scalar);
        let result_height:f64 = (self.angle.to_radians().sin() * self.scalar) + (vec2.angle.to_radians().sin() * vec2.scalar);
        let result_hypotenuse:f64 = (result_base.powf(2.0) + result_height.powf(2.0)).sqrt();
        let result_angle:f64 = (result_height/result_hypotenuse).asin().to_degrees();

        return MathVector{
            scalar: result_hypotenuse, 
            angle: result_angle,
        }
    }
    fn print(&self){
        println!("Vector has a magnitude of {} and an angle of {}.", self.scalar, self.angle);
    }
}

pub fn solve_vector_addition() {
    println!("||| VECTOR ADDITION |||");
    let mut vec1 = MathVector{
        scalar: 0.0,
        angle: 0.0,
    };
    let mut vec2 = MathVector{
        scalar: 0.0,
        angle: 0.0,
    };

    vec1.scalar = get_user_number(format!("What's the magnitude of vector #1?"));
    vec1.angle = get_user_number(format!("What's the angle of vector #1?"));
    vec2.scalar = get_user_number(format!("What's the magnitude of vector #2?"));
    vec2.angle = get_user_number(format!("What's the angle of vector #2?"));
    
    vec1.add_vector(vec2).print();
}

