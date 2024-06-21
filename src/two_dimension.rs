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

pub trait Vector {
    fn dot_product(&self, other: &Self) -> f64;
    fn cross_product(&self, other: &Self) -> Self;
}

type Pt2D = TwoDimension;

pub struct Vec2D {
    pub i: f64,
    pub j: f64,
}

impl Vector for Vec2D {
    fn dot_product(&self, other: &Self) -> f64 {
        self.i * other.i + self.j * other.j
    }

    fn cross_product(&self, other: &Self) -> Self {
        Vec2D {
            i: self.j * other.i - self.i * other.j,
            j: self.i * other.j - self.j * other.i,
        }
    }
}