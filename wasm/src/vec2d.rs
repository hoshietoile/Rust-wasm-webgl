use std::ops::{ Add, Sub };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
  pub fn new(angle: f64, speed: f64) -> Self {
    let sin = angle.sin() * speed; 
    let cos = angle.cos() * speed; 
    Self { x: sin, y: cos }
  }

  // pub fn normalize(&self) -> Self {
  //     let d = (self.x * self.x + self.y * self.y).sqrt();
  //     if d == 0. {
  //         Vec2d {
  //           angle: self.angle,
  //           speed: self.speed,
  //           x: 0.,
  //           y: 0.,
  //       }
  //     } else {
  //         Vec2d {
  //           angle: self.angle,
  //           speed: self.speed,
  //           x: self.x / d,
  //           y: self.y / d,
  //         }
  //     }
  // }

  // pub fn distance_from(&self, other: Self) -> f64 {
  //     ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  // }

  // pub fn mul(&self, k: f64) -> Self {
  //     Self {
  //         angle: self.angle,
  //         speed: self.speed,
  //         x: self.x * k,
  //         y: self.y * k,
  //     }
  // }
 
  pub fn angle_change(&mut self, angle: f64, speed: f64) {
    self.x = angle.sin() * speed;
    self.y = angle.cos() * speed;
  }
}

impl Add for Vec2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2d { 
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod test {
  use super::*;

  // #[test]
  // fn vec2d_distance() {
  //   let criteria = Vec2d::new(5., 5.);
  //   let v = criteria.distance_from(Vec2d::new(10., 10.));
  //   assert_eq!(v, (50.0f64).sqrt());
  // }
}