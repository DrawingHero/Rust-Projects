use std::io;

fn main() {
    println!("Please enter a valid Fahrenheit Tempature :");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    println!("Fahrenheit : {input}Celcius : {}",(number - 32) * 5/9);
    return;
}