# Structs in Rust

This project demonstrates the usage of different types of **structs** in Rust, which are used to group related data together into meaningful types. Structs can have named fields, be tuple-like, or have no fields at all.

## Description

In Rust, **structs** are used to encapsulate related data under a single type. This project highlights three types of structs:

1. **Named Field Structs**: These structs have fields with names for clarity.
2. **Tuple Structs**: Fields are defined by their position rather than names.
3. **Unit Structs**: Structs without any fields, typically used for specific purposes like implementing traits.

### Types of Structs in the Project

1. **Named Field Struct**:  
   The `User` struct contains information about a user, such as their username, email, sign-in count, and whether they are active.
   ```rust
   struct User {
       username: String,
       email: String,
       sign_in_count: u64,
       active: bool,
   }
   ```

2. **Tuple Struct**:  
   The `Coordinates` struct is defined without named fields, where the position of the values matters. This is useful when naming fields isn’t necessary.
   ```rust
   struct Coordinates(i32, i32, i32);
   ```

3. **Unit Struct**:  
   A **Unit Struct** doesn't have any fields. While it's declared in this project, it's not fully utilized. Unit structs are often used for specific cases, such as trait implementations.
   ```rust
   struct UnitStruct;
   ```

## Concepts

- **Named Field Structs**: Useful when each field represents something important and needs to be named for clarity.
- **Tuple Structs**: Efficient when you only need to store values without the need for named fields.
- **Unit Structs**: These are structs with no fields, used in specific situations, such as when implementing traits.