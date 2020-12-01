use super::lib::{Point3, Vec3};

pub struct Ray {
  origin: Point3,
  direction: Vec3,
}

impl Ray {
  pub fn new(origin: &Point3, direction: &Vec3) -> Ray {
    Ray {
      origin: origin.clone(),
      direction: direction.clone()
    }
  }
  pub fn origin(&self) -> Point3 {
    self.origin.clone()
  }
  pub fn direction(&self) -> Vec3 {
    self.direction.clone()
  }
  pub fn at(&self, t: f64) -> Point3 {
    (t * self.direction) + self.origin
  }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create_ray(){
      let vec = Vec3::new(4.0, 3.0, 2.0);
      let point = Point3::new(1.0, 1.0, 1.0);
      let ray = Ray::new(&point, &vec);

      let mut origin = ray.origin();
      let origin2 = ray.origin();

      origin *= 10.0;

    }


}