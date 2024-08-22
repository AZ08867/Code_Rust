fn main() {
    println!("Hello, loop!");
    let mut counter = 1;

    // infinite loop
    loop {
        //task
        println!("{counter}");
        counter += 2;
        // condition to break the loop
        if counter >= 10 {
            break;
        }
    }
}
