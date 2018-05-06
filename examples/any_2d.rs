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
