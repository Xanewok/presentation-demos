pub mod util;

use util::IntPair;

type FloatPair = util::Pair<f32>;

fn local_helper_function(first: &IntPair, second: &IntPair) -> IntPair{
    IntPair { x: first.y, y: second.x }
}

pub fn do_something_complicated(pair: IntPair) -> IntPair {
    // Let's just pretend we work really hard.
    pair
}
