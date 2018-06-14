use core::transform::Transform;

pub trait Transformable {
    fn transform(self, t: Transform) -> Self;
}
