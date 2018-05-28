extern crate num;

mod core;

use core::Vector2f;
use core::Vector3f;

fn main() {
    let v3 = Vector3f::new(1.0, 2.0, 3.0);
    let v2 = Vector2f::new(1.0, 2.0);

    println!("Luminary");
    println!("{:?}", v3);
    println!("{:?}", v2);
}
