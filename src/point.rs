pub struct Point {
    pub x: f64,
    pub y: f64,
    id: u32,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, id: 0 }
    }
}
