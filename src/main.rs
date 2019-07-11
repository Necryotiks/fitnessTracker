mod profiles;
mod person;
mod foods;
mod utils;
extern crate chrono;


use std::io;
use profiles::*;
fn main() {
    //TODO: everything
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    //user input here
    loop {
        //Clear screen with ANSI commands
        print!("{}[2J{}[;H", 27 as char, 27 as char);
        println!("Welcome to fitness app ver. {}", VERSION);
        println!("1. New profile");
        println!("2. Load profile");
        println!("0. Save and Quit");
        //expand as needed

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().as_ref() {
                    "0" => {
                        Profile::save_profile();
                        break;
                    }
                    "1" => Profile::create_profile(),
                    "2" => Profile::load_profile(),
                    _ => (),
                }
            }
            Err(_) => println!("Unable to read line!"),
        }

    }
}
