
#[derive(Debug)]
pub struct Event {
  pub thread_id: u32,
  pub start_at: u32,
  pub end_at: u32,
}

impl Event {
  pub fn new(
    thread_id: u32,
    start_at: u32,
    end_at: u32,
  ) -> Self {
    Event { thread_id, start_at, end_at }
  }
}