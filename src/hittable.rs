use super::lib::{Vec3, Point3, dot};
use super::ray::{Ray};
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
  p: Point3,
  normal: Vec3,
  t: f64,
  front_face: bool
}

impl HitRecord {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    self.front_face = dot(&r.direction(), outward_normal) < 0.0;
    if self.front_face {
      self.normal = outward_normal.clone();
    } else {
      self.normal = -1.0 * outward_normal.clone();
    }
  }
  pub fn get_t(&self) -> f64 {
    return self.t;
  }
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}