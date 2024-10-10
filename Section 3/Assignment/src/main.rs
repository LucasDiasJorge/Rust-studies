
enum Shape {
    Triangle,
    Squares,
    Pentagon,
    Octagon
}

impl Shape {
    fn corners(&self) -> i8 {
        match self {
            Shape::Triangle => 3,
            Shape::Squares => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8
        }
    }
}

fn main() {

    let shape = Shape::Triangle;
    println!("{}", shape.corners());

    let shape = Shape::Squares;
    println!("{}", shape.corners());

    let shape = Shape::Pentagon;
    println!("{}", shape.corners());

    let shape = Shape::Octagon;
    println!("{}", shape.corners());

}
