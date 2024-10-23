# Rust Day 1 Learning

## 1. Setup Rust
Install Rust by following the official documentation at [rust-lang.org](https://www.rust-lang.org/).

## 2. Variables Declaration
In Rust, variables are immutable by default. To make a variable mutable, you need to use `mut`. You can also specify the type, such as `i32` for a signed 32-bit integer or `u32` for an unsigned 32-bit integer.

Example:
```rust
let mut var_name: i32 = value;  // 'mut' makes the variable mutable
```

- `i` stands for signed integers.
- `u` stands for unsigned integers.
- `mut` makes a variable mutable.

## 3. Functions in Rust
Functions in Rust are declared using the `fn` keyword. Here's an example of an addition function:

```rust
fn addition(x: i32, y: i32) -> i32 {
    x + y
}
```

- `fn` is used to define a function.
- You specify the return type after the `->` symbol.

## 4. Input from User
To get input from the user, you can use Rust's standard input/output libraries. Below is an example program that takes two integer inputs, adds and subtracts them, and prints the result:

```rust
use std::io::{stdin, stdout, Write};  // Import standard I/O library

fn main() {
    print!("Enter Value of X: ");
    stdout().flush().unwrap();  // Clear the console buffer
    let mut x: String = String::new();  // Create a new mutable string
    stdin().read_line(&mut x).unwrap();  // Read input from the user

    print!("Enter Value of Y: ");
    stdout().flush().unwrap();
    let mut y: String = String::new();
    stdin().read_line(&mut y).unwrap();

    // Parse the input strings into integers and call the functions
    let add = addition(x.trim().parse::<i32>().unwrap(), y.trim().parse::<i32>().unwrap());
    let subtract = subtraction(x.trim().parse::<i32>().unwrap(), y.trim().parse::<i32>().unwrap());

    println!("Addition of X and Y is: {}", add);
    println!("Subtraction of X and Y is: {}", subtract);

    if add > subtract {
        println!("Addition is larger than subtraction!");
    }
}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}

fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}
```

### Key Points:
- **`stdin().read_line()`** reads input from the user.
- **`.trim()`** removes any whitespace or newline characters.
- **`.parse::<i32>().unwrap()`** parses the input string into a 32-bit integer and handles potential errors.
- **`stdout().flush()`** is used to clear the output buffer before printing.
