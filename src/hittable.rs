use super::lib::{Vec3, Point3};
use super::ray::{Ray};
mod lib;

pub struct HitRecord {
  p: Point3,
  normal: Vec3,
  t: f64,
  front_face: bool
}

impl HitRecord {
  fn set_face_normal(&self, r: &Ray, outward_normal: &Vec3) -> void {
    self.front_face = dot(r.direction(), outward_normal) < 0;
    if self.front_face {
      self.normal = outward_normal.clone();
    } else {
      self.normal = -(outward_normal.clone());
    }
  }
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}