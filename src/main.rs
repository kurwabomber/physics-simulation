mod vector_addition;

fn main() {

    println!("whats up gang\nList of Applications (type to play):\n1. Vector Addition");
    loop{
        let mut input1 = String::new();
        match std::io::stdin().read_line(&mut input1){
            Err(error) => {
                println!("ay lmao!!!! that's not a string! Error: {}", error);
                continue;
            }
            Ok(_) => println!(""),
        }
        
        let num:i32 = match input1.trim().parse(){
            Err(error) => {
                println!("Hey bitch! that's not a number! Error: {}", error);
                continue;
            }
            Ok(num) => num,
        };

        match num {
            1 => vector_addition::solve_vector_addition(),
            _ => println!("Try an actual selection this time."),
        }
    }
}
