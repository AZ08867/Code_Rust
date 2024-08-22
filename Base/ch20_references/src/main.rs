fn main() {
    println!("Hello, references or borrowing!");
    // initial owner
    let s = String::from("hello");

    println!("initial owner: {s}");

    //references and borrowing
    references(&s);

    // initial owner still owns value
    println!("final owner: {s}");
}

fn references(s: &String) {
    println!("borrower:{s}");
}
