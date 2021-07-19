//Loops - iterate until condition is met
pub fn run() {
//Infinite Loops
let mut count = 0;
/*loop {
    count +=1;
    println!("Number: {}", count);
    if count == 20 {
        break;
    }
}
 */
//While FizzBuzz
while count <= 100 {
    if count  % 15 == 0 {
        println!("FIZZBUZZ");
    } else if count % 3 == 0 {
    println!("FIZZ");
    } else if count % 5 == 0 {
        println!("BUZZ")
    } else {    
        println!("{}", count);
    }
    count += 1;
}
// For Range
for x in 0..100 {
    if x % 2 == 0 {
        println!("Even");
    } else if x % 2 == 1 {
        println!("Odd");
    } else {
        println!("{}", X);
    }
}
}