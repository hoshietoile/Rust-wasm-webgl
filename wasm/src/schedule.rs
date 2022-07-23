use super::event::{ Event };
use super::event_thread::{ EventThread };
use super::Disk;

#[derive(Debug)]
pub struct Schedule {
  gen_id: u32,
  time: u32,
  iter: u32,
  events: Vec<Event>,
  pub threads: Vec<EventThread>,
}

impl Schedule {
  pub fn new() -> Self {
    Schedule {
      gen_id: 0,
      time: 0,
      iter: 0,
      events: vec![],
      threads: vec![],
    }
  }

  pub fn iterate(&mut self) {
    self.iter += 1;
  }

  pub fn reset_iteration(&mut self) {
    self.iter = 0;
  }

  pub fn generate_id(&mut self) -> u32 {
    self.gen_id += 1;
    self.gen_id
  }

  // threadを追加
  pub fn subscribe_thread(&mut self, thread: EventThread) -> Option<()> {
    // self.threads.push(thread);
    // self.threads = self.threads
    //   .iter()
    //   .map()
    let found = self.threads
      .iter()
      .position(|th| thread.id == th.id);
    match found {
      Some(nth) => {
        let update_target = self.threads.get_mut(nth)?;
        update_target.setting = thread.setting;
        update_target.shot_behavior = thread.shot_behavior;
      },
      None => {
        self.threads.push(thread);
      },
    };
    Some(())
  }

  // threadのSettingに基づいてEventを生成+登録
  pub fn refresh_events(&mut self) {
    self.events = self.threads
      .iter()
      .flat_map(|thread| {
        let start_at_ms = thread.setting.start_at;
        let end_at_ms = thread.setting.end_at;
        let mut buff: Vec<Event> = vec![];

        // relative_ms -> イテレーション内での相対的ms
        // absolute_ms -> スクリーン全体での絶対的ms
        for (relative_ms, absolute_ms) in (start_at_ms..=end_at_ms).enumerate() {
          if (relative_ms as u32) % super::convert_interval_to_frame(thread.setting.shot_interval) == 0 {
            buff.push(Event::new(thread.id, absolute_ms, absolute_ms)); // TODO: end_at入らない気がするが
          }
        }
        buff
      })
      .collect::<Vec<Event>>();
  }

  // 現イテレーションでのeventを巡回
  pub fn walkthrough_events(&mut self, disks: &mut Vec<Option<Disk>>) -> Option<()> {
    let events_iter = self.events
      .iter()
      .filter(|event| event.start_at == self.iter);
    // crate::log!("iter: {:?}, events: {:?}", current_iter, events_iter.clone().collect::<Vec<_>>());
    for event in events_iter {
      let thread = self.threads
        .iter()
        .find(|thread| thread.id == event.thread_id);
      thread?.spawn_disks(&mut disks.as_mut());
    }
    Some(())
  }
}