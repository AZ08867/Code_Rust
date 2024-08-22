fn main() {
    println!("Hello, if!");
    // conditions
    let (x, y) = (1, 3);

    // if statement
    if x < y {
        println!("x is less than y");
    } else if x > y {
        println!("x is greater than y");
    } else {
        println!("x is equal to y");
    }
}
