
use crate::object::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::math::vec3::Color;

pub struct World {
    objects: Vec<Box<dyn Hittable>>
}

impl World {
    pub fn new_default() -> World {
        World { objects: Vec::new() }
    }

    pub fn new(object: Box<dyn Hittable>) -> World {
        let mut objects = Vec::new();
        objects.push(object);
        World { objects } 
    }

    pub fn world_hit(&self, ray: &Ray, weight_min: f64, weight_max: f64) -> Result<HitRecord, ()> {
        let mut closest_so_far = weight_max;

        let mut hit_record: Option<HitRecord> = None;
        for i in 0 .. self.objects.len() {
            let object = self.objects[i].as_ref();
            match object.hit(ray, weight_min, closest_so_far) {
                Ok(record) => {
                    closest_so_far = record.weight;
                    hit_record = Some(record);
                }
                Err(()) => {}
            }
        }

        match hit_record {
            Some(record) => {
                Ok(record)
            }
            None => {
                Err(())
            }
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear_all_objects(&mut self) {
        self.objects.clear();
    }

    pub fn get_sky_color(&self) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}

