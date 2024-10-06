
struct Car {
    mpg: f32,
    color: String,
    top_speed: i16
}

impl Car{

    fn new(mpg: f32, color: String, top_speed: i16) -> Car {
        Car {mpg: mpg, color, top_speed}
    }

    fn new_empty() -> Car {
        Car {mpg: 0.0, color: "White".to_string(), top_speed: 0}
    }

    fn print(&self){
        println!("Car: \n{} mpg \n{} color \n{} top speed", self.mpg, self.color, self.top_speed);
    }

    fn set_mpg(&mut self, mpg: f32){
        self.mpg = mpg;
    }

    fn set_color(&mut self, color: String){
        self.color = color;
    }

    fn set_top_speed(&mut self, top_speed: i16){
        self.top_speed = top_speed;
    }
}

fn main() {

    let mut jetta: Car = Car::new_empty();
    jetta.print();

    jetta.set_mpg(35.5);
    jetta.set_color(String::from("Gray w/ red details"));
    jetta.set_top_speed(250);
    jetta.print();
}
