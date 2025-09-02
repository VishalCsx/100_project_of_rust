use std::io;
fn main(){
    println!("Simple CLI Calculator");
    println!("Oparations : +,-,*,/");

    // Get the first input 
    let mut input = String::new();
    println!("Please Enter the first number");
     io::stdin().read_line(&mut input).expect("failed to read line");
     let num1:f64 = input.trim().parse().expect("Invalid number");
     // Get Operators

     input.clear();
     println!("please Enter oprator : +,-,*,/");
     io::stdin().read_line(&mut input).expect("failed to read line");
     let op = input.trim().to_string();

     // Second Number 
     input.clear();

    println!("Please enter the second number ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num2:f64 = input.trim().parse().expect("Invalid number ");

    // calculator

    let result = match op.as_str(){
        "+" => num1+num2,
        "-" => num1-num2,
        "*" => num1*num2,
        "/"=>{
            if num2!=0.0{
                num1/num2
            }else {
                println!("Error : divison by the zero");
                return;
            }
        }
        _=>{
            println!("Invalid operator");
            return;
        }
    };
    println!("Result : {}",result)
}