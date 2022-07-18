import { atom } from 'jotai';

export const gameStateAtom = atom({
  canvas_id: "canvas",
  disk_num: 400,
  width: 450,
  height: 800,
  disk_size: 4,
  shot_type: 0,
  shot_speed: 1,
  shot_way_num: 6,
  shot_interval: 1000,
  shot_behavior: 0,
  speed_change_per: 0.1,
  speed_change_interval: 100,
  x_coordinate: 450 / 2,
  y_coordinate: 800 / 2,
  reflect_count: 0,
});