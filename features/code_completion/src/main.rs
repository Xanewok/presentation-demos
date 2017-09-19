use util::{mix, Pair};

mod util;

type IntPair = Pair<i32>;

fn greet_pair(pair: &IntPair) {
    println!("Hello, {:?}!", pair);
}

fn main() {
    let pair = IntPair { x: 1, y: 2 };
    let mixed = // TODO: Mix it up
    // TODO: Be polite and greet it
}
