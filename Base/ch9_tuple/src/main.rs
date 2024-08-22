// {:?} -> (display in row)
// {:#?} -> (display in column)

fn main() {
    println!("Hello, Tuple!");
    // construct mutable tuple
    let mut tuple = ("Rufus", 12, true);
    println!("Tuple: {:#?}", tuple);
    println!("{:?}\nis a mutable tuple", tuple);

    println!("----------1-----------");
    // access tuple elements
    println!("Name: {}", tuple.0);
    println!("Age: {}", tuple.1);
    println!("Is handsome: {}", tuple.2);

    // mutate tuple elements
    tuple.2 = false;
    println!("Updated tuple: {:?}", tuple);
    println!("Is new handsome: {}", tuple.2);

    // using variables to access tuple elements
    let (name, age, is_handsome) = tuple;
    println!("--------------2---------------");

    println!("Name: {}\nAge: {}\nIs handsome: {}", name, age, is_handsome);
    println!("--------------3----------------");

    println!("{name}\n{age}\n{is_handsome}");
}
