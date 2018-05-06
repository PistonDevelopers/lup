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
