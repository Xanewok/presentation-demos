use util::*; // Let's be more explicit about it

mod util;

type IntPair = Pair<i32>;

fn greet_pair(pair: &IntPair) {
    println!("Hello, {:?}!", pair);
}

fn main() {
    let pair = IntPair { x: 1, y: 2 };
    let mixed = pair.mix();
    greet_pair(&mixed);
}
