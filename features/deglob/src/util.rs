/// A simple pair of `T` values.
#[derive(Debug)]
pub struct Pair<T> {
    pub x: T,
    pub y: T
}

/// Is able to `mix()` itself.
pub trait Mix {
    fn mix(self) -> Self;
}

impl<T> Mix for Pair<T> {
    fn mix(self) -> Pair<T> {
        Pair {
            x: self.y,
            y: self.x
        }
    }
}

/// A newtype wrapper around `i32` that implements `Mix`.
pub struct Mixable32(i32);

impl Mix for Mixable32 {
    fn mix(self) -> Mixable32 {
        let value = -(self.0);

        Mixable32(value)
    }
}