// imports the cmp, io and num libraries
use std::cmp::Ordering;
use std::io::{stdin, stdout, Write};
use std::num::IntErrorKind;

fn user_input() -> i32 {
    // initiates string called time
    let mut time = String::new();
    // gives input prompt asking for a countdown number
    write!(stdout(), "Countdown Number: ").unwrap();
    // makes sure that the text is written to the screen
    stdout().flush().unwrap();
    // reads the user input from the screen
    stdin().read_line(&mut time).unwrap();
    // converts the users input into an integer
    match time.trim().parse::<i32>() {
        Ok(t) => {
            // checks if the user input is 0 or less
            match t.partial_cmp(&0) {
                // checks if t (the integer entered) is less than 0
                Some(Ordering::Less) => {
                    println!("Cannot count down from negative numbers!");
                    return 0;
                }
                // checks if t (the integer entered) is greater than 0
                Some(Ordering::Greater) => return t,
                // checks if t (the integer entered) is 0
                Some(Ordering::Equal) => {
                    println!("Cannot count down from 0!");
                    return 0;
                }
                // checks if t (the integer entered) is neither Greater, Lesser or Equal
                None => {
                    println!("Invalid input!");
                    return 0;
                }
            }
        }

        Err(e) => {
            match e.kind() {
                // catches Positive Overflow errors
                IntErrorKind::PosOverflow => {
                    // prints error message
                    println!("Positive Overflow: The number you entered is too big!");
                    return 0;
                }
                // catches Negative Overflow errors
                IntErrorKind::NegOverflow => {
                    // prints error message
                    println!("Negative Overflow: The number you entered is too small!");
                    return 0;
                }

                // catches wildcard errors
                _ => {
                    println!("Error: {}", e);
                    return 0;
                }
            }
        }
    }
}

fn main() {
    // binds the outcome of user_input to the time variable
    let time = user_input();
    // checks if the code is equal to 0 (and error has been thrown)
    if time == 0 {
        return;
    }
    // tells the user the time has started along with the amount of time
    println!("[+] Starting a {} second timer ", time);
    // iterates from the int time -> 1. .rev() allows the code to iterate backwards (10, 9, 8 etc)
    for i in (1..=time).rev() {
        // prints the current point in the timer
        print!("{}", i);
        // makes sure that the text is written to the screen
        stdout().flush().unwrap();

        // clears the line so the countdown can stay on one line
        print!("\r");
        // waits one second
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("Times up!")
}
