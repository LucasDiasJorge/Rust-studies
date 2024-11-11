// Defining a generic structure Point, which can use different types for x and y.
struct Point<T, U> {
    x: T,
    y: U,
}

// Defining the Overview trait with a default implementation for the overview function.
trait Overview {
    fn overview(&self) -> String {
        String::from("default implementation")
    }
}

// Defining the Course struct with the default implementation of the Overview trait.
struct Course {
    headline: String,
    author: String,
}

// Implementing the Overview trait for Course, using the default implementation.
impl Overview for Course {}

// Defining AnotherCourse struct with a custom implementation of overview.
struct AnotherCourse {
    headline: String,
    author: String,
}

// Implementing the Overview trait for AnotherCourse with a custom implementation.
impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}\n{}", self.headline, self.author)
    }
}

// Function to call the overview method on any item that implements the Overview trait.
fn call_overview(item: &impl Overview) {
    println!("Call overview:\n{}", item.overview());
}

fn main() {
    // Creating Point instances with different types for x and y.
    let coord = Point { x: 5.0, y: 5.0 };
    let coord2 = Point { x: "5", y: 5.0 };

    // Creating instances of Course and AnotherCourse.
    let course1 = Course {
        headline: String::from("Headline"),
        author: String::from("Author"),
    };
    let course2 = AnotherCourse {
        headline: String::from("Headline 2"),
        author: String::from("Author 2"),
    };

    // Calling call_overview on both course1 and course2.
    call_overview(&course1);
    call_overview(&course2);
}

// Additional examples:
// fn overview(item1: &impl Overview, item2: &impl Overview) {} // Different types
// fn overview<T: Overview>(item: &T, item2: &T) {} // Same types
// fn overview(item1: &impl Overview + AnotherTrait) {}
// fn overview<T: Overview + AnotherTrait>(item: &T, item2: &T) {}
