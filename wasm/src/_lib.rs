mod dom_utils;
mod utils;
mod vec2d;
mod rectanble;

use rand::Rng;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::{WebGlBuffer, WebGlRenderingContext, WebGlUniformLocation};
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
    vec2d: Vec2d, // moving velocity
}

impl Disk {
    fn new(x: f64, y: f64, angle: f64, speed: f64) -> Self {
        let vec2d = Vec2d::new(angle, speed, x, y);
        Self { age: 0, x, y, vec2d }
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
fn gen_single_new_disk(speed: f64, bound_x: u32, bound_y: u32, degree: f64) -> Disk {
    let angle = std::f64::consts::PI * 180. * degree;
    Disk::new(
        (bound_x as f64) / 2.,
        (bound_y as f64) / 2.,
        angle,
        speed,
    )
}

/**
 * generate single disk with random-generated vectors.
 */
fn gen_random_new_disk(speed: f64, bound_x: u32, bound_y: u32) -> Disk {
    let mut rng = rand::thread_rng();
    gen_single_new_disk(speed, bound_x, bound_y, rng.gen_range(0., 1.)) 
}

/**
 * generate a group of disks with provided number with `num`.
 */
fn gen_circle_new_disks(speed: f64, bound_x: u32, bound_y: u32, num: u32, deg_offset: f64) -> Vec<Disk> {
    let mut v: Vec<Disk> = vec![];
    let deg = 360. / (num as f64);
    for i in (0..num).into_iter() {
        let angle = std::f64::consts::PI * ((deg_offset + deg * i as f64) / 180.);
        let d = Disk::new(
            (bound_x as f64) / 2.,
            (bound_y as f64) / 2.,
            angle,
            speed,
        );
        v.push(d);
    }
    v
}

/**
 * 放射状
 */
fn gen_radial_new_disks(speed: f64, bound_x: u32, bound_y: u32, num: u32) -> Vec<Disk> {
    gen_circle_new_disks(speed, bound_x, bound_y, num, 0.)
}

/**
 * 回転
 */
fn gen_rotate_new_disks(speed: f64, bound_x: u32, bound_y: u32, num: u32, deg_offset: f64) -> Vec<Disk> {
    gen_circle_new_disks(speed, bound_x, bound_y, num, deg_offset)
}

/**
 * 渦巻状
 */
fn gen_swirl_new_disk(speed: f64, bound_x: u32, bound_y: u32, num: u32, deg_offset: f64) -> Disk {
    let deg = 360. / (num as f64);
    let i = deg_offset / (num as f64);
    // let degree = deg_offset + deg * i; 
    let angle = std::f64::consts::PI * ((deg_offset + deg * i as f64) / 180.);
    // gen_single_new_disk(speed, bound_x, bound_y, degree)
    Disk::new(
        (bound_x as f64) / 2.,
        (bound_y as f64) / 2.,
        angle,
        speed,
    )
}

#[derive(Debug)]
enum ShotBehavior {
    Normal,
    // SpeedUp(f64, f64),   // 加速率(%)/加速イテレーション
    // SpeedDown(f64, f64), // 減速率(%)/減速イテレーション
    SpeedUp,   // 加速率(%)/加速イテレーション
    SpeedDown, // 減速率(%)/減速イテレーション
    Reflect,
    Besier(f64, f64),
}

fn resolve_shot_behavior(num: u32) -> ShotBehavior {
    match num {
        1 => ShotBehavior::SpeedUp,
        2 => ShotBehavior::SpeedDown,
        3 => ShotBehavior::Reflect,
        // 4 => ShotBehavior::Besier(0.1, 100.),
        _ => ShotBehavior::Normal,
    }
}


// TODO: WASMでEnum使う方法わからん
// #[derive(Debug)]
// #[derive(Serialize, Deserialize)]
// #[wasm_bindgen]
// pub enum ShotType {
//     Circle,
//     Random,
// }

const FRAMES_PER_SEC: u32 = 60;
const MILLI_SECONDS: f64 = 1_000.;
const DISK_NUM: u32 = 4_096;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Screen {
    iter: u32,
    gl: WebGlRenderingContext,
    uniform_point_size: WebGlUniformLocation,
    buffer_coords: WebGlBuffer,

    attrib_coords: i32,
    attrib_color: i32,
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

    disks: Vec<Box<Option<Disk>>>,

    fps_counter: u32,
    fps_time: f64,
    last_fps: u32,
}

// fn test_milli_sec() {}

#[wasm_bindgen]
impl Screen {
    /**
     * ショットの種別に従って出力を分岐する
     */
    fn resolve_shot_type(&mut self, iter: u32) -> () {
        match self.shot_type {
            // ShotType::Random => {
            0 => {
                let new_disk = gen_random_new_disk(1.0, self.width, self.height);
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
            1 => {
                // TODO: optionを返す関数を定義してみる?
                if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
                let new_disks = gen_radial_new_disks(self.shot_speed, self.width, self.height, self.shot_way_num);
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
            2 => {
                // TODO: optionを返す関数を定義してみる?
                if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
                let new_disks = gen_rotate_new_disks(self.shot_speed, self.width, self.height, self.shot_way_num, 1.5 * (iter as f64));
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
            3 => {
                if (iter as f64) % ((FRAMES_PER_SEC as f64) * (self.shot_interval / MILLI_SECONDS)) != 0. { return }
                let new_disk = gen_swirl_new_disk(1.0, self.width, self.height, self.shot_way_num,  1.5 * (iter as f64));
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
            _ => { return () }
        }
    }
    /**
     * 1イテレーションごとの座標計算
     */
    fn on_animation_frame(&mut self, time: f64) -> () {
        self.iter += 1;
        // ショットタイプに応じた処理を実行
        self.resolve_shot_type(self.iter);

        self.calc_fps(time);
        // log!("{}", time);
        log!("last_fps: {}", self.last_fps);

        // 速度の変化
        // ※ このやり方だと どちらかというと弾幕全体の速度変化
        // -> 弾一つ一つの残存時間と速度変化をやりたい
        // let speed_change_flg = (iter as f64) % (FRAMES_PER_SEC as f64) * (self.speed_change_interval / (MILLI_SECONDS as f64)) == 0.;
        // log!("{}", speed_change_flg);
        
        
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
                        if v.x - size < 0. {
                            return Box::new(None);
                        } else if v.x + size > width {
                            return Box::new(None);
                        }
                        if v.y - size < 0. {
                            return Box::new(None);
                        } else if v.y + size > height {
                            return Box::new(None);
                        }
                        Box::new(Some(v))
                    },
                    _ => Box::new(None),
                }
            })
            .collect::<Vec<Box<Option<Disk>>>>();
    }

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
        // log!("{}", iter);
        self.on_animation_frame(time);
        self.draw();
    }

    /**
     * レンダリング処理
     */
    fn draw(&self) -> () {
        self.gl.clear_color(0., 0., 0., 1.);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.buffer_coords),
        );
        let buff_vec = self
            .disks
            .iter()
            .flat_map(|d| {
                match **d {
                    Some(disk) => {
                        let cloned = disk.clone();
                        vec![cloned.x as f32, cloned.y as f32]
                    },
                    None => vec![0. , 0.]
                }
            })
            .collect::<Vec<f32>>();
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &js_sys::Float32Array::view(buff_vec.as_slice()), //
                WebGlRenderingContext::STREAM_DRAW,
            )
        }
        self.gl.vertex_attrib_pointer_with_f64(
            self.attrib_coords as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0.,
        );
        self.gl
            .enable_vertex_attrib_array(self.attrib_coords as u32);

        self.gl.enable_vertex_attrib_array(self.attrib_color as u32);
        self.gl
            .vertex_attrib3f(self.attrib_color as u32, 1., 0., 0.);

        self.gl
            .uniform1f(Some(&self.uniform_point_size), self.disk_size as f32);

        self.gl
            .draw_arrays(WebGlRenderingContext::POINTS, 0, DISK_NUM as i32);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub canvas_id: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub disk_size: Option<f64>,
    pub collision: Option<bool>,
    // pub shot_type: Option<ShotType>,
    pub shot_type: Option<u32>,     // 種別
    pub shot_speed: Option<f64>,    // 速度
    pub shot_way_num: Option<u32>,  // 発射WAY数
    pub shot_interval: Option<f64>, // 発射間隔
    pub shot_behavior: Option<u32>, // 弾の挙動
    pub speed_change_per: Option<f64>,
    pub speed_change_interval: Option<f64>,
}


#[wasm_bindgen]
pub fn init_gl(option_input: JsValue) -> Screen {
    log!("options before"); 
    log!("options {:?}", option_input); 
    let options: Options = option_input.into_serde().unwrap();
    log!("options {:?}", options);
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
    log!("speed_change_interval {:?}", speed_change_interval);
    let shot_behavior = resolve_shot_behavior(shot_behavior);
    log!("shot behavior{:?}", shot_behavior);

    let context = dom_utils::get_webgl_context_by_id(canvas_id.as_str(), width, height).unwrap();
    let program = dom_utils::create_program(&context).unwrap();
    context.use_program(Some(&program));

    let disks = init_disks(DISK_NUM);
    let attrib_coords = context.get_attrib_location(&program, "a_coords");
    let buffer_coords = context.create_buffer().unwrap();
    let attrib_color = context.get_attrib_location(&program, "a_color");
    let buffer_color = context.create_buffer().unwrap();
    let uniform_height = context.get_uniform_location(&program, "u_height").unwrap();
    let uniform_width = context.get_uniform_location(&program, "u_width").unwrap();
    let uniform_point_size = context
        .get_uniform_location(&program, "u_pointsize")
        .unwrap();
    context.uniform1f(Some(&uniform_height), width as f32);
    context.uniform1f(Some(&uniform_width), height as f32);

    // ランダム生成した浮動小数点値を1diskあたりに3値(rgb)割り当てる
    let mut random = rand::thread_rng();
    let color_buffer_array = (0..(DISK_NUM * 3))
        .into_iter()
        .map(|_| random.gen_range(0., 1.) as f32)
        .collect::<Vec<f32>>();
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer_color));
    unsafe {
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &js_sys::Float32Array::view(color_buffer_array.as_slice()), //
            WebGlRenderingContext::STREAM_DRAW,
        )
    }
    context.vertex_attrib_pointer_with_f64(
        attrib_color as u32,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0.,
    );

    Screen {
        iter: 0,
        gl: context,
        disks,
        disk_size,
        width,
        height,
        uniform_point_size,
        attrib_coords,
        buffer_coords,
        attrib_color,
        shot_type,
        shot_speed,
        shot_way_num,
        shot_interval,
        shot_behavior,
        speed_change_per,
        speed_change_interval,
        fps_counter: 0,
        fps_time: 0.,
        last_fps: 0,
    }
}

