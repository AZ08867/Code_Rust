fn main() {
    println!("Hello, ownership!");
    // initial owner

    let s = String::from("hello");
    // s is now owned by main()
    println!("initial owner: {s}");

    //move ownership
    take_ownership(s);

    // value dropped from initial owner
    // println!("initial owner: {s}");
}

fn take_ownership(s: String) {
    println!("new owner: {s}");
}
