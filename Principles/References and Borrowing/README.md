# Borrowing and References in Rust

This project demonstrates key concepts in Rust: **borrowing**, **references**, and **mutable references**. Rust's ownership system enforces strict rules about how data is accessed and modified, ensuring memory safety.

## Description

In Rust, **borrowing** allows a function to access data without taking ownership. There are two types of references:

- **Immutable References (`&T`)**: Allows read-only access to the data.
- **Mutable References (`&mut T`)**: Allows modification of the data.

Rust enforces the following rules for references:
- You can have either one mutable reference or multiple immutable references at the same time.
- References must always be valid and not cause data races.

This project demonstrates how to use **mutable references** to modify data without transferring ownership.

## Code Example

```rust
fn main() {
    let mut s = String::from("hello");
    change_string(&mut s); // Pass a mutable reference to allow modification
    println!("{}", s); // Since we used a mutable reference, "s" is still accessible here
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world"); // Modify the string by appending ", world"
}
```

### How It Works:

1. **Creating a Mutable String**:
   ```rust
   let mut s = String::from("hello");
   ```
   A mutable string `s` is created.

2. **Borrowing a Mutable Reference**:
   ```rust
   change_string(&mut s);
   ```
   We pass a **mutable reference** of `s` to the function `change_string`. This allows the function to modify `s` without taking ownership.

3. **Modifying the Borrowed Value**:
   ```rust
   fn change_string(some_string: &mut String) {
       some_string.push_str(", world");
   }
   ```
   Inside the function, `some_string` (which is a mutable reference to `s`) is modified by appending ", world".

4. **Accessing the Modified Value**:
   ```rust
   println!("{}", s);
   ```
   After the modification, we can still access `s` because its ownership was never transferred, only borrowed.

## Concepts

- **Borrowing**: Allowing a function to use a value without taking ownership.
- **References**: Access to a value via a pointer, either immutably (`&T`) or mutably (`&mut T`).
- **Mutable References**: Provide mutable access to data, but only one mutable reference can exist at any time to prevent data races.