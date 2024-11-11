
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {

    let coord = Point{x: 5.0, y: 5.0};
    let coord2 = Point{x: "5", y: 5.0};

}
