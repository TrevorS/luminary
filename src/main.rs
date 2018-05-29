extern crate num;

mod core;

use core::Vector2i;
use core::Vector3f;

fn main() {
    let v3 = Vector3f::new(1.0, 2.0, 3.0);
    let v2 = Vector2i::new(1, 2);

    println!("Luminary");
    println!("{:?}", v3);
    println!("{:?}", v2);
}
