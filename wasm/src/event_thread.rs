use rand::Rng;

use super::shot::{ ShotBehavior, ShotType };
use super::setting::{ Setting };
use super::disk::{ Disk };

#[derive(Debug, Clone)]
pub struct EventThread {
  pub id: u32,
  pub iter: u32, // スレッド単位での実行時間
  pub setting: Setting,
}

impl EventThread {
  pub fn new(id: u32, setting: Setting) -> Self {
    EventThread {
      id,
      iter: 0,
      setting,
    }
  }

  /**
   * setting更新
   */
  pub fn update_setting(&mut self, setting: Setting) {
    self.setting = setting;
  }

  /**
   * ショット種別毎にScreen.disksへのデータ割り当て
   */
  pub fn spawn_disks(&mut self, disks: &mut Vec<Option<Disk>>) {
    self.iter += 1;

    // 各弾種共通設定
    let sleep_interval= self.setting.sleep_interval;
    let sleep_timeout = self.setting.sleep_timeout;
    let shot_behavior = self.setting.shot_behavior
      .iter()
      .map(|sb| match &sb {
        ShotBehavior::Sleep(_1, _2) => ShotBehavior::Sleep(sleep_interval as i32, sleep_timeout as i32),
        ShotBehavior::SpeedUp(_1, _2) => {
          ShotBehavior::SpeedUp(
            self.setting.speed_change_interval.unwrap_or(0.),
            self.setting.speed_change_per.unwrap_or(1) as f64 / 100.,
          )
        },
        ShotBehavior::SpeedDown(_1, _2) => {
          ShotBehavior::SpeedDown(
            self.setting.speed_change_interval.unwrap_or(0.),
            self.setting.speed_change_per.unwrap_or(1) as f64 / 100.,
          )
        },
        ShotBehavior::Reflect(_) => ShotBehavior::Reflect(self.setting.reflect_count),
        ShotBehavior::Gravity(_1, _2) => ShotBehavior::Gravity(
          self.setting.gravity_direction.unwrap_or(0),
          self.setting.gravity_change_per.unwrap_or(1) as f64 / 100.,
        ),
        _ => ShotBehavior::Normal
      }
    )
    .collect::<Vec<ShotBehavior>>();

    let new_disks = match self.setting.shot_type {
      ShotType::Circle => {
        let degree = 360. / (self.setting.shot_way_num as f64);
        let offset = self.setting.degree_change_by;
        (0..self.setting.shot_way_num)
          .into_iter()
          .enumerate()
          .map(|(i, _)| {
            let angle = std::f64::consts::PI * ((degree * i as f64) / 180.) + (offset * self.iter as f64);
            Some(
              Disk::new(
                self.setting.x_coordinate,
                self.setting.y_coordinate,
                shot_behavior.clone(),
                self.setting.disk_type,
                self.setting.disk_size,
                angle,
                self.setting.shot_speed,
                self.setting.disk_color,
              ),
            )
          })
          .collect::<Vec<Option<Disk>>>()
      },
      ShotType::Linear => {
        let degree = 100. / (self.setting.shot_way_num as f64); // 射出角
        let offset = self.setting.degree_change_by;
        (0..self.setting.shot_way_num)
          .into_iter()
          .enumerate()
          .map(|(i, _)| {
            let angle = std::f64::consts::PI * ((degree * i as f64) / 180.) - std::f64::consts::PI * 50. / 180.  + (offset * self.iter as f64);
            Some(
              Disk::new(
                self.setting.x_coordinate,
                self.setting.y_coordinate,
                shot_behavior.clone(),
                self.setting.disk_type,
                self.setting.disk_size,
                angle,
                self.setting.shot_speed,
                self.setting.disk_color,
              )
            )
          })
          .collect::<Vec<Option<Disk>>>()
      },
      ShotType::Random => {
        let mut rng = rand::thread_rng();
        (0..self.setting.shot_way_num)
          .into_iter()
          .map(|_| {
            let degree = rng.gen_range(0., 1.);
            let angle = std::f64::consts::PI * 180. * degree;
            Some(
              Disk::new(
                self.setting.x_coordinate,
                self.setting.y_coordinate,
                shot_behavior.clone(),
                self.setting.disk_type,
                self.setting.disk_size,
                angle,
                self.setting.shot_speed,
                self.setting.disk_color,
              ),
            )
          })
          .collect::<Vec<Option<Disk>>>()
        
      },
      _ => vec![],
    };

    // ScreenのVec<Disks>の空きに順次割り当てる
    for new_disk in new_disks {
      for disk in disks.iter_mut() {
        match *disk {
          Some(_) => continue,
          _ => {
            *disk = new_disk;
            break;
          }
        } 
      }
    }
  }
}