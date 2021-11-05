
use std::ops::Index;
use std::ops::Neg;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

pub type Color = Vec3;

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    // constructor
    pub fn default() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 } 
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // vector functions
    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
        Vec3 { 
            x: lhs.y * rhs.z - lhs.z * rhs.y, 
            y: lhs.z * rhs.x - lhs.x * rhs.z, 
            z: lhs.x * rhs.y - lhs.y * rhs.x }
    }

    pub fn length(&self) -> f64 {
        self.sqaure_length().sqrt()
    }

    pub fn sqaure_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        (*self) /= self.length();
    }

    // color functions
    pub fn write_color(&self) {
        let r = (255.999 * self.x) as i32;
        let g = (255.999 * self.y) as i32;
        let b = (255.999 * self.z) as i32;
        println!("{} {} {}", r, g, b);
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out range vec3!")
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z }
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z }
    }
}

impl SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.x * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3 { 
            x: self.x * rhs.x, 
            y: self.y * rhs.y, 
            z: self.z * rhs.z }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<&Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: &Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 { 
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

