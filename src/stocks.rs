pub fn get_user_number<T: std::str::FromStr>(question:String) -> T {
    loop{
        let mut input = String::new();
        println!("{}", question);
        match std::io::stdin().read_line(&mut input){
            Err(error) => {
                println!("Error: {}", error);
                continue;
            }
            Ok(_) => {}
        }
        match input.trim().parse(){
            Err(_) => {
                println!("NaN!");
                continue;
            }
            Ok(value) => return value,
        };
    }
}