fn main() {
    // comparison operators
    println!("1 == 2: {}", 1 == 2);
    println!("1!= 2: {}", 1 != 2);
    println!("1 < 2: {}", 1 < 2);
    println!("1 > 2: {}", 1 > 2);
    println!("1 <= 2: {}", 1 <= 2);
    println!("1 >= 2: {}", 1 >= 2);

    // logical operators
    println!("true && false: {}", true && false);

    // bitwise operators
    println!("1 & 2: {}", 1 & 2);
    println!("1 | 2: {}", 1 | 2);
    println!("1 ^ 2: {}", 1 ^ 2);
    println!("!1: {}", !1);
    println!("1 << 2: {}", 1 << 2);
    println!("1 >> 2: {}", 1 >> 2);

    // assignment operators
    let mut x = 1;
    x += 1;
    println!("x: {}", x);
    x -= 1;
    println!("x: {}", x);
    x *= 2;
    println!("x: {}", x);
    x /= 2;
    println!("x: {}", x);
    x %= 2;
    println!("x: {}", x);
    println!("Hello, world!");

    // && Lazy Boolean AND operator
    println!("1 == 1 && 1 > 1 is {}", 1 == 1 && 1 > 1);

    // || Lazy Boolean OR operator
    println!("1 == 1 || 1 > 1 is {}", 1 == 1 || 1 > 1);
}
