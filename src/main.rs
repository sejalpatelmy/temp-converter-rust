use std::io;

fn main() {
    println!("Hello, world!");

    println!("Welcome to temp convert");
    println!("We convert Celsius to Fahrenheit");

    let mut degree = String::new();

    io::stdin()
        .read_line(&mut degree)
        .expect("Failed to read the degree");

    let degree: f64 = degree
        .trim()
        .parse()
        .expect("Please enter a valid number");

    println!("Number entered is {}", degree);
    println!("Number is correct");
    println!(
        "Converting {} Celsius to Fahrenheit...",
        degree
    );

    let far = degree * 1.8 + 32.0;

    println!("Fahrenheit is {}", far);
}
