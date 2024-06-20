pub trait Dimension {
    fn distance(&self, other: &Self) -> f64;

}

#[derive(Debug)]
pub struct TwoDimension {
    pub x: f64,
    pub y: f64,
}

impl Dimension for TwoDimension {
    fn distance(&self, other: &Self) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}