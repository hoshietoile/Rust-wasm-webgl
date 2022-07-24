import { atom } from 'jotai';

export const gameStateAtom = atom({
  canvas_id: "canvas",
  disk_num: 400,
  width: 450,
  height: 800,
  disk_size: 2,
  shot_type: 1,
  shot_speed: 1,
  shot_way_num: 10,
  shot_interval: 1000,
  shot_behavior: 0,
  speed_change_per: 0.1,
  speed_change_interval: 100,
  x_coordinate: 450 / 2,
  y_coordinate: 800 / 2,
  reflect_count: 0,
  iteration_ms: 100,
  start_at: 200,
  end_at: 10000,
  sleep_interval: 50,
  sleep_timeout: 20,
});