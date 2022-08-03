import { atom } from 'jotai';

export const gameStateAtom = atom({
  canvas_id: "canvas",
  theme: 1,
  disk_num: 400,
  width: 800,
  height: 800,
  disk_size: 2,
  shot_type: 1,
  shot_speed: 1,
  shot_way_num: 10,
  shot_interval: 1000,
  shot_behavior: [0],
  speed_change_per: 10,
  speed_change_interval: 100,
  x_coordinate: 800 / 2,
  y_coordinate: 800 / 2,
  reflect_count: 0,
  iteration_ms: 2000,
  start_at: 0,
  end_at: 2000,
  sleep_interval: 50,
  sleep_timeout: 20,
  degree_change_by: 1,
  disk_type: 0,
  disk_color: 0,
  gravity_direction: 0,
  gravity_change_per: 10,
});