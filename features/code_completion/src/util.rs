use std::fmt::Debug;

/// A simple pair of `T` values.
#[derive(Debug)]
pub struct Pair<T> {
    pub x: T,
    pub y: T
}

impl<T> Pair<T> {
    pub fn say_hi(&self) where T: Debug {
        println!("{:?} says hi!", self);
    }
}

/// Mix a `Pair<T>`, swapping its `x` and `y` values.
pub fn mix<T>(value: Pair<T>) -> Pair<T> {
    Pair {
        x: value.y,
        y: value.x
    }
}
