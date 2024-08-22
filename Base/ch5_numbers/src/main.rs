fn main() {
    println!("Hello, numbers");
    let a = 42;
    println!("{} is type i32, by default", a);

    let b: i64 = -123456789;
    println!("{} is type i64", b);

    let c = 3.14159265359;
    println!("{} is type f64", c);

    println!("0.1 + 0.2 = {}", 0.1 + 0.2);

    println!("-------------");
    const PI: f64 = 3.14159;
    println!("PI = {}", PI);

    println!("PI with 2 digits = {:.2}", PI);

    println!("1000000 == 1_000_000 = {}", 1000000 == 1_000_000);
    let d = f32::sqrt(8.0);
    println!("sqrt(8.0) = {:.5}", d);
    println!("The square root of 8 is {:.5}", d);

    // from chapter on basic math
    println!("5 + 3 = {}", 5 + 3);
    println!("5 * 3 = {}", 5 * 3);
    println!("5.0 / 2.0 = {}", 5.0 / 2.0);
}
