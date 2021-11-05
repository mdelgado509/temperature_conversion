use std::io;

fn main() {
    println!("Would you like to convert to Fahrenheit or Celcius?");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line.");
    
    println!("Please input the temperature");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line.");

    let temp: f64 = temp.trim().parse().expect("Please type a number!");

    let result = if choice == "Fahrenheit" {
        temp * 1.8 + 32.0 
    } else {
        (temp - 32.0) / 1.8 
    };

    println!("The temperature in {} is: {:.1} degrees", choice, result);
}
