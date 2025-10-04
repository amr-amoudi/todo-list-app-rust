use std::io;

pub fn use_integer_input() -> i32{
    loop {
        let input = use_input();
        
        match input.trim().parse::<i32>() {
            Ok(i) => return i,
            Err(_) => println!("invalid input must be a number"),
        }
    }
}

pub fn use_input() -> String{
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


