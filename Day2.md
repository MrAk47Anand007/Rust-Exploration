

# Rust Day 2 Learning

## 1. Loops in Rust

### For Loop
In Rust, the `for` loop is used to iterate over a range of numbers. The syntax is as follows:
```rust
for n in 1..num {
    if n % 2 == 0 {
        println!("{}", n);
    }
}
```
- `1..num` specifies the range, where `1` is the start and `num` is the end (exclusive).
- The loop prints even numbers between `1` and `num`.

### While Loop
The `while` loop in Rust works similarly to other languages. Here's an example that prints even numbers between `num` and 100:
```rust
while num < 100 {
    if num % 2 == 0 {
        println!("{}", num);
    }
    num += 1;
}
```

## 2. Option, Some(), and None
In Rust, `Option` is used to handle values that may or may not be present. It can be `Some(value)` when the value exists or `None` when it doesn't. This is especially useful for error handling or returning optional values.

Example:
```rust
fn find_number(value: i32) -> Option<i32> {
    if value > 0 {
        Some(value)
    } else {
        None
    }
}
```

## 3. Pattern Matching with match
The `match` keyword is used for control flow based on pattern matching. It helps exhaustively check all possible cases and can return values based on conditions.

Example usage:
```rust
let result = find_number(num);

match result {
    Some(n) => println!("Number found: {}", n),
    None => println!("Not found"),
}
```
- `Some(n)` matches if the value is present and prints it.
- `None` matches if the value is not found.

### Key Points:
- **`Some()`** wraps a value if it exists.
- **`None`** is used when there is no value.
- **`match`** is used for pattern matching to handle multiple cases in a clean and exhaustive way.
