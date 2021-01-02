use std::rc::Rc;
use super::hittable::{Hittable, HitRecord};
use super::ray::{Ray};

pub struct HittableList {
  objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
  pub fn new() -> Self {
    HittableList {
      objects: Vec::new()
    }
  }
  pub fn clear(&mut self) {
    self.objects.clear();
  }
  pub fn add(&mut self, object: Rc<dyn Hittable>) {
    self.objects.push(object);
  }
}

impl Hittable for HittableList {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let mut temp_rec: HitRecord = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;
    for object in self.objects.iter() {
      if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.get_t();
        rec.set_t(temp_rec.get_t());
        rec.set_p(temp_rec.get_p());
        rec.set_front_face(temp_rec.get_front_face());
        rec.set_normal(&temp_rec.get_normal());

      }
    }
    return hit_anything;
  }
} 