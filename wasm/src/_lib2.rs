mod dom_utils;
mod utils;
mod vec2d;
mod rectanble;

use rand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::{WebGlBuffer, WebGlRenderingContext, WebGlUniformLocation, CanvasRenderingContext2d};
use vec2d::{Vec2d};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn output_log(s: &str) {
    log(&format!("Hello {}", s));
}

#[derive(Clone, Copy, Debug)]
pub struct Disk {
    age: u32, // exist age
    x: f64,   // x-coordinate
    y: f64,   // y-coordinate
    reflect_count: Option<u32>,
    behavior: ShotBehavior, // 
    vec2d: Vec2d, // moving velocity
}

impl Disk {
    fn new(x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, angle: f64, speed: f64) -> Self {
        let vec2d = Vec2d::new(angle, speed, x, y);
        Self { age: 0, x, y, reflect_count, behavior, vec2d }
    }

    fn gain_age(&mut self, by: u32) -> Self {
        self.age = self.age + by;
        *self
    }

    fn speed_up(&mut self, by: f64) -> Self {
        self.vec2d.speed_up(by);
        *self
    }

    fn speed_down(&mut self, by: f64) -> Self {
        if self.vec2d.x <= 0.1 || self.vec2d.y <= 0.1 {
            return *self;
        }
        self.vec2d.speed_down(by);
        *self
    }
}

/**
 * ディスクのベクタを初期化する
 */
fn init_disks(disk_num: u32) -> Vec<Box<Option<Disk>>> {
    let mut disks_buffer: Vec<Box<Option<Disk>>> = Vec::with_capacity(disk_num as usize);

    for _ in 0..disk_num {
        disks_buffer.push(Box::new(None));
    }
    disks_buffer
}

/**
 * generate single disk angled with provided degree.
 */
fn gen_single_new_disk(speed: f64, x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, degree: f64) -> Disk {
    let angle = std::f64::consts::PI * 180. * degree;
    Disk::new(
        x,
        y,
        behavior,
        reflect_count,
        angle,
        speed,
    )
}

/**
 * generate single disk with random-generated vectors.
 */
fn gen_random_new_disk(speed: f64, x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>) -> Disk {
    let mut rng = rand::thread_rng();
    gen_single_new_disk(speed, x, y, behavior, reflect_count, rng.gen_range(0., 1.)) 
}

/**
 * generate a group of disks as wall.
 */
fn gen_wall_new_disks(speed: f64, width: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, num: u32) -> Vec<Disk> {
    let span = width / (num as f64);
    let angle = std::f64::consts::PI * (90. * 0.) / 180.;
    (0..num).into_iter()
        .enumerate()
        .map(|(i, v)| {
            Disk::new(
                (i as f64) * span + (span / 2.),
                y,
                behavior,
                reflect_count,
                angle,
                speed,
            )
        })
        .collect::<Vec<Disk>>()
}

/**
 * generate a group of disks with provided number with `num`.
 */
fn gen_circle_new_disks(speed: f64, x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, num: u32, deg_offset: f64) -> Vec<Disk> {
    let deg = 360. / (num as f64);
    (0..num).into_iter()
        .enumerate()
        .map(|(i, _)| {
            let angle = std::f64::consts::PI * ((deg_offset + deg * i as f64) / 180.);
            Disk::new(
                x,
                y,
                behavior,
                reflect_count,
                angle,
                speed,
            )
        })
        .collect::<Vec<Disk>>()
}

/**
 * 放射状
 */
fn gen_radial_new_disks(speed: f64, x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, num: u32) -> Vec<Disk> {
    gen_circle_new_disks(speed, x, y, behavior, reflect_count, num, 0.)
}

/**
 * 回転
 */
fn gen_rotate_new_disks(speed: f64, x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, num: u32, deg_offset: f64) -> Vec<Disk> {
    gen_circle_new_disks(speed, x, y, behavior, reflect_count, num, deg_offset)
}

/**
 * 渦巻状
 */
fn gen_swirl_new_disk(speed: f64, x: f64, y: f64, behavior: ShotBehavior, reflect_count: Option<u32>, num: u32, deg_offset: f64) -> Disk {
    let deg = 360. / (num as f64);
    let i = deg_offset / (num as f64);
    let angle = std::f64::consts::PI * ((deg_offset + deg * i as f64) / 180.);
    Disk::new(
        x,
        y,
        behavior,
        reflect_count,
        angle,
        speed,
    )
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ShotBehavior {
    Normal,
    SpeedUp,   // 加速率(%)/加速イテレーション
    SpeedDown, // 減速率(%)/減速イテレーション
    Reflect,
    Random,
    // Besier(f64, f64),
}

fn resolve_shot_behavior(num: u32) -> ShotBehavior {
    match num {
        1 => ShotBehavior::SpeedUp,
        2 => ShotBehavior::SpeedDown,
        3 => ShotBehavior::Reflect,
        4 => ShotBehavior::Random,
        // 4 => ShotBehavior::Besier(0.1, 100.),
        _ => ShotBehavior::Normal,
    }
}

const FRAMES_PER_SEC: u32 = 60;
const MILLI_SECONDS: f64 = 1_000.;
const DISK_NUM: u32 = 4_096;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Screen {
    iter: u32,
    gl: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    disk_size: f64,
    shot_type: u32,
    shot_speed: f64,
    shot_way_num: u32,
    shot_interval: f64,
    shot_behavior: ShotBehavior,
    speed_change_per: f64,
    speed_change_interval: f64,

    x_coordinate: f64,
    y_coordinate: f64,

    reflect_count: Option<u32>,

    disks: Vec<Box<Option<Disk>>>,

    fps_counter: u32,
    fps_time: f64,
    last_fps: u32,
}

#[wasm_bindgen]
impl Screen {
    /**
     * ショットの種別に従って出力を分岐する
     */
    fn update_on_shot_type(&mut self, iter: u32) -> () {
        match self.shot_type {
            // ShotType::Random => {
            // ランダム
            0 => {
                let new_disk = gen_random_new_disk(
                    1.0,
                    self.x_coordinate,
                    self.y_coordinate,
                    self.shot_behavior,
                    self.reflect_count,
                );
                for d in self.disks.iter_mut() {
                    match **d {
                        Some(_) => continue,
                        _ => {
                            *d = Box::new(Some(new_disk));
                            break;
                        },
                    }
                }
            }
            // ShotType::Circle => {
            // 放射状
            1 => {
                // TODO: optionを返す関数を定義してみる?
                if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
                let new_disks = gen_radial_new_disks(
                    self.shot_speed,
                    self.x_coordinate,
                    self.y_coordinate,
                    self.shot_behavior,
                    self.reflect_count,
                    self.shot_way_num,
                );
                for nd in new_disks {
                    for d in self.disks.iter_mut() {
                        match **d {
                            Some(_) => continue,
                            _ => {
                                *d = Box::new(Some(nd));
                                break;
                            }
                        }
                    }
                }
            }
            // 放射状回転
            2 => {
                // TODO: optionを返す関数を定義してみる?
                if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
                let new_disks = gen_rotate_new_disks(
                    self.shot_speed,
                    self.x_coordinate,
                    self.y_coordinate,
                    self.shot_behavior,
                    self.reflect_count,
                    self.shot_way_num,
                    1.5 * (iter as f64),
                );
                for nd in new_disks {
                    for d in self.disks.iter_mut() {
                        match **d {
                            Some(_) => continue,
                            _ => {
                                *d = Box::new(Some(nd));
                                break;
                            }
                        }
                    }
                }
            }
            // 渦巻
            3 => {
                if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
                let new_disk = gen_swirl_new_disk(
                    1.0,
                    self.x_coordinate,
                    self.y_coordinate,
                    self.shot_behavior,
                    self.reflect_count,
                    self.shot_way_num, 
                    1.5 * (iter as f64),
                );
                for d in self.disks.iter_mut() {
                    match **d {
                        Some(_) => continue,
                        _ => {
                            *d = Box::new(Some(new_disk));
                            break;
                        },
                    }
                }
            }
            // 撃ち降ろし
            4 => {
              if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
              let new_disks = gen_wall_new_disks(
                  self.shot_speed,
                  self.width as f64,
                  self.y_coordinate,
                  self.shot_behavior,
                  self.reflect_count,
                  self.shot_way_num,
              );
              for nd in new_disks {
                  for d in self.disks.iter_mut() {
                      match **d {
                          Some(_) => continue,
                          _ => {
                              *d = Box::new(Some(nd));
                              break;
                          }
                      }
                  }
              }  
            }
            _ => { return () }
        }
    }

    // 反射時の処理
    fn on_reflect(&self, v: &mut Disk) -> Box<Option<Disk>> {
        let size = self.disk_size as f64;
        let width = self.width as f64;
        let height = self.height as f64;
        let should_reflect = v.reflect_count.unwrap_or(0) > 0;
        return match (should_reflect, v.behavior) {
            // 反射弾の場合
            (true, ShotBehavior::Reflect) => {
                // X軸方向
                if v.x - size < 0. || v.x + size > width {
                    v.x -= v.vec2d.x;
                    v.vec2d.x = -v.vec2d.x;
                    v.reflect_count = v.reflect_count
                        .and_then(|v| Some(v - 1));
                }
                // Y軸方向
                if v.y - size < 0. || v.y + size > height {
                    v.y -= v.vec2d.y;
                    v.vec2d.y = -v.vec2d.y;
                    v.reflect_count = v.reflect_count
                        .and_then(|v| Some(v - 1));
                }
                Box::new(Some(*v))
            }
            _ => {
                // 通常弾の場合
                if v.x + size < 0. || v.x - size > width || v.y + size < 0. || v.y - size > height {
                    return Box::new(None);
                }
                Box::new(Some(*v))
            }
        }
    }

    /**
     * 1イテレーションごとの座標計算
     */
    fn on_animation_frame(&mut self, time: f64) -> () {
        self.iter += 1;

        // ショットタイプに応じた処理を実行
        self.update_on_shot_type(self.iter);

        // FPS
        self.calc_fps(time);

        // Disk数
        log!("disk_nums: {}", self.disks
            .clone()
            .into_iter()
            .filter_map(|disk| {
                match *disk {
                    Some(_) => Some(1),
                    _ => None,
                }
            })
            .count(),
        );
        log!("last_fps: {}", self.last_fps);
        
        // ランダム弾用
        let mut rng = rand::thread_rng();
        
        // 座標計算処理
        let size = self.disk_size as f64;
        let width = self.width as f64;
        let height = self.height as f64;
        self.disks = self.disks
            .clone()
            .into_iter()
            .map(|disk| {
                let d = *(disk.clone().as_ref());
                match d {
                    Some(mut v) => {
                        v.x += v.vec2d.x;
                        v.y += v.vec2d.y;

                        /** ランダム挙動...きもいので要調整 */
                        if self.shot_behavior == ShotBehavior::Random {
                            if v.behavior == ShotBehavior::Random {
                                let angle = std::f64::consts::PI * 180. * rng.gen_range(0., 1.);
                                v.vec2d.angle_change(angle);
                            }
                        }

                        /** 速度変化 */
                        let speed_change_flg = (v.age as f64) % (FRAMES_PER_SEC as f64) * (self.speed_change_interval / (MILLI_SECONDS as f64)) == 0.;
                        if speed_change_flg {
                            match self.shot_behavior {
                                ShotBehavior::SpeedUp => {
                                    v.speed_up(self.speed_change_per);
                                },
                                ShotBehavior::SpeedDown => {
                                    v.speed_down(self.speed_change_per);
                                },
                                _ => (),
                            }
                        }

                        // 弾種による分岐
                        // 反射弾
                        if self.shot_behavior == ShotBehavior::Reflect {
                            return self.on_reflect(&mut v);
                        }

                        // 通常弾の場合
                        if v.x + size < 0. || v.x - size > width || v.y + size < 0. || v.y - size > height {
                            return Box::new(None);
                        }

                        Box::new(Some(v))
                    },
                    _ => Box::new(None),
                }
            })
            .collect::<Vec<Box<Option<Disk>>>>();
    }

    /**
     * FPS計算
     */
    fn calc_fps(&mut self, time: f64) {
        if self.fps_time + 1000. < time {
            self.last_fps = self.fps_counter;
            self.fps_counter = 0;
            self.fps_time = time;
        }
        self.fps_counter += 1;
    }

    /**
     * 各アニメーションフレームごとの処理
     */
    pub fn do_frame(&mut self, time: f64) -> () {
        self.on_animation_frame(time);
        self.draw();
    }

    /**
     * レンダリング処理
     */
    fn draw(&self) {
        self.gl.save();
        self.gl.set_fill_style(&JsValue::from("rgb(80, 80, 80, 0.8)"));
        self.gl.fill_rect(0., 0., self.width as f64, self.height as f64);
        self.gl.set_fill_style(&JsValue::from(format!("rgb(155, 0, 0, 1")));

        for (_, disk) in self.disks.iter().enumerate() {
            let d = **disk;
            match d {
                Some(d) => {
                    self.gl.begin_path();
                    self.gl
                        .arc(
                            d.x.into(),
                            d.y.into(),
                            self.disk_size,
                            0.,
                            std::f64::consts::PI * 2.0,
                        )
                        .unwrap();
                    self.gl.fill();
                },
                _ => {
                    continue;
                }
            }
        }

        self.gl.restore();
    }

    pub fn update(&mut self, option_input: JsValue) {
        let options: Options = option_input.into_serde().unwrap();
        self.width = options.width.unwrap_or(500);
        self.height = options.height.unwrap_or(500);
        self.disk_size = options.disk_size.unwrap_or(32.);
        self.shot_type = options.shot_type.unwrap_or(0);
        self.shot_speed= options.shot_speed.unwrap_or(1.0);
        self.shot_way_num = options.shot_way_num.unwrap_or(6);
        self.shot_interval= options.shot_interval.unwrap_or(500.);
        self.speed_change_per = options.speed_change_per.unwrap_or(0.1);
        self.speed_change_interval = options.speed_change_interval.unwrap_or(100.);
        self.x_coordinate = options.x_coordinate.unwrap_or((self.width / 2) as f64);
        self.y_coordinate = options.y_coordinate.unwrap_or((self.height / 2) as f64);
        self.reflect_count = options.reflect_count;

        let shot_behavior= options.shot_behavior.unwrap_or(0);
        self.shot_behavior = resolve_shot_behavior(shot_behavior);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub canvas_id: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub disk_size: Option<f64>,
    pub collision: Option<bool>,
    pub shot_type: Option<u32>,     // 種別
    pub shot_speed: Option<f64>,    // 速度
    pub shot_way_num: Option<u32>,  // 発射WAY数
    pub shot_interval: Option<f64>, // 発射間隔
    pub shot_behavior: Option<u32>, // 弾の挙動
    pub speed_change_per: Option<f64>,
    pub speed_change_interval: Option<f64>,
    pub x_coordinate: Option<f64>, // X座標
    pub y_coordinate: Option<f64>, // Y座標
    pub reflect_count: Option<u32>, // 反射数 
}

#[wasm_bindgen]
pub fn init_gl(option_input: JsValue) -> Screen {
    log!("options {:?}", option_input); 
    let options: Options = option_input.into_serde().unwrap();
    let canvas_id = options.canvas_id;
    let width = options.width.unwrap_or(500);
    let height = options.height.unwrap_or(500);
    let disk_size = options.disk_size.unwrap_or(32.);
    let shot_type = options.shot_type.unwrap_or(0);
    let shot_speed= options.shot_speed.unwrap_or(1.0);
    let shot_way_num = options.shot_way_num.unwrap_or(6);
    let shot_interval= options.shot_interval.unwrap_or(500.);
    let shot_behavior= options.shot_behavior.unwrap_or(0);
    let speed_change_per = options.speed_change_per.unwrap_or(0.1);
    let speed_change_interval = options.speed_change_interval.unwrap_or(100.);
    let shot_behavior = resolve_shot_behavior(shot_behavior);
    let x_coordinate = options.x_coordinate.unwrap_or((width / 2) as f64);
    let y_coordinate = options.y_coordinate.unwrap_or((height / 2) as f64);
    let reflect_count = options.reflect_count;

    let context = dom_utils::get_context2d_by_id(canvas_id.as_str(), width, height).unwrap();

    let disks = init_disks(DISK_NUM);

    Screen {
        iter: 0,
        gl: context,
        disks,
        disk_size,
        width,
        height,
        shot_type,
        shot_speed,
        shot_way_num,
        shot_interval,
        shot_behavior,
        speed_change_per,
        speed_change_interval,
        x_coordinate,
        y_coordinate,
        reflect_count,
        fps_counter: 0,
        fps_time: 0.,
        last_fps: 0,
    }
}




// #[derive(Debug)]
// struct MyDisk {
//     x: f64,
//     y: f64,
// }

// #[derive(Debug)]
// struct MyThread <'a> {
//     disks: &'a Vec<MyDisk>,
// }

// #[derive(Debug)]
// struct MyGame {
//     disks: Vec<MyDisk>,
// }

// fn main() {
//     let d1 = MyDisk{x:10., y:10.};
//     let d2 = MyDisk{x:20., y:20.};
//     let d3 = MyDisk{x:30., y:30.};
//     let g = Game {
//         disks: vec![d1, d2, d3],
//     };
//     let mut t = MyThread{disks: &g.disks};
//     // println!("{:?}", g);
//     t.disks.as_mut().map(|v| {
//         println!("{:?}", v);
        
//     })
//     println!("{:?}", t);
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let mem = Arc::new(Mutex::new(0));

//     thread::spawn({
//         let mem_local = Arc::clone(&mem);
//         move || {
//             loop {
//                 thread::sleep(Duration::from_millis(10));
//                 let num = mem_local.lock().unwrap();
//                 if *num > 100 {
//                     println!("break");
//                     exit(0);
//                 }
//             }
//         }
//     });

//     let mem_local = Arc::clone(&mem);
//     for _num in 0..200 {
//         thread::sleep(Duration::from_millis(100));
//         let mut num = mem_local.lock().unwrap();
//         *num = *num + 1;
//         println!("{}", *num);
//     }
// }