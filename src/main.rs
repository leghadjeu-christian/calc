use std::{io, env};
use std::fs::OpenOptions;


fn main() {
     let file = OpenOptions::new().write(true).create(true).open("foo.txt");

    let result: f64;

 let args: Vec<String> = env::args ().collect();
  
  if args.len() != 0 {

  }

    println!("Enter the first number: ");
    let x: f64 = input_parser();

    if f64::is_nan(x) {
        println!("Invalid input!");
        return;
    }

    println!("Enter the second number: ");
    let y: f64 = input_parser();


    if f64::is_nan(y) {
        println!("Invalid input!");
        return;
    }

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("(5) exponentiation");
    println!("(6) modulus");
    println!("Select the number an operation: ");

    let operation: f64 = input_parser();

    if f64::is_nan(operation) {
        println!("Invalid input!");
        return;
    }

    let operation: i32 = operation as i32;

    match operation {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => { if y == 0.0 {
            println!("cannot divide by zero ");
            return;
        } else 
             {result = x / y}},
        5 => result = x.powf(y),
        6 => result = x % y,
        _ => {
            println!("Invalid selection");
            return;
        }
    }

    println!("The result is: {}", result);
}

fn input_parser() -> f64 {
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).expect("Invalid Input");
    let x: f64 = match x.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            return f64::NAN;
        }
    };

    return x;
}