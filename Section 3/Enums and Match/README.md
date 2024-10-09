# Enums and Match Statements

This project is a basic Rust program that explores the use of `enum` types and `match` statements, two powerful features in Rust that allow for expressive and flexible control flow. The code includes two enums, `Pet` and `IpAddrKind`, and demonstrates how to use the `match` statement to handle different cases based on the enum's variants.

## Code Overview

### Enums and Variants

#### `Pet` Enum
The `Pet` enum represents different types of pets and includes five variants: `Cat`, `Dog`, `Bird`, `Fish`, and `Hamster`. Each pet has an associated noise and a descriptive string:
```rust
enum Pet {
    Cat,
    Dog,
    Bird,
    Fish,
    Hamster
}
```

#### `IpAddrKind` Enum
The `IpAddrKind` enum represents types of IP addresses and includes two variants:
- `V4`: Holds a `String` representing an IPv4 address.
- `V6`: Represents an IPv6 address but doesn't hold any additional data in this example.

```rust
enum IpAddrKind {
    V4(String),
    V6
}
```

### Implementing Methods for Enums

#### `Pet` Methods
The `Pet` enum has two methods: `noise` and `who_am_i`. Both use the `match` statement to handle each variant and return specific strings:
- `noise`: Returns the sound associated with each pet.
- `who_am_i`: Returns a string that describes the pet.

```rust
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
```

### Main Function

The `main` function demonstrates the usage of the `Pet` and `IpAddr` enums:
- It creates an instance of `Pet::Dog`, then prints its identity and noise.
- It also creates instances of `IpAddr`, one for IPv4 and another for IPv6, showcasing how data can be attached to an enum variant.

```rust
fn main() {
    let dog = Pet::Dog;
    println!("{} \"{}\"", dog.who_am_i(), dog.noise());

    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1"))
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

## Concepts Explained

### Enums
In Rust, `enum`s allow you to define a type that can be one of several variants. Each variant can hold data, making `enum`s versatile and useful for representing different types of data under a single type.

### `match` Statement
The `match` statement is used to control flow by matching a value against different patterns. It ensures that all possible cases are handled, either explicitly or by using a wildcard `_`.

### Using `Option` for Nullable Values

In Rust, the `Option` enum is a powerful tool for representing values that may or may not be present. It helps avoid common issues associated with `null` values in other programming languages by explicitly handling the presence or absence of a value. `Option` has two variants:

- `Some(value)`: Indicates the presence of a value.
- `None`: Indicates the absence of a value.

`Option` makes it clear which values are available and which might be absent, encouraging safer handling of potentially null data. Rust's `Option` enum, combined with `match` statements, offers a reliable way to deal with nullable values in a manner that’s both explicit and concise.