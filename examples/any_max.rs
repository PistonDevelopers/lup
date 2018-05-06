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
