#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

/// implementations are only associated with a struct or enum, and provide ways
/// for us to associate custom logic with Coin.
impl Coin {
    pub fn from_string(str: &str) -> Option<Coin> {
        match str {
            "Penny" => Some(Coin::Penny),
            "Nickel" => Some(Coin::Nickel),
            "Dime" => Some(Coin::Dime),
            "Quarter" => Some(Coin::Quarter),
            // "_" means every other possible condition
            // since match statements are exhaustive.
            _ => {
                eprintln!("[ERROR] '{}' not a valid Coin!", str);
                None
            }
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            Coin::Penny => "Penny".to_string(),
            Coin::Nickel => "Nickel".to_string(),
            Coin::Dime => "Dime".to_string(),
            Coin::Quarter => "Quarter".to_string(),
        }
    }

    pub fn get_value(&self) -> u64 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}
