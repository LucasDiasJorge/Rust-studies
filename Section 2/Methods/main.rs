struct Square {
    height: u32,
    width: u32,
}

impl Square {

    // Method to calculate the area of the square
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // Getter for height
    fn my_height(&self) -> u32 {
        self.height
    }

    // Getter for width
    fn my_width(&self) -> u32 {
        self.width
    }

    // Method to change height
    fn change_height(&mut self, height: u32) {
        self.height = height;
    }

    // Method to change width
    fn change_width(&mut self, width: u32) {
        self.width = width;
    }

}

fn main() {

    let mut sq = Square { height: 10, width: 10 };

    println!("{}", sq.area());        // Output: 100
    println!("{}", sq.my_height());   // Output: 10
    println!("{}", sq.my_width());    // Output: 10

    sq.change_height(20);  // Change the height of the square
    sq.change_width(30);   // Change the width of the square
    println!("{}", sq.area());        // Output: 600
}
