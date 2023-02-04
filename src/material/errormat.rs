
use crate::material::{Material, ScatteredResult};
use crate::object::HitRecord;
use crate::ray::Ray;


#[derive(Clone)]
pub struct ErrorMat {

}

impl ErrorMat {
    pub fn new_default() -> ErrorMat {
        ErrorMat { }
    }
}

impl Material for ErrorMat {
    fn scatter(&self, ray: &Ray, hit_record : &HitRecord) -> Option<ScatteredResult> {
        None
    }
}

