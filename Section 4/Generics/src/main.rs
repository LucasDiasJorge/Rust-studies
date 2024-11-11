
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

    let course1 = Course{headline: String::from("Headline"), author: String::from("Author")};
    let course2 = AnotherCourse{headline: String::from("Headline 2"), author: String::from("Author 2")};

    //println!("{}",course1.overview());
    //println!("{}",course2.overview());

    call_overview(&course1);
    call_overview(&course2);

}

fn call_overview(item: &impl Overview) {
    println!("Call overview:\n {}", item.overview());
}

// fn overview(item1: &impl Overview, item2: &impl Overview) {} Different types
// fn overview<T: Overview>(item: &T, item2: &T) {} Same types