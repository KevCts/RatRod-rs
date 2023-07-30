#[derive(Debug, PartialEq)]
pub struct Node {
    pub x: f64, 
    pub y: f64, 
    pub z: f64 
}

impl Node {
    pub fn new(x : f64, y : f64, z : f64) -> Node {
        Node {
            x,
            y,
            z
        }
    }
}
