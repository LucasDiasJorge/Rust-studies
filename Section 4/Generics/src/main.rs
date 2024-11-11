
struct Point<T, U> {
    x: T,
    y: U,
}

trait Overview {
    fn overview(&self) -> String {
        String::from("default implementation")
    }
}

struct Course {
    headline: String,
    author: String
}

impl Overview for Course {
    // default implementation
}

struct AnotherCourse {
    headline: String,
    author: String
}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}\n{}",self.headline, self.author)
    }
}

fn main() {

    let coord = Point{x: 5.0, y: 5.0};
    let coord2 = Point{x: "5", y: 5.0};

    let couser1 = Course{headline: String::from("Headline"), author: String::from("Author")};
    let couser2 = AnotherCourse{headline: String::from("Headline 2"), author: String::from("Author 2")};

    println!("{}",couser1.overview());
    println!("{}",couser2.overview());

}
