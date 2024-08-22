fn main() {
    println!("Hello, for!");
    // exclude upper bound
    for i in 0..5 {
        println!("i = {}", i);
    }
    println!("-----------1-----------");

    // include upper bound
    for i in 0..=5 {
        println!("i = {i}");
    }
    println!("-----------2-----------");

    // rev() method
    for i in (1..=10).rev() {
        println!("i = {i}");
    }
    println!("-----------3-----------");

    // step_by() method
    for i in (0..=50).step_by(9) {
        println!("i = {i}");
    }
}
