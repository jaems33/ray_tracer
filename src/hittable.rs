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
  pub fn new() -> Self {
    HitRecord {
      p: Point3::new_empty(),
      normal: Vec3::new_empty(),
      t: 0.0,
      front_face: false
    }
  }
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
  pub fn get_p(self) -> Point3 {
    return self.p.clone();
  }
  pub fn get_normal(&self) -> Vec3 {
    self.normal.clone()
  }
  pub fn get_front_face(&self) -> bool {
    return self.front_face;
  }
  pub fn set_t(&mut self, new_t: f64) {
    self.t = new_t;
  }
  pub fn set_p(&mut self, p: Point3){
    self.p = p.clone();
  }
  pub fn set_front_face(&mut self, face: bool) {
    self.front_face = face;
  }
  pub fn set_normal(&mut self, face: &Vec3) {
    self.normal = face.clone();
  }
}

pub trait Hittable {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}