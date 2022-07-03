#[derive(Debug)]
pub struct Rectangle {
  x: f64,
  y: f64,
  width: f64,
  height: f64,
}

impl Rectangle {
  pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
    Self { x, y, width, height }
  }

  pub fn hit_test(&self, other: &Self) -> bool {
    let horizontal = (other.x < self.x + self.width) && (self.x < other.x + other.width);
    let vertical = (other.y < self.y + self.height) && (self.y < other.y + other.height);
    horizontal && vertical
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn hit_test_test() {
  //   let r1 = Rectangle::new(10., 10., 10., 10.);
  //   let r2 = Rectangle::new(10., 10., 10., 10.);
  //   let r3 = Rectangle::new(10., 10., 10., 10.);
  //   let r4 = Rectangle::new(10., 10., 10., 10.);
  //   let r5 = Rectangle::new(10., 10., 10., 10.);

  // }
}