use std::io;

/// Function to generate the Fibonacci number at position `n`
fn fibonacci(n: u32) -> u64 {
    if n == 0 {

        return 0;
    } else if n == 1 {

        return 1;
    } else {

        let mut prev = 0;
        let mut curr = 1;

        for _ in 2..=n {

            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        return curr;
    }
}

fn main() {
    println!("Welcome to my Project!");
    println!("I created a fibonacci sequence generator");
    println!();

    println!("Enter the position of the Fibonacci number you want:");
    println!();
    
    loop {
        println!("Enter the position of the Fibonacci number you want (or type 'exit' to quit):");

        // Read user input from the console
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // Check if the user wants to exit
        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        // Parse user input to an unsigned 32-bit integer
        let position: u32 = match trimmed_input.parse() {

            Ok(num) => num,

            Err(_) => {
                println!("Please enter a valid number or type 'exit' to quit.");
                continue;
            }
        };

        // Generate and print the Fibonacci number at the specified position
        let fibonacci_number = fibonacci(position);

        println!("Fibonacci number at position {}: {}", position, fibonacci_number);
    }
}
