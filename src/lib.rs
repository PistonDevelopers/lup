//! # Lup ("loop")
//!
//! A custom indexed loop macro library for Rust.
//!
//! This library offers a different pattern than iterators based on indexed loops.
//! Index loops are very common in game programming and scientific computing.
//! The loops included in the library reflects the ones used in [Dyon](https://github.com/pistondevelopers/dyon).
//!
//! You can create your own custom loops by implementing the `Lup` trait.
//!
//! ### Motivation
//!
//! - Closer to mathematical notation than the iterator pattern
//! - Reduces chance of logical errors when solving complex problems
//! - Supports `continue`, `break` or `return` from within the loop
//! - Packed notation for nested loops, `i in 0..3, j in 0..3 =>`
//! - Short-hand notation with safe range, `i, j by list =>`
//! - Automatically infers evidence type of secrets
//! - Can be used for numeric brute-force theorem proving
//! - Ergononmic when working with vectors up to 4 dimensions
//!
//! ### Usage
//!
//! The loops `Any`, `All`, `Max` and `Min` uses the `Secret` structure.
//! This type contains evidence that justifies the value.
//!
//! Here is a simple example using the `Any` loop and the `by` syntax:
//!
//! ```rust
//! #[macro_use]
//! extern crate lup;
//!
//! use lup::Any;
//!
//! fn main() {
//!     let words = vec!["mary", "had", "a", "little", "lamb"];
//!     println!("Words:");
//!     println!("{:?}\n", words);
//!
//!     // Find the word "lamb".
//!     let lamb = lup!(Any<_>: i by words => {words[i] == "lamb"});
//!     println!("Is there any word `lamb`?");
//!     println!("{}\n", lamb.value); // Prints `true`.
//!
//!     println!("What is the evidence?");
//!     println!("{:?}\n", lamb.evidence); // Prints `Some(4)`.
//!
//!     println!("Using the evidence to find the word:");
//!     println!("{}\n", words[lamb.evidence.unwrap()]); // Prints `lamb`.
//! }
//! ```
//!
//! The `by` syntax uses the range from the list.
//! In the example above, this is the same as:
//!
//! ```ignore
//! lup!(Any<_>: i in 0..words.len() => {words[i] == "lamb"})
//! ```
//!
//! Here is a slightly more complex example:
//!
//! ```rust
//! #[macro_use]
//! extern crate lup;
//!
//! use lup::Any;
//!
//! fn main() {
//!     // Create a 2D array.
//!     let arr = [[1, 2], [3, 4]];
//!
//!     // Look for a number greater than 2.
//!     let b = lup!(Any<_>: i, j by arr => {arr[i][j] > 2});
//!
//!     println!("{:?}", b.evidence); // Prints `Some((1, 0))`.
//!
//!     // Get the number that is greater than 2.
//!     let ev = b.evidence.unwrap();
//!     println!("{}", arr[ev.0][ev.1]); // Prints `3`.
//! }
//! ```
//!
//! In the example above, there are two loops, one with index `i` and one with index `j`.
//!
//! The type of the result of the inner `j` loop is `Secret<(usize), bool>`.
//!
//! The type of the result of the outer `i` loop is `Secret<(usize, usize), bool>`.
//!
//! The `Secret` type is used to combine results from loops to give meaningful answers.
//!
//! Another example using the `Max` loop:
//!
//! ```rust
//! #[macro_use]
//! extern crate lup;
//!
//! use lup::Max;
//!
//! fn main() {
//!     let data = vec![
//!         (1, 1),
//!         (2, 2),
//!         (3, 4),
//!         (4, 4)
//!     ];
//!
//!     let a = lup!(Max<_, _>: i by data => {data[i].0 as f32});
//!     let b = lup!(Max<_, _>: i by data => {data[i].1 as f32});
//!
//!     println!("{} vs {}", a.value, b.value); // Prints `4 vs 4`.
//!     println!("{:?} vs {:?}", a.evidence, b.evidence); // Prints `Some(3) vs Some(2)`.
//! }
//! ```
//!
//! We convert to `f32` since `Max` is implemented only for `f32` and `f64`.
//!
//! The evidence points to the item that first achieves maximum value.
//!
//! Here is an example that demonstrates the full power of secrets:
//!
//! ```rust
//! #[macro_use]
//! extern crate lup;
//!
//! use lup::{Any, Max};
//!
//! fn main() {
//!     let data = vec![
//!         vec![1, 2, 6, 4, 5, 3],
//!         vec![4, 6, 9, 3, 2, 1],
//!     ];
//!
//!     // Find out whether any list has a maximum value less than 7.
//!     let search = lup!(Any<_>: i by data => {
//!         lup!(Max<_, _>: j by data[i] => {data[i][j] as f32}).le(&7.0)
//!     });
//!
//!     println!("{}", search.value); // Prints `true`.
//!     println!("{:?}", search.evidence); // Prints `Some((0, 2))`.
//!     let ev = search.evidence.unwrap();
//!     println!("{}", data[ev.0][ev.1]); // Prints `6`.
//! }
//! ```
//!
//! How do we know that a list has a maximum value less than 7?
//! We know it because the maximum value of the first list is 6!
//!
//! Secrets provide us with a meaningful reason why something is true.
//!
//! In the example above, the secret of the `Max` loop propagates to the `Any` loop.
//!
//! The example calls `.le` instead of using `<=` because Rust does not allow
//! overriding the return value of comparison operators.


#![deny(missing_docs)]

pub use sum::Sum;
pub use prod::Prod;
pub use any::Any;
pub use all::All;
pub use max::Max;
pub use min::Min;
pub use vector::Vector;
pub use sift::Sift;
pub use secret::Secret;

mod sum;
mod prod;
mod any;
mod all;
mod max;
mod min;
mod vector;
mod sift;
mod secret;

/// Implemented by custom loops.
pub trait Lup<I, T> {
    /// The resulting type.
    type Inner;
    /// Initialize loop.
    fn start() -> Self;
    /// Iterate loop.
    fn it(&mut self, I, T) -> bool;
    /// Unwrap the resulting value.
    fn unwrap(self) -> Self::Inner;
}

#[macro_export]
macro_rules! lup(
    ($sum:ty : $i:tt , $($j:tt),+ by $list:expr => $body:block) => {
        lup!($sum : $i in 0..$list.len() => {
            lup!($sum : $($j),* by $list[$i] => $body)
        })
    };
    ($sum:ty : $i:tt in $iter:expr , $($j:tt in $iter2:expr),+ => $body:block) => {
        lup!($sum : $i in $iter => {
            lup!($sum : $($j in $iter2),* => $body)
        })
    };
    ($sum:ty : $i:tt by $list:expr => $body:block) => {
        lup!($sum : $i in 0..$list.len() => $body)
    };
    ($sum:ty : $i:tt in $iter:expr => $body:block) => {{
        use $crate::Lup;
        let mut sum: $sum = Lup::start();
        let mut iter = $iter;
        while let Some($i) = iter.next() {
            if !sum.it($i, $body) {break};
        }
        sum.unwrap()
    }}
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list() {
        type Num = f32;
        let list = vec![vec![vec![1.0, 2.0, 3.0]]];

        let sum = lup!(Sum<Num>:
            i in 0..list.len(),
            j in 0..list[i].len(),
            k in 0..list[i][j].len() => {
                list[i][j][k]
            });
        assert_eq!(sum, 6.0);

        let prod = lup!(Prod<Num>:
            i in 0..list.len(),
            j in 0..list[i].len(),
            k in 0..list[i][j].len() => {
                list[i][j][k]
            });
        assert_eq!(prod, 6.0);

        let any = lup!(Any<_>: i in 0..list[0][0].len() => {
            list[0][0][i] > 2.0
        });
        assert_eq!(any.evidence, Some(2));

        let any = lup!(Any<_>:
            i in 0..list.len(),
            j in 0..list[i].len(),
            k in 0..list[i][j].len() => {
                list[i][j][k] > 2.0
            });
        assert_eq!(any.evidence, Some((0, 0, 2)));

        let all = lup!(All<_>:
            i in 0..list.len(),
            j in 0..list[i].len(),
            k in 0..list[i][j].len() => {
                list[i][j][k] < 3.0
            });
        assert_eq!(all.evidence, Some((0, 0, 2)));

        // Find a list where not all items are less than 3.
        let comb = lup!(Any<_>:
            i in 0..list.len(),
            j in 0..list[i].len() => {
                !lup!(All<_>: k in 0..list[i][j].len() => {
                    list[i][j][k] < 3.0
                })
            });
        assert_eq!(comb.value, true);
        assert_eq!(comb.evidence, Some((0, 0, 2)));

        let max = lup!(Max<_, Num>:
            i in 0..list.len(),
            j in 0..list[i].len(),
            k in 0..list[i][j].len() => {
                list[i][j][k]
            });
        assert_eq!(max.value, 3.0);
        assert_eq!(max.evidence, Some((0, 0, 2)));

        let min = lup!(Min<_, Num>:
            i in 0..list.len(),
            j in 0..list[i].len(),
            k in 0..list[i][j].len() => {
                list[i][j][k]
            });
        assert_eq!(min.value, 1.0);
        assert_eq!(min.evidence, Some((0, 0, 0)));

        // Find a list which maximum value is equal to 3.
        let comb = lup!(Any<_>:
            i in 0..list.len(),
            j in 0..list[i].len() => {
                lup!(Max<_, f32>: k in 0..list[i][j].len() => {
                    list[i][j][k]
                }).eq(&3.0)
            });
        assert_eq!(comb.value, true);
        assert_eq!(comb.evidence, Some((0, 0, 2)));
    }

    #[test]
    fn vector() {
        let list = vec![[0.2, 0.3, 0.4], [0.1, 0.5, 0.7]];

        let sum = lup!(Sum<[f32; 3]>: i in 0..list.len() => {list[i]});
        assert_eq!(sum, [0.3, 0.8, 1.1]);

        let prod = lup!(Prod<[f32; 3]>: i in 0..list.len() => {list[i]});
        assert_eq!(prod, [0.020000001, 0.15, 0.28]);

        let v = lup!(Vector<[f32; 4]>: i in 0..3 => {list[0][i] + list[1][i]});
        assert_eq!(v, [0.3, 0.8, 1.1, 0.0]);
    }

    #[test]
    fn sift() {
        let list = lup!(Sift<Vec<f32>>: i in 0..5 => {i as f32 + 1.0});
        assert_eq!(list, vec![1.0, 2.0, 3.0, 4.0, 5.0]);

        let list = lup!(Sift<_>:
            i in 0..3,
            j in 0..3 => {
                i as f32 + j as f32
            });
        assert_eq!(list, vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 2.0, 3.0],
            vec![2.0, 3.0, 4.0],
        ]);

        let symmetric = lup!(All<_>:
            i in 0..3,
            j in 0..3 => {
                list[i][j] == list[j][i]
            });
        assert_eq!(symmetric.value, true);
    }

    #[test]
    fn short() {
        let list = lup!(Sift<_>: i in 0..3, j in 0..3, k in 0..3 => {
            i as f32 + j as f32 + k as f32
        });
        let a = lup!(Sum<_>: i, j, k by list => {list[i][j][k]});
        assert_eq!(a, 81.0);
    }
}
