
use std::f64::consts::PI;
use std::ops::{
    Index, Neg,
    Add, AddAssign,
    Sub, SubAssign,
    Mul, MulAssign, 
    Div, DivAssign
};
use std::clone::Clone;
use rand::{Rng, random};

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    // constructor
    pub fn new_default() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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

    pub fn rand() -> Vec3 {
        Vec3::rand_range((0.0, 1.0))
    }

    pub fn rand_range(range: (f64, f64)) -> Vec3 {
        Vec3 { 
            x: rand::thread_rng().gen_range(range.0 .. range.1), 
            y: rand::thread_rng().gen_range(range.0 .. range.1), 
            z: rand::thread_rng().gen_range(range.0 .. range.1) 
        }
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let result = Vec3::rand_range((-1.0, 1.0));
            if result.sqaure_length() <= 1.0 {
                return result;
            }
        }
    }

    pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::rand_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return in_unit_sphere;
        }
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

    pub fn get_normal(&self) -> Vec3 {
        (*self) / self.length()
    }

    pub fn set_from_index(&mut self, index: usize, value: f64) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!("out range vec3!")
        }
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
    type Output = Vec3;

    fn neg(mut self) -> Vec3 {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

// Add implementation
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
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

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f64> for Vec3 {
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


