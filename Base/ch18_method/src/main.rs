#[derive(Debug)]
struct Triangle {
    a: f64,
    b: f64,
}

// method `hypotenuse` is defined for `Triangle` struct
// impl -> implementation
impl Triangle {
    fn hypotenuse(&self) -> f64 {
        f64::sqrt(self.a.powi(2) + self.b.powi(2))
    }
}

fn main() {
    println!("Hello, world!");
    let shape = Triangle { a: 3.0, b: 4.0 };
    println!("{:?}", shape);
    println!("{:.2}", shape.hypotenuse());
    println!("---------------------");
    println!("Side a:\t\t{}", shape.a);
    println!("Side b:\t\t{}", shape.b);
    println!("Hypotenuse:\t{}", shape.hypotenuse());
}
