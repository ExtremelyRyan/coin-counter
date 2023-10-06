This is a simple example of rust that I threw together to help give a more "real" look at what rust is and how it operates. 


this program asks the user for a number of coins, and stores them in a file called `change.txt`. if this file does not exist, it will be created. 

`change.txt` will then be read into memory and total up the amount of money contained in the file.


to run the tests at the bottom of main.rs, use `cargo test`