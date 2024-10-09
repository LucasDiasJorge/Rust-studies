enum Pet {
    Cat,
    Dog,
    Bird,
    Fish,
    Hamster
}

impl Pet {
    fn noise(&self) -> &'static str {
        match self {
            Pet::Cat => "Meow",
            Pet::Dog => "Woof",
            Pet::Bird => "Tweet",
            Pet::Fish => "Blub",
            Pet::Hamster => "Squeak"
        }
    }

    fn who_am_i(&self) -> &'static str {
        match self {
            Pet::Cat => "I am a cat",
            Pet::Dog => "I am a dog",
            Pet::Bird => "I am a bird",
            Pet::Fish => "I am a fish",
            Pet::Hamster => "I am a hamster"
        }
    }
}

enum IpAddrKind {
    V4(String),
    V6(Option<String>)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {

    let dog = Pet::Dog;
    println!("{} \"{}\"", dog.who_am_i(), dog.noise());

    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddrKind::V6(Some(String::from("::1")));
    let unspecified_v6 = IpAddrKind::V6(None);

    // Fixing definition, Like Java optional: Some() value present, None isn't
    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None;

    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("Sum is: {}", sum);

    let x: Option<i32> = Some(5);
    match x {
        Some(v) => println!("Value present: {}", v),
        None => println!("No value present"),
    }


}

