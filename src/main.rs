use std::io;

fn main() {
    println!("Hello, world!");

    println!("Welcome to temp convert");
    println!("We convert Celicus to Farenhit");
    let mut degree = String::new();
    io::stdin().read_line(&mut degree).expect("Failed to read the Degree");
    let degree: f64 = degree.trim().parse().expect("please enter valid number");

    println!("number entered is {}",degree);
    println!("number is correct");
    println!("converting it to degree celcius {} to farenhit ", degree);
   let far: i32 = degree  * 1.8 + 32;
    
    let check = degree * 1.8 ; 
    println!("farhente is {} ",check);

    println!("farhente is {} ",far);
}
