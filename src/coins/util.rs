use rand::Rng;
use std::fs::OpenOptions;
use std::{
    fs::File,
    io::{ErrorKind, Write},
};

use crate::coins::coin::Coin;

/// this function is an example of generating a change file in the root directory of the program.
///
/// # Panics
///
/// Panics if .
///
/// # Errors
/// 
/// ```rust
/// let file = generate_change("file",coins);
/// ```
///
/// This function will return an error if .
pub fn generate_change_file(
    filename: &str,
    coins_list: Vec<String>,
) -> Result<String, std::io::Error> {
    // try to open a file. Files are automatically closed at end of function.
    let mut change_file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
    {
        // if our file exists, we return the 'file'
        Ok(file) => file,
        // if we get an error opening the file:
        Err(err) => match err.kind() {
            // if file isnt found, create it.
            ErrorKind::NotFound => match File::create(filename) {
                // return file if it worked.
                Ok(fc) => fc,
                // otherwise notify user creation failed.
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // handle other errors with general message.
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // write coins list to file
    for c in coins_list {
        writeln!(change_file, "{}", c)?;
    }

    Ok(String::from("Everything looks good!"))
}

pub fn generate_coins(mut num_coins: Option<u32>) -> Vec<String> {
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];

    let mut output: Vec<String> = Vec::new();

    if num_coins.is_none() {
        num_coins = Some(50);
    }

    for _ in 0..num_coins.unwrap() {
        let num = rand::thread_rng().gen_range(0..coins.len());
        let c: &Coin = coins.get(num).unwrap();
        output.push(c.get_name());
    }

    output
}

pub fn count_coins(filename: &str) -> String {
    let collection: Vec<String> = std::fs::read_to_string(filename)
        .expect("unable to open file")
        .lines()
        .map(String::from)
        .collect();

    if collection.is_empty() {
        return "no coins found :(".to_string();
    }

    let mut coin_amount: u64 = 0;

    for item in collection {
        // if there is a empty string, ignore it.
        if item.is_empty() {
            continue;
        }
        if let Some(coin) = Coin::from_string(&item) {
            coin_amount += coin.get_value();
        }
    }

    let result: String = match coin_amount >= 100 {
        true => {
            let dollar = coin_amount % 100;
            let cents = coin_amount.rem_euclid(dollar);
            format!("you have {} dollars and {} cents!", dollar, cents)
        }
        false => {
            format!("you have {} cents!", coin_amount)
        }
    };
    result
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_coin_value() {
        assert_eq!(Coin::Nickel.get_value(), 5);
    }

    #[test]
    fn test_count_coins() {
        let result = count_coins("test.txt");
        assert_eq!(result, "you have 40 cents!".to_string());
    }
}
