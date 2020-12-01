use std::ops::{Add, AddAssign, Sub, Index, MulAssign, DivAssign};

pub struct Vec3 {
    values: [f64; 3],
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub enum Channel {
    Red,
    Green,
    Blue
}

impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 {
            values: [e0, e1, e2],
        }
    }
    fn new_empty() -> Self {
        Vec3 {
            values: [0.0, 0.0, 0.0],
        }
    }
    fn x(&self) -> f64 {
        self.values[0]
    }
    fn y(&self) -> f64 {
        self.values[1]
    }
    fn z(&self) -> f64 {
        self.values[2]
    }

    fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            values: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            values: [
                self.x() - other.x(),
                self.y() - other.y(),
                self.z() - other.z(),
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.values[0] += rhs.values[0];
        self.values[1] += rhs.values[1];
        self.values[2] += rhs.values[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.values[0] *= rhs;
        self.values[1] *= rhs;
        self.values[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.values[0] /= rhs;
        self.values[1] /= rhs;
        self.values[2] /= rhs;
    }
}

impl Index<Channel> for Vec3 {
    type Output = f64;
    fn index(&self, index: Channel) -> &Self::Output {
        match index {
            Channel::Red => &self.values[0],
            Channel::Green => &self.values[1],
            Channel::Blue => &self.values[2],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use float_cmp::approx_eq;
    #[test]
    fn it_works_with_one_element() {
        let vec = Vec3::new(24.0, 12.0, 8.0);
        assert!(approx_eq!(f64, vec.values[0], 24.0, ulps = 2));
        assert!(approx_eq!(f64, vec.values[1], 12.0, ulps = 2));
        assert!(approx_eq!(f64, vec.values[2], 8.0, ulps = 2));
    }

    #[test]
    fn can_be_empty() {
        let vec = Vec3::new_empty();
        assert!(approx_eq!(f64, vec.values[0], 0.0, ulps = 2));
        assert!(approx_eq!(f64, vec.values[1], 0.0, ulps = 2));
        assert!(approx_eq!(f64, vec.values[2], 0.0, ulps = 2));
    }

    #[test]
    fn can_add_two_vec3s() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        vec1 += vec2;
        assert!(approx_eq!(f64, vec1.x(), 3.0, ulps = 2));
        assert!(approx_eq!(f64, vec1.y(), 5.0, ulps = 2));
        assert!(approx_eq!(f64, vec1.z(), 7.0, ulps = 2));
    }

    #[test]
    fn can_sub_two_vec3s() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 3.0, 4.0);
        vec1 = vec1 - vec2;
        assert!(approx_eq!(f64, vec1.x(), -1.0, ulps = 2));
        assert!(approx_eq!(f64, vec1.y(), -1.0, ulps = 2));
        assert!(approx_eq!(f64, vec1.z(), -1.0, ulps = 2));
    }

    #[test]
    fn can_mul_two_vec3s() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        vec1 *= 2.0;
        assert!(approx_eq!(f64, vec1.x(), 2.0, ulps = 2));
        assert!(approx_eq!(f64, vec1.y(), 4.0, ulps = 2));
        assert!(approx_eq!(f64, vec1.z(), 6.0, ulps = 2));
    }

    #[test]
    fn can_index_into_vec3s() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        assert!(approx_eq!(f64, vec1[Channel::Red], 1.0, ulps = 2));
        assert!(approx_eq!(f64, vec1[Channel::Green], 2.0, ulps = 2));
        assert!(approx_eq!(f64, vec1[Channel::Blue], 3.0, ulps = 2));
    }

    #[test]
    fn can_get_length_squared() {
        let vec = Vec3::new(3.0, 2.0, 4.0);
        assert!(approx_eq!(f64, vec.length_squared(), 29.0, ulps = 2));
    }

    #[test]
    fn can_get_length() {
        let vec = Vec3::new(2.0, 2.0, 1.0);
        assert!(approx_eq!(f64, vec.length(), 3.0, ulps = 2));
    }
}
