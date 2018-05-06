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
