/* 2 Types of Strings
1. Primitive str = Immutable fixed-length string somewhere in memory
2. String = Growable, heap-allocated data structure - Use when you need to modify or own string data
 */
pub fn run() {
let mut mom = String::from("Mom");
println!("Length: {}",mom.len());
//Push char
mom.push('R');
//Push string
mom.push_str("ocks!");
//Capacity in bytes
println!("Capacity: {}", mom.capacity());
//Check if empty
println!("Is Empty: {}", mom.is_empty());
//Contains
println!("Contains 'Rocks'{}", mom.contains("Rocks"));
//Replace
println!("{}", mom.replace("Rocks", "Sucks"));
//Loop through string by whitespace
for token in mom.split_whitespace() {
    println!("{}", token);
}
println!("{}", mom);
//Create string with capacity
let mut s = String::with_capacity(10);
s.push('a');
s.push('b');
println!("{}", s);
//Assertions
assert_eq!(2, s.len());
assert_eq!(10, s.capacity());
println!("{}", s);

}