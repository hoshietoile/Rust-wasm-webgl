mod dom_utils;
mod utils;
mod vec2d;
mod rectanble;

mod disk;
mod shot;
mod setting;
mod schedule;
mod event;
mod event_thread;

use setting::Setting;
use shot::ShotBehavior;
use vec2d::Vec2d;
// use rand::Rng;
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;
use web_sys::{WebGlBuffer, WebGlRenderingContext, WebGlUniformLocation, HtmlImageElement, CanvasRenderingContext2d};
// use vec2d::{Vec2d};
use disk::{ Disk, DiskType, DiskColor };
use schedule::{ Schedule };
use event_thread::{ EventThread };

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn output_log(s: &str) {
    log(&format!("Hello {}", s));
}

const FRAMES_PER_SEC: u32 = 60;
const MILLI_SECONDS: u32 = 1_000;
const DISK_NUM: u32 = 4_096;

/**
 * 定数をもとにインターバル(ms)をフレーム数に変換する
 * ex) 500ms -> 30fr
 *     200ms -> 12fr
 */
pub fn convert_interval_to_frame(interval: u32) -> f64 {
    (FRAMES_PER_SEC as f64) * ((interval as f64) / (MILLI_SECONDS as f64))
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct Screen {
    context: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    theme: u32,

    sprite_sheet: HtmlImageElement,

    disks: Vec<Option<Disk>>,
    schedule: Schedule,

    fps_counter: u32,
    fps_time: f64,
    last_fps: u32,
}

type ThreadId = u32;

#[wasm_bindgen]
impl Screen {
    /**
     * アニメーション実行
     */
    pub fn do_frame(&mut self, time: f64) -> () {
        self.on_animation_frame(time);
        self.draw();
    }

    /**
     * ThreadID一覧
     */
    pub fn get_thread_ids(&self) -> Vec<ThreadId> {
        self.schedule.threads
            .iter()
            .map(|thread| thread.id)
            .collect()
    }
    
    /**
     * settings作成/更新
     */
    pub fn upsert_thread_setting(&mut self, thread_id: Option<u32>, option_input: JsValue) -> Option<ThreadId> {
        log!("options {:?}", option_input); 
        let options: setting::SettingOptions = option_input.into_serde().unwrap();
        let setting = setting::Setting::new(&options);
        let thread = match thread_id {
            Some(id) => {
                self.schedule
                    .threads
                    .iter()
                    .find(|thread| thread.id == id)
                    .map(|thread| {
                        let mut cloned = thread.clone();
                        cloned.update_setting(setting);
                        cloned 
                    })
            },
            None => {
                let thread_id = self.schedule.generate_id();
                Some(
                    EventThread::new(
                        thread_id,
                        setting,
                    )
                )
            },
        }?;
        let thread_id = thread.id;
        self.schedule.subscribe_thread(thread);
        self.schedule.refresh_events();
        Some(thread_id)
    }
}

impl Screen {
    /**
     * 各アニメーションフレームごとの処理
     */
    fn on_animation_frame(&mut self, time: f64) -> () {
        self.schedule.iterate();

        /* スケジュールされたイベントの走査 */
        let mut disks: &mut Vec<Option<Disk>> = self.disks.as_mut();
        self.schedule.walkthrough_events(&mut disks);

        /* Diskのステータスで座標更新 */
        self.update_disks();

        /* fps更新 */
        self.calc_fps(time);
    }

    /**
     * 反射時処理
     */
    fn on_reflect(&self, disk: Disk) -> Option<Disk> {
        let mut v = disk.clone();
        let size = v.disk_size;
        let width = self.width;
        let height = self.height;
        let should_reflect = v.reflect_count.unwrap_or(0) > 0;
        let reflect_behavior = v.behavior
            .iter()
            .find(|&sb| match sb {
                ShotBehavior::Reflect(_) => true,
                _ => false,
            });
        match (should_reflect, reflect_behavior) {
            (true, Some(ShotBehavior::Reflect(Some(_)))) => {
                // X軸方向
                if v.x - size < 0. || v.x + size > width {
                    v.x -= v.vec2d.x;
                    v.vec2d.x = -v.vec2d.x;
                    v.reflect_count = v.reflect_count.map(|num| num - 1);
                }
                // Y軸方向
                if v.y - size < 0. || v.y + size > height {
                    v.y -= v.vec2d.y;
                    v.vec2d.y = -v.vec2d.y;
                    v.reflect_count = v.reflect_count.map(|num| num - 1);
                }
                Some(v)
            }
            _ => {
                // 通常弾の場合
                if v.x + size < 0. || v.x - size > width || v.y + size < 0. || v.y - size > height {
                    return None;
                }
                Some(v)
            }
        }
    }

    /**
     * 登録しているDiskのステータスに従って座標を更新
     */
    fn update_disks(&mut self) {
        self.disks = self.disks
            .clone()
            .into_iter()
            .map(|disk| {
                match disk {
                    Some(mut v) => {
                        v.gain_age(1);

                        // スリープ制御
                        // TODO: ShotBehavior用の解析関数作る
                        v.clone().behavior
                            .iter()
                            .for_each(|&sb| {
                                match sb {
                                    // スリープ
                                    ShotBehavior::Sleep(interval, timeout) => {
                                        v.sleep_time += {
                                            if interval == 0 {
                                                0
                                            } else if v.age % (interval as u32) == 0 {
                                                timeout
                                            } else if v.sleep_time > 0 {
                                                -1
                                            } else {
                                                0
                                            }
                                        }
                                    },
                                    ShotBehavior::SpeedDown(_, per) => {
                                        v.speed -= v.speed * per; 
                                        v.vec2d = Vec2d::new(v.angle, v.speed);
                                    },
                                    ShotBehavior::SpeedUp(_, per) => {
                                        v.speed += v.speed * per; 
                                        v.vec2d = Vec2d::new(v.angle, v.speed);
                                    },
                                    // 重力減衰/加速
                                    ShotBehavior::Gravity(direction, by) => {
                                        let angle = std::f64::consts::PI * (90. * direction as f64) / 180.;
                                        let vec2d = Vec2d::new(angle, v.speed * by);
                                        v.vec2d = v.vec2d + vec2d;
                                    }
                                    _ => (),
                                }
                            });
                        
                        if v.sleep_time > 0 {
                            return Some(v);
                        }

                        v.x += v.vec2d.x;
                        v.y += v.vec2d.y;

                        let d = self.on_reflect(v);
                        d
                    },
                    _ => None,
                }
            })
            .collect::<Vec<Option<Disk>>>();
    }

    fn resolve_sprite_src(&self, disk_type: &DiskType, disk_color: &DiskColor) -> (f64, f64, f64, f64) {
        let casted_disk_color = (*disk_color as usize) as f64;
        match disk_type {
            DiskType::Oval => (0. + (casted_disk_color * 10.), 0., 10., 10.),
            DiskType::Dot => (2. + (casted_disk_color * 24.), 12., 18., 18.),
            DiskType::Circle => (2. + (casted_disk_color * 24.), 37., 18., 18.),
            DiskType::Orb => (2. + (casted_disk_color * 30.), 61., 24., 24.),
            DiskType::Arrow => (2., 89., 61., 61.),
        }
    }

    /**
     * レンダリング処理
     */
    fn draw(&self) {
        self.context.save();
        let bg_color = if self.theme == 0 { "rgb(200, 200, 200, 1.0)" } else { "rgb(80, 80, 80, 1.0)" };
        self.context.set_fill_style(&JsValue::from(bg_color));
        self.context.fill_rect(0., 0., self.width as f64, self.height as f64); 

        for (_, disk) in self.disks.iter().enumerate() {
            match disk {
                Some(d) => {
                    let sprite = self.resolve_sprite_src(&d.disk_type, &d.disk_color);
                    self.context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        &self.sprite_sheet,
                        sprite.0,
                        sprite.1,
                        sprite.2,
                        sprite.3,
                        (d.x as f64) - (d.disk_size / 2.),
                        (d.y as f64) - (d.disk_size / 2.),
                        d.disk_size,
                        d.disk_size, 
                    ).unwrap();
                },
                _ => {
                    continue;
                }
            }
        }

        // TODO: モニタ部分をカスタムしやすいように別関数&座標計算入れるようにしたい
        // アクティブ段数
        let active_disk_count = self.disks
            .iter()
            .filter_map(|disk| {
                match disk {
                    Some(_) => Some(1),
                    _ => None,
                }
            })
            .count();

        self.context.set_fill_style(&JsValue::from("rgba(0, 0, 0, 0.5)"));
        self.context.fill_rect(10., 730., 180., 45.);
        self.context.set_fill_style(&JsValue::from("rgb(255, 255, 255)"));
        self.context.set_stroke_style(&JsValue::from("rgb(255, 255, 255)"));
        self.context.set_font("16px sans-serif");
        self.context.fill_text(&format!("FPS: {}", self.last_fps), 15., 750.);
        self.context.fill_text(&format!("アクティブ弾数: {}", active_disk_count), 15., 770.);

        self.context.restore();
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
}

/**
 * ディスクのベクタを初期化する
 */
fn init_disks(disk_num: u32) -> Vec<Option<Disk>> {
    let mut disks_buffer: Vec<Option<Disk>> = Vec::with_capacity(disk_num as usize);
    for _ in 0..disk_num {
        disks_buffer.push(None);
    }
    disks_buffer
}

#[wasm_bindgen]
pub fn init_screen(option_input: JsValue) -> Screen {
    log!("options {:?}", option_input); 
    let options: setting::SettingOptions = option_input.into_serde().unwrap();
    let setting = setting::Setting::new(&options);

    // Screen情報
    let canvas_id = options.canvas_id;
    let width = options.width;
    let height = options.height;
    let theme = options.theme;

    // TODO: DOM操作系はResultsを返すようにしてエラーをキャッチしたい
    let context = dom_utils::get_context2d_by_id(canvas_id.as_str(), width, height).unwrap();

    // Disks初期化
    let disks = init_disks(DISK_NUM);

    // Scheduleの初期化と最初のEventThreadを登録
    let mut schedule = Schedule::new();
    let thread_id = schedule.generate_id();
    let thread = EventThread::new(
        thread_id,
        setting,
    );
    schedule.subscribe_thread(thread);
    schedule.refresh_events();

    // 弾のスプライト取得
    let img = dom_utils::image("img-src").unwrap();

    Screen {
        width,
        height,
        theme,
        context,

        disks,
        schedule,

        sprite_sheet: img,

        // fps
        fps_counter: 0,
        fps_time: 0.,
        last_fps: 0,
    }
}
