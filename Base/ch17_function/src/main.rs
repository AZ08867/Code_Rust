fn main() {
    println!("Hello, function!");
    println!("2 plus 7 is {}", add_int(2, 7));
    println!("3.14 plus 2.71 is {}", add_float(3.14, 2.71));
}

fn add_int(a: i32, b: i32) -> i32 {
    a + b
}

fn add_float(a: f32, b: f32) -> f32 {
    a + b
}
