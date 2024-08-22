fn main() {
    println!("Hello, Variable!");
    let mut x = 5;
    let y = 6;
    let mut z = x + y;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("x + y = {}", z);

    println!("---------1-----------");
    x = 2;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("x + y = {}", x + y);

    z = x * y;
    println!("---------2------------");
    println!("x = {}", x);
    println!("y = {}", y);
    println!("x * y = {}", z);

    // how to increment
    println!("---------3------------");
    let mut i = 1;
    i += 1;
    i = i + 1;
    println!("i = {}", i);
    // how to perform basic math
    // using variables
    let height = 10;
    let width = 20;
    let area = height * width;
    println!("---------4------------");
    println!("area = {}", area);
    // using literals
    let area = 15 * 25;
    println!("---------5------------");
    println!("area = {}", area);
}
