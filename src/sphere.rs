use super::lib::{Vec3, Point3, dot};
use super::ray::{Ray};
use super::hittable::{Hittable, HitRecord};

pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  fn new(center: &Point3, r: f64){
    Sphere {
      center,
      r
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc: Vec3 = r.origin() - center.clone();
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    let root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return false;
      }
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    let outward_normal = (rec.p - c) / self.radius;
    rec.set_face_normal(r, outward_normal);

    return true;
  }
}