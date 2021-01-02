use super::lib::{Vec3, Point3, dot};
use super::ray::{Ray};
use super::hittable::{Hittable, HitRecord};

pub struct Sphere {
  center: Point3,
  radius: f64,
}

impl Sphere {
  pub fn new(c: &Point3, r: f64) -> Self {
    Sphere {
      center: c.clone(),
      radius: r
    }
  }
}

impl Hittable for Sphere {
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
    let oc: Vec3 = r.origin() - self.center.clone();
    let a = r.direction().length_squared();
    let half_b = dot(&oc, &r.direction());
    let c = oc.length_squared() - self.radius*self.radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
      return false;
    }
    let sqrtd = discriminant.sqrt();

    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || t_max < root {
      root = (-half_b + sqrtd) / a;
      if root < t_min || t_max < root {
        return false;
      }
    }

    rec.set_t(root);
    rec.set_p(r.at(rec.get_t()));
    let outward_normal = (rec.get_p() - self.center.clone()) / self.radius;
    rec.set_face_normal(r, &outward_normal);

    return true;
  }
}