# Structs and Associated Methods in Rust

This project demonstrates how to define a **struct** and implement associated methods using `impl` in Rust. The example focuses on a `Square` struct that encapsulates the properties of a square, and provides methods to calculate the area, retrieve dimensions, and modify the size.

## Description

In Rust, **structs** can have methods associated with them using `impl` blocks. These methods can either:
- Take references to the struct (immutable or mutable) to access or modify its fields.
- Act as utility functions for the struct itself.

This project demonstrates:
1. **Immutable methods**: Methods that only read data from the struct.
2. **Mutable methods**: Methods that modify the fields of the struct.
3. **Calculating values**: A method to calculate the area of the square.

### Key Methods:

- **`area(&self)`**: Calculates the area of the square by multiplying its height and width.
- **`my_height(&self)`**: Returns the current height of the square.
- **`my_width(&self)`**: Returns the current width of the square.
- **`change_height(&mut self, height: u32)`**: Modifies the height of the square.
- **`change_width(&mut self, width: u32)`**: Modifies the width of the square.

### Example Workflow:

1. The `Square` is initialized with a height and width of 10.
2. The `area()` method calculates the area as `10 * 10 = 100`.
3. The height and width are changed using `change_height` and `change_width`.
4. After the changes, the area is recalculated as `20 * 30 = 600`.

## Concepts

- **Structs**: Used to represent data with named fields.
- **Methods**: Functions defined within an `impl` block to associate behavior with structs.
    - Immutable methods (`&self`): Do not modify the struct.
    - Mutable methods (`&mut self`): Can modify the struct's fields.
