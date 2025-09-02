use std::io;
fn main(){
println!("Palidrome checker");
let mut  input = String::new();
io::stdin().read_line(&mut input).expect("failed to read line");
let text = input.trim().to_lowercase();

let reversed:String = input.trim().chars().rev().collect();

if text==reversed{
    println!("{} is the palidrome",text );
}else {
      println!("{} is not the palidrome", text );
}

}