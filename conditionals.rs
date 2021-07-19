//Conditionals - Check condition of something and act on the result
pub fn run() {
let age: u8 = 51;
let check_id: bool = false;
let knows_bartender = true;
if age >= 21 && check_id || knows_bartender {
    println!("Bartender: Welcome");
} else if age < 21 && check_id {
    println!("Bartender: You gotta go!");
} else {
    println!("Bartender: I need to see some ID");
}
//Shorthand IF
let is_of_age = if age >= 21 {true} else {false};
println!("Is of age: {}", is_of_age);
}