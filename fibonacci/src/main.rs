use std::io;
fn main() {
    println!("Please Enter a number.");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read line");
    let number: i64 = match input_text.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid integer.");
            return;
        }
    };
    //Generating Fibonacci number based on formula
    let mut count = 0;
    let mut fib1 = 1;
    let mut fib2 = 0;
    let mut finalfib = 0;
    println!("{fib2}");
    if number == 0 {
        return println!("0 is the 1st number of the Fibonacci Sequence.");
    }
    println!("{fib1}");
    loop {
        count += 1;
        finalfib = fib1 + fib2;
        if finalfib <= 1 {
            println!("{finalfib}");
        }
        else if number == 1 {
        return println!("1 is both the 2nd and 3rd numbers of the Fibonacci Sequence.");
    }

        else {
            println!("{finalfib}");
            if finalfib == number {
                return println!("{number} is the {y}{x} Fibonacci number.", y = count + 2);
            }
            else if finalfib > number {
                return println!("Num does not exist in sequence.")
            }
        }
        fib2 = fib1;
        fib1 = finalfib;
    }
}