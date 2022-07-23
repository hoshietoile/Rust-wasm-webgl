use super::shot::{ ShotBehavior, ShotType };
use super::setting::{ Setting };
use super::disk::{ Disk };

#[derive(Debug)]
pub struct EventThread {
  pub id: u32,
  pub shot_behavior: ShotBehavior,
  pub setting: Setting,
}

impl EventThread {
  pub fn new(id: u32, shot_behavior: ShotBehavior, setting: Setting) -> Self {
    EventThread {
      id,
      shot_behavior, 
      setting,
    }
  }

  /**
   * ショット種別毎にScreen.disksへのデータ割り当て
   */
  pub fn spawn_disks(&self, disks: &mut Vec<Option<Disk>>) {
    // crate::log!("{:?}", self.setting.shot_type);

    let new_disks = match self.setting.shot_type {
      ShotType::Circle => {
        let degree = 360. / (self.setting.shot_way_num as f64);
        (0..self.setting.shot_way_num)
          .into_iter()
          .enumerate()
          .map(|(i, _)| {
            let angle = std::f64::consts::PI * ((degree * i as f64) / 180.);
            Some(
              Disk::new(
                self.setting.x_coordinate,
                self.setting.y_coordinate,
                self.setting.shot_behavior,
                self.setting.disk_type,
                self.setting.disk_size,
                angle,
                self.setting.shot_speed,
              ),
            )
          })
          .collect::<Vec<Option<Disk>>>()
      },
      ShotType::Linear => {
        vec![]
      },
      ShotType::Random => {
        vec![]
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

  
  // pub fn update_screen_disks(&self, disks: &mut Vec<Option<Disk>>) {
  //   disks.iter_mut()
  //     .for_each(|disk| {
  //       match (self.setting.shot_type, disk) {
  //         (ShotType::Circle, Some(v)) => {

  //         },
  //         _ => (),
  //       }
  //     })
  // }
}