extern crate split;

#[derive(Debug)]
struct MyBinStruct {}

fn peek_struct(value: &MyBinStruct) {
    println!("{:?}", value);
}

fn main() {
    let pair = split::util::Pair { x: 1, y: 2 };
}