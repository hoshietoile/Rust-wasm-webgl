use super::shot::{ ShotBehavior };
use super::vec2d::{ Vec2d };

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DiskType {
  Oval,    // 楕円
  Dot,     // 米粒
  Circle,  // 中玉
  Orb,     // 大玉
  Arrow,   // 弓矢
}

pub fn resolve_disk_type(disk_type: u32) -> DiskType {
  match disk_type {
    0 => DiskType::Oval,
    1 => DiskType::Dot,
    2 => DiskType::Circle,
    3 => DiskType::Orb,
    4 => DiskType::Arrow,
    _ => DiskType::Oval,
  }
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub age: u32, // exist age
    pub reflect_count: u32, // reflect count
    pub x: f64,   // x-coordinate
    pub y: f64,   // y-coordinate
    pub speed: f64, // speed
    pub vec2d: Vec2d, // moving velocity
    pub behavior: ShotBehavior, // shot behavior.
    pub disk_type: DiskType, // disk type.
    pub disk_size: f64, // disk size.
    pub img_source: Option<String>, // image source.
}

impl Disk {
    pub fn new(
      x: f64,
      y: f64,
      behavior: ShotBehavior,
      disk_type: DiskType,
      disk_size: f64,
      angle: f64,
      speed: f64,
    ) -> Self {
        let vec2d = Vec2d::new(angle, speed);
        Self {
          age: 0,
          reflect_count: 0,
          x,
          y,
          speed,
          vec2d,
          behavior, 
          disk_type,
          disk_size,
          img_source: None,
        }
    }

    fn gain_age(&mut self, by: u32) {
        self.age = self.age + by;
    }

    // fn speed_up(&mut self, by: f64) {
    //     self.vec2d.speed_up(by);
    // }

    // fn speed_down(&mut self, by: f64) {
    //     if self.vec2d.x >= 0.1 && self.vec2d.y >= 0.1 {
    //       self.vec2d.speed_down(by);
    //     }
    // }

    // pub fn speed_up(&mut self, by: f64) {
    //   let speed = self.speed + by;
    //   let sin = self.angle.sin() * speed;
    //   let cos = self.angle.cos() * speed;
    //   self.x = sin;
    //   self.y = cos;
    //   self.speed = speed;
    // }

    // pub fn speed_down(&mut self, by: f64) {
    //   let speed = self.speed - by;
    //   let sin = self.angle.sin() * speed;
    //   let cos = self.angle.cos() * speed;
    //   self.x = sin;
    //   self.y = cos;
    //   self.speed = speed;
    // }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::f64::consts::PI;

  #[test]
  fn test_disk_type_resolve() {
    let results = vec![
      DiskType::Oval,     
      DiskType::Dot,
      DiskType::Circle,
      DiskType::Orb,
      DiskType::Arrow,
      DiskType::Oval,     
    ];
    let nums = (0..=5).into_iter().collect::<Vec<_>>();
    for (i, &num) in nums.iter().enumerate() {
      let disk_type = resolve_disk_type(num);
      assert_eq!(results[i], disk_type);
    }
  }

  #[test]
  fn test_gain_age() {
    let disk_type = resolve_disk_type(0);
    let mut disk = Disk::new(
      10.,
      10.,
      ShotBehavior::Normal,
      disk_type,
      4.,
      PI * 90. / 180.,
      10.,
    );
    disk.gain_age(1);
    assert_eq!(disk.age, 1);
    disk.gain_age(5);
    assert_eq!(disk.age, 6);
  }
}