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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is: {:?}", six);
    println!("none is: {:?}", none); // `Option<i32>` does not implement `Display` (required by `{}`): E0277 if without '?'

    let dog = Some(Pet::Dog);

    if let Some(Pet::Dog) = dog {
        println!("Dog found!");
    } else {
        println!("Dog not found");
    }

    let cat = Some(Pet::Cat);

    if let Some(Pet::Dog) = cat {
        println!("Dog found!");
    } else {
        println!("Dog not found");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    let x = 10;

    match x {
        1 | 2 => println!("one or two"),
        _ => println!("anything"), // _ is or operator
    }

    let y = 10;

    match y {
        1..=5 => println!("one to five"), // including 5
        6..=10 => println!("six to ten"), // including 10
        _ => println!("others")
    }

    let z = Some(5);
    let a = 5;

    match z {
        Some(50) => println!("fifty"),
        Some(z) if z == a => println!("The five = five"), // if inside if
        _ => println!("others, run to the hills")
    }

}

fn plus_one (x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),
        None => None,
    }
}