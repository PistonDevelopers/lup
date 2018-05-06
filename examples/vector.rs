#[macro_use]
extern crate lup;

use lup::Vector;

fn main() {
    let a = lup!(Vector<[f64; 4]>: i in 0..4 => {i as f64});
    println!("{:?}", a); // Prints `[0.0, 1.0, 2.0, 3.0]`.

    // Create matrix.
    let m = lup!(Vector<[[f64; 4]; 4]>: i in 0..4_usize => {
        lup!(Vector<[f64; 4]>: j in 0..4_usize => {
            i as f64 - j as f64
        })
    });
    println!("{:?}", m);

    // Get orthogonal matrix.
    let m2 = lup!(Vector<[[f64; 4]; 4]>: i by m => {
        lup!(Vector<[f64; 4]>: j by m[i] => {m[j][i]})
    });
    println!("{:?}", m2);
}
