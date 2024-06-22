use std::ops;


#[derive(Debug)]
pub struct TwoDimension {
    pub x: f64,
    pub y: f64,
}

pub trait Vector {
    fn dot_product(&self, other: &Self) -> f64;
    fn cross_product(&self, other: &Self) -> Self;
}


pub trait Pt {
    fn distance(&self, other: &Self) -> f64;

    fn midpoint(&self, other: &Self) -> Self;

    fn slope(&self, other: &Self) -> f64;

    fn translate(&self, i: f64, j: f64) -> Self;
}

type Pt2D = TwoDimension;

impl Pt2D {
    pub fn new(x: f64, y: f64) -> Pt2D {
        Pt2D { x, y }
    }
}

impl Pt for Pt2D {
    fn distance(&self, other: &Pt2D) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn midpoint(&self, other: &Pt2D) -> Pt2D {
        Pt2D {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    fn slope(&self, other: &Pt2D) -> f64 {
        (self.y - other.y) / (self.x - other.x)
    }

    fn translate(&self, i: f64, j: f64) -> Pt2D {
        Pt2D {
            x: self.x + i,
            y: self.y + j,
        }
    }
}

#[derive(Debug)]
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

impl ops::Add<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn add(self, rhs: Vec2D) -> Self::Output {
        Vec2D {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
        }
    }
}


impl ops::Sub<Vec2D> for Vec2D {
    type Output = Vec2D;
    fn sub(self, rhs: Vec2D) -> Self::Output {
        Vec2D {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

impl ops::Mul<f64> for Vec2D {
    type Output = Vec2D;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2D {
            i: self.i * scalar,
            j: self.j * scalar,
        }
    }
}

impl ops::Div<f64> for Vec2D {
    type Output = Vec2D;

    fn div(self, scalar: f64) -> Self::Output {
        Vec2D {
            i: self.i / scalar,
            j: self.j / scalar,
        }
    }
}