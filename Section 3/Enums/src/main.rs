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
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {

    let dog = Pet::Dog;
    println!("{} \"{}\"",dog.who_am_i(), dog.noise());

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1"))
    };

    let loopack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    
}
