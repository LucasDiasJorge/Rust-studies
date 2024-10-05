# Ownership and Memory Management in Rust

This project demonstrates fundamental concepts of **ownership**, **borrowing**, **stack vs heap memory**, **moves**, **clones**, and **copies** in Rust.

## Description

This Rust project showcases key ownership principles such as:

- **Stack vs Heap Memory Allocation**: Basic types like integers are stored on the stack, while complex types (like `String`) are allocated on the heap.
- **Move Semantics**: Values can be transferred between variables, where Rust enforces strict rules to avoid data races.
- **Cloning**: Expensive but necessary for duplicating heap-allocated data.
- **Copying**: Used for simple types that don’t require deep copying.

## Concepts

- **Ownership**: Ownership rules ensure memory safety without needing a garbage collector.
- **Move**: Transfers ownership of a value from one variable to another.
- **Clone**: Creates a deep copy of a value, especially for heap-allocated data.
- **Copy**: Some types implement the `Copy` trait, allowing them to be duplicated without moving ownership.
- **Stack vs Heap**: Simple types live on the stack; complex types are stored on the heap and must be managed manually.