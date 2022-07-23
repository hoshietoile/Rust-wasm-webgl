use serde::{ Deserialize, Serialize };
use super::shot::{
  ShotType,
  ShotBehavior, 
  resolve_shot_type,
  resolve_shot_behavior,
};
use super::disk::{ resolve_disk_type, DiskType };

/**
 * Jsからの入力値
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct SettingOptions {
  // Screen設定
  pub canvas_id: String,
  pub width: f64,
  pub height: f64,
  pub iteration_ms: u32, // 1イテレーションのトータルms
  pub start_at: u32, // 開始ms
  pub end_at: u32,

  // Thread設定
  pub disk_size: Option<f64>,
  pub shot_type: Option<u32>,     // 種別
  pub disk_type: Option<u32>,     // 弾種別
  pub shot_speed: Option<f64>,    // 速度
  pub shot_way_num: Option<u32>,  // 発射WAY数
  pub shot_interval: Option<u32>, // 発射間隔
  pub shot_behavior: Option<u32>, // 弾の挙動
  pub speed_change_per: Option<f64>, // ショット速度変化率
  pub speed_change_interval: Option<f64>, // ショット速度変化インターバル
  pub x_coordinate: Option<f64>, // X座標
  pub y_coordinate: Option<f64>, // Y座標
  pub reflect_count: Option<u32>, // 反射数 
}

/**
 * 各Threadに割り当てる設定値
 */
#[derive(Debug, Clone)]
pub struct Setting {
  pub iteration_ms: u32,
  pub start_at: u32,
  pub end_at: u32,
  pub disk_size: f64,
  pub disk_type: DiskType,
  pub shot_type: ShotType,     // 種別
  pub shot_behavior: ShotBehavior, // 弾の挙動
  pub shot_speed: f64,    // 速度
  pub shot_way_num: u32,  // 発射WAY数
  pub shot_interval: u32, // 発射間隔
  pub x_coordinate: f64, // X座標
  pub y_coordinate: f64, // Y座標

  // 設定されていなくてもいいもの
  pub reflect_count: Option<u32>, // 反射数 
  pub speed_change_per: Option<f64>, // ショット速度変化率
  pub speed_change_interval: Option<f64>, // ショット速度変化インターバル
}

impl Setting {
  pub fn new(options: &SettingOptions) -> Self {
    let width = options.width;
    let height = options.height;
    let iteration_ms = options.iteration_ms;
    let start_at = options.start_at;
    let end_at = options.end_at;
    let shot_type = options.shot_type.unwrap_or(0);
    let shot_type = resolve_shot_type(shot_type);
    let shot_behavior = options.shot_behavior.unwrap_or(0);
    let shot_behavior = resolve_shot_behavior(shot_behavior);
    let disk_type = options.disk_type.unwrap_or(0);
    let disk_type = resolve_disk_type(disk_type);
    Self {
      iteration_ms,
      start_at,
      end_at,
      shot_type,
      shot_behavior,
      disk_type,
      disk_size: options.disk_size.unwrap_or(4.),
      shot_speed: options.shot_speed.unwrap_or(1.0),
      shot_way_num: options.shot_way_num.unwrap_or(6), 
      shot_interval: options.shot_interval.unwrap_or(500), 
      x_coordinate: options.x_coordinate.unwrap_or(width / 2.),
      y_coordinate: options.y_coordinate.unwrap_or(height / 2.),
      speed_change_per: options.speed_change_per,
      speed_change_interval: options.speed_change_interval,
      reflect_count: options.reflect_count,
    }
  }
}

// #[cfg(tests)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_settings() {

//   }
// }