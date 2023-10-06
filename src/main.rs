pub mod coins;

use std::io;

fn main() {
    

    println!("please enter number of coins to generate: ");

    let mut num_coins = String::new();

    io::stdin()
        .read_line(&mut num_coins)
        .expect("Failed to read line");

    let number = num_coins.trim().parse::<u32>();

    let coins_list: Vec<String> = match number {
        Ok(num) => coins::util::generate_coins(Some(num)),
        Err(_) => {
            eprintln!("entry was not a number. Using default number (50).");
            coins::util::generate_coins(None)
        }
    }; 

    let file = "change.txt";
    match coins::util::generate_change_file(file, coins_list) {
        Err(e) => panic!("oops! Something went wrong: {} ", e),
        Ok(str) => println!("string from genereate_change_file:\n {str}"),
    }; 
    
    println!("{}", coins::util::count_coins(file));
}
