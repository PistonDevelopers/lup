# Lup ("loop")

A custom indexed loop macro library for Rust.

This library offers a different pattern than iterators based on indexed loops.
Index loops are very common in game programming and scientific computing.
The loops included in the library reflects the ones used in [Dyon](https://github.com/pistondevelopers/dyon).

You can create your own custom loops by implementing the `Lup` trait.

### Motivation

- Closer to mathematical notation than the iterator pattern
- Reduces chance of logical errors when solving complex problems
- Supports `continue`, `break` or `return` from within the loop
- Packed notation for nested loops, `i in 0..3, j in 0..3 =>`
- Short-hand notation with safe range, `i, j by list =>`
- Automatically infers evidence type of secrets (the ones used in [Dyon](https://github.com/pistondevelopers/dyon)
- Can be used for numeric brute-force theorem proving
- Ergononmic when working with vectors up to 4 dimensions

### Usage

The loops `Any`, `All`, `Max` and `Min` uses the `Secret` structure.
This type contains evidence that justifies the value.

Here is a simple example using the `Any` loop and the `by` syntax:

```rust
#[macro_use]
extern crate lup;

use lup::Any;

fn main() {
    let words = vec!["mary", "had", "a", "little", "lamb"];
    println!("Words:");
    println!("{:?}\n", words);

    // Find the word "lamb".
    let lamb = lup!(Any<_>: i by words => {words[i] == "lamb"});
    println!("Is there any word `lamb`?");
    println!("{}\n", lamb.value); // Prints `true`.

    println!("What is the evidence?");
    println!("{:?}\n", lamb.evidence); // Prints `Some(4)`.

    println!("Using the evidence to find the word:");
    println!("{}\n", words[lamb.evidence.unwrap()]); // Prints `lamb`.
}
```

The `by` syntax uses the range from the list.
In the example above, this is the same as:

```ignore
lup!(Any<_>: i in 0..words.len() => {words[i] == "lamb"})
```

Here is a slightly more complex example:

```rust
#[macro_use]
extern crate lup;

use lup::Any;

fn main() {
    // Create a 2D array.
    let arr = [[1, 2], [3, 4]];

    // Look for a number greater than 2.
    let b = lup!(Any<_>: i, j by arr => {arr[i][j] > 2});

    println!("{:?}", b.evidence); // Prints `Some((1, 0))`.

    // Get the number that is greater than 2.
    let ev = b.evidence.unwrap();
    println!("{}", arr[ev.0][ev.1]); // Prints `3`.
}
```

In the example above, there are two loops, one with index `i` and one with index `j`.

The type of the result of the inner `j` loop is `Secret<(usize), bool>`.

The type of the result of the outer `i` loop is `Secret<(usize, usize), bool>`.

The `Secret` type is used to combine results from loops to give meaningful answers.

Another example using the `Max` loop:

```rust
#[macro_use]
extern crate lup;

use lup::Max;

fn main() {
    let data = vec![
        (1, 1),
        (2, 2),
        (3, 4),
        (4, 4)
    ];

    let a = lup!(Max<_, _>: i by data => {data[i].0 as f32});
    let b = lup!(Max<_, _>: i by data => {data[i].1 as f32});

    println!("{} vs {}", a.value, b.value); // Prints `4 vs 4`.
    println!("{:?} vs {:?}", a.evidence, b.evidence); // Prints `Some(3) vs Some(2)`.
}
```

We convert to `f32` since `Max` is implemented only for `f32` and `f64`.

The evidence points to the item that first achieves maximum value.

Here is an example that demonstrates the full power of secrets:

```rust
#[macro_use]
extern crate lup;

use lup::{Any, Max};

fn main() {
    let data = vec![
        vec![1, 2, 6, 4, 5, 3],
        vec![4, 6, 9, 3, 2, 1],
    ];

    // Find out whether any list has a maximum value less than 7.
    let search = lup!(Any<_>: i by data => {
        lup!(Max<_, _>: j by data[i] => {data[i][j] as f32}).le(&7.0)
    });

    println!("{}", search.value); // Prints `true`.
    println!("{:?}", search.evidence); // Prints `Some((0, 2))`.
    let ev = search.evidence.unwrap();
    println!("{}", data[ev.0][ev.1]); // Prints `6`.
}
```

How do we know that a list has a maximum value less than 7?
We know it because the maximum value of the first list is 6!

Secrets provide us with a meaningful reason why something is true.

In the example above, the secret of the `Max` loop propagates to the `Any` loop.

The example calls `.le` instead of using `<=` because Rust does not allow
overriding the return value of comparison operators.
