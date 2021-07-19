//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language
pub fn run() {
let name = "Jake";
let mut  age = 37;
println!("My name is {}", name, age);
age = 38;
println!("My name is {} and I am {}", name, age);
//Define constant
const ID: i32 = 001;
println!("ID: {}",ID);
//Assign multiple variable
let (my_name, my_age) = ("Bailey Jay", 32);
println!("{} is {}", my_name, my_age);
}