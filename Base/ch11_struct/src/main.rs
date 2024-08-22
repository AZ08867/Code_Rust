struct Dog {
    name: String,
    age: u32,
    breed: String,
}

fn main() {
    println!("Hello, struct!");
    // create struct instance
    let mut my_dog = Dog {
        name: String::from("Rufus"),
        age: 3,
        breed: "egg-dog".to_string(),
    };

    // access struct fields
    println!(
        "name: {}\nage: {}\nbreed: {}",
        my_dog.name, my_dog.age, my_dog.breed
    );

    // mutable struct field value

    my_dog.age = 4;
    println!("new age: {}", my_dog.age);
}
