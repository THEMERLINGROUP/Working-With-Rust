pub fn run(){
sell("Rosemary", 25);
let get_sum = add(2,5);
println!("Sum: {}", get_sum);
//Closure
let n3: i32 = 10;
let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
println!("C Sum: {}", add_nums(3,30)); 
}
fn sell(product: &str, cost: i64) {
    println!("{} {} is the name of product and cost in the sale.", product, cost);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}