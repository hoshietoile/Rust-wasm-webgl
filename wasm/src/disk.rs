use super::shot::{ ShotBehavior };
use super::vec2d::{ Vec2d };

// TODO: 仮
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

#[derive(Debug, Copy, Clone)]
pub enum DiskColor {
  Red,
  Orange,
  Yellow,
  Green,
  LightBlue,
  Blue,
  DeepBlue,
  Purple,
  Pink,
}

pub fn resolve_disk_color(disk_color: u32) -> DiskColor {
  match disk_color {
    0 => DiskColor::Red,
    1 => DiskColor::Orange,
    2 => DiskColor::Yellow,
    3 => DiskColor::Green,
    4 => DiskColor::LightBlue,
    5 => DiskColor::Blue,
    6 => DiskColor::DeepBlue,
    7 => DiskColor::Purple,
    8 => DiskColor::Pink,
    _ => DiskColor::Red,
  }
}

#[derive(Debug, Clone)]
pub struct Disk {
    pub age: u32, // exist age
    pub reflect_count: Option<u32>, // reflect count
    pub x: f64,   // x-coordinate
    pub y: f64,   // y-coordinate
    pub speed: f64, // speed
    pub angle: f64, //
    pub vec2d: Vec2d, // moving velocity
    pub behavior: Vec<ShotBehavior>, // shot behavior.
    pub disk_type: DiskType, // disk type.
    pub disk_size: f64, // disk size.
    pub img_source: Option<String>, // image source.
    pub sleep_time: i32,
    pub disk_color: DiskColor,
}

impl Disk {
    pub fn new(
      x: f64,
      y: f64,
      behavior: Vec<ShotBehavior>,
      disk_type: DiskType,
      disk_size: f64,
      angle: f64,
      speed: f64,
      disk_color: DiskColor,
    ) -> Self {
        let vec2d = Vec2d::new(angle, speed);
        let reflect_count = behavior
          .iter()
          .find_map(|&sb| match sb {
            ShotBehavior::Reflect(num) => num,
            _ => None,
          });
        Self {
          age: 0,
          reflect_count,
          x,
          y,
          speed,
          angle,
          vec2d,
          behavior, 
          disk_type,
          disk_size,
          img_source: None,
          sleep_time: 0,
          disk_color,
        }
    }

    pub fn gain_age(&mut self, by: u32) {
        self.age = self.age + by;
    }
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
      vec![ShotBehavior::Normal],
      disk_type,
      4.,
      PI * 90. / 180.,
      10.,
      DiskColor::Red,
    );
    disk.gain_age(1);
    assert_eq!(disk.age, 1);
    disk.gain_age(5);
    assert_eq!(disk.age, 6);
  }
}