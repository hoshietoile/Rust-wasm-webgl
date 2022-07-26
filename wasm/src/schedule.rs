use super::event::{ Event };
use super::event_thread::{ EventThread };
use super::Disk;

#[derive(Debug)]
pub struct Schedule {
  gen_id: u32,
  end_at: u32,
  iter: u32,
  events: Vec<Event>,
  pub threads: Vec<EventThread>,
}

impl Schedule {
  pub fn new() -> Self {
    Schedule {
      gen_id: 0,
      end_at: 0,
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
        let shot_interval_frame = super::convert_interval_to_frame(thread.setting.shot_interval);

        // relative_ms -> イテレーション内での相対的ms
        // absolute_ms -> スクリーン全体での絶対的ms
        for absolute_ms in start_at_ms..=end_at_ms {
          if (absolute_ms as f64) % shot_interval_frame == 0. {
            buff.push(Event::new(thread.id, absolute_ms, absolute_ms)); // TODO: end_at入らない気がするが
          }
        }
        buff
      })
      .collect::<Vec<Event>>();

    let end_at = self.events
      .iter()
      .max_by(|a, b| a.end_at.cmp(&b.end_at));

    self.end_at = end_at.unwrap().end_at;
  }

  // 現イテレーションでのeventを巡回
  pub fn walkthrough_events(&mut self, disks: &mut Vec<Option<Disk>>) -> Option<()> {
    let iter = self.iter;
    // リセット 必要かは検討
    if iter == self.end_at {
      self.reset_iteration();
    }

    let events_iter = self.events
      .iter()
      .filter(|event| event.start_at == iter);
    for event in events_iter {
      let thread = self.threads
        .iter_mut()
        .find(|thread| thread.id == event.thread_id);
      thread?.spawn_disks(&mut disks.as_mut());
    }
    Some(())
  }
}