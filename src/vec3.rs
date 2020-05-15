use std::ops::{Add, Mul, Div, AddAssign, Sub, Neg, DivAssign};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a> Add<&'a Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a> Sub<&'a Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &'a Vec3) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl <'a> DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }
}

impl Vec3 {
    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }
}

impl Vec3 {
    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.x * rhs.z - lhs.z * rhs.x,
            z: lhs.x * rhs.y - lhs.y * rhs.x,
        }
    }
}

impl Vec3 {
    pub fn ppm_color(&self) -> String {
        let r = self.x;
        let g = self.y;
        let b = self.z;
        // Divide the color total by the number of samples and gamma-correct for gamma=2.0.
        let r = r.sqrt();
        let g = g.sqrt();
        let b = b.sqrt();
        format!("{} {} {}\n",
                (255.999 * r.max(0.).min(0.999)) as i32,
                (255.999 * g.max(0.).min(0.999)) as i32,
                (255.999 * b.max(0.).min(0.999)) as i32)
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
