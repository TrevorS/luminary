#[derive(Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x: x, y: y }
    }

    fn get(self, i: usize) -> T {
        assert!(i <= 1);

        match i {
            0 => self.x,
            _ => self.y,
        }
    }
}
