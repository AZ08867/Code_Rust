fn main() {
    println!("Hello, strings!");
    // characters and strings
    let a = 'a';
    println!("{} is type char", a);

    let b = "b"; // &str is a string slice(string literal)
    println!("{} is type &str", b);

    let c = "hello rust";
    println!("{} is also type &str", c);

    // how to display quotation marks
    println!("from doggo says \"book!\".");

    // how to display new lines
    println!("line 1\nline 2\nline 3");

    // how to display tabs
    println!("line 1\tline 2\tline 3");

    // string concatenation
    let s1 = "hello";
    let s2 = "world";
    let s3 = s1.to_string() + " " + s2;
    println!("\"{}\" is type string", s3);
}
