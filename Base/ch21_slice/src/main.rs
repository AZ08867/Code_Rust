fn main() {
    println!("Hello, slice!");
    // string slice
    let s = String::from("hello world");

    let slice1 = &s[0..5];
    let slice2 = &s[6..11];

    println!("slice1: {slice1}\nslice2: {slice2}");

    //array slice
    let arr = [1, 2, 3, 4, 5];

    let slice3 = &arr[0..2];
    let slice4 = &arr[3..];

    println!("slice3: {:?}\nslice4: {:?}", slice3, slice4);
}
