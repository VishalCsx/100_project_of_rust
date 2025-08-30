use::std::io;
use std::{io::stdin, result};
fn main(){
    println!("Temprature Convertor CLI");
    println!("Choose a conversion option :");
    println!("1. Celsius -> Fahrenheit");
    println!("2. Fahrenheit -> Celsius");
    println!("3. Celsius -> Kelvin");
    println!("4. Kelvin -> Celsius");
    println!("5. Farhrenheit -> Kelvin");
    println!("6. Kelvin -> Fahrenheit");

    // take options input

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice:u32 = choice.trim().parse().expect("Please enter the number ");



    // take the tempature value
    println!("Enter the tempature value");
    let mut value = String::new();
 io::stdin().read_line(&mut value).expect("Failed to read line");
 let value :f64 = value.trim().parse().expect("Please enter a number");


 // Match conversion
 let result = match  choice {
    1 => (value * 9.0/5.0)+32.0,
    2 => (value - 32.0)*5.0/9.0,
    3 => (value + 273.15),
    4 => (value -  273.15),
    5 => (value - 32.0) * 5.0/9.0+273.15,
    6 =>(value - 273.15)*9.0/5.0 + 32.0,
 _ =>{
    println!("Invalid options");
    return;
 }
     
 };
println!("Converted value = {:.2}",result)
}