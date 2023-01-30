# Countdown Timer


## New Additions

### **Things not implemented in the [last](https://github.com/GiddyLinux/denary-to-binary-convertor) mini project**
    - Functions
    - Match statements for Ok and Err
    - Sleep function
    - Iterative loops
    - Clearing screen input



## Features
    - Error handling 
        - Positive Overflow Error
        - Negative Overflow Error
        - Invalid Digit Error
    - Fully commented for clarity
    - Sorted into clear functions

## Installation

### **Use the binary provided in releases**

### Self Compilation
This section is for people who want to compile the project themselves

 Requires [Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/) to compile

```sh
git clone https://github.com/GiddyLinux/countdown-timer.git
cd countdown-timer
cargo build --release --target <platform of choice>
```

## Usage

Below is an example demonstration of the program, since after each number the program will clear the line you cannot see the countdown in action from static text
```sh
$ ./countdown-timer-x86_64-unknown-linux-gnu
Countdown Number: 5
[+] Starting a 5 second timer
Times up!
```
Below is an example of an error possibly triggered by misusing the program
```sh
$ ./countdown-timer-x86_64-unknown-linux-gnu
Countdown Number: 0
Cannot count down from 0!
```
