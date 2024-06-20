use crate::two_dimension::Dimension;

#[derive(Debug)]
pub struct ThreeDimension {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Dimension for ThreeDimension {

    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}


