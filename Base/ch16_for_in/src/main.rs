fn main() {
    println!("Hello, for_in!");
    // loop over array
    let my_array = [10, 20, 30, 40, 50];

    for element in my_array {
        println!("Element: {}", element);
    }

    println!("--------------");
    // loop over string literal
    let my_string = "Hello, world!";
    for character in my_string.chars() {
        println!("Character: {}", character);
    }
}
