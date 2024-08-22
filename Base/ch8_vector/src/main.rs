fn main() {
    println!("Hello, vectors!");
    // construct a mutable vector
    let mut vector = vec![1, 2, 3];
    println!("The vector is: {:?}", vector);

    // add an element to end of the vector
    vector.push(4);
    println!("The vector is: {:?}", vector);

    // remove the last element from the vector
    vector.pop();
    println!("The vector is: {:?}", vector);
}
