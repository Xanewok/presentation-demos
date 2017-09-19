use util::{mix, Pair}; // What are those, again?

mod util;

type IntPair = Pair<i32>;

fn greet_pair(pair: &IntPair) {
    println!("Hello, {:?}!", pair);
}

fn main() {
    println!("Hello, world!");
}
