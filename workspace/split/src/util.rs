/// A simple pair of `T` values.
#[derive(Debug)]
pub struct Pair<T> {
    pub x: T,
    pub y: T
}

/// Mix a `Pair<T>`, swapping its `x` and `y` values.
pub fn mix<T>(value: Pair<T>) -> Pair<T> {
    Pair {
        x: value.y,
        y: value.x
    }
}

pub type IntPair = Pair<i32>;

struct Unused { }
