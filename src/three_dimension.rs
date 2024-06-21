use crate::two_dimension::*;
use std::ops;

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


type Pt3D = ThreeDimension;

#[derive(Debug)]
pub struct Vec3D {
    pub i: f64,
    pub j: f64,
    pub k: f64,
}


impl Vector for Vec3D {
    fn dot_product(&self, other: &Self) -> f64 {
        self.i * other.i + self.j * other.j + self.k * other.k
    }

    fn cross_product(&self, other: &Self) -> Self {
        /*
        i  j  k
        a1 a2 a3 =>| a2 a3 | * i - | a1 a3 | * j + | a1 a2 | * k
        b1 b2 b3   | b2 b3 |       | b1 b3 |         | b1 b2 |
         */
        let a1 = self.i;
        let a2 = self.j;
        let a3 = self.k;
        let b1 = other.i;
        let b2 = other.j;
        let b3 = other.k;
        Vec3D {
            i: a2 * b3 - a3 * b2,
            j: -(a1 * b3 - a3 * b1),
            k: a1 * b2 - a2 * b1,
        }
    }
}


impl ops::Add<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}


impl ops::Sub<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Vec3D) -> Self::Output {
        Vec3D {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

impl ops::Mul<f64> for Vec3D {
    type Output = Vec2D;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2D {
            i: self.i * scalar,
            j: self.j * scalar,
        }
    }
}

impl ops::Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, scalar: f64) -> Self::Output {
        Vec3D {
            i: self.i / scalar,
            j: self.j / scalar,
            k: self.k / scalar,
        }
    }
}



