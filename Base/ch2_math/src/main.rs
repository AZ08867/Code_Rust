fn main() {
    println!("Hello, world!");
    println!("1 + 1 = {}", 1 + 1);
    println!("1 + 1 = {}", 1.0 + 1.0);
    println!("5 - 2 = {}", 5 - 2);
    println!("5 * 2 = {}", 5 * 2);
    println!("5 / 2 = {}", 5 / 2);
    println!("5 % 2 = {}", 5 % 2);
    println!("6.0 / 2.0 = {}", 6.0 / 2.0);
    // exponents
    println!("2.0 ^ 3.0 = {}", 2.0_f32.powf(3.0));
    println!("2 ^ 3 = {}", 2_i64.pow(3));
    println!("2 ^ 3 = {}", i32::pow(2, 3));
    // order of operations
    println!(
        "3 + (7 - 5)^4 * 3 / 2 * 5 % 2 = {}",
        3 + i32::pow(7 - 5, 4) * 3 / 2 * 5 % 2
    );
}
