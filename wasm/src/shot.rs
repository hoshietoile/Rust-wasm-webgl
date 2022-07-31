#[derive(Debug, Clone)]
pub enum ShotType {
    Random,
    Circle,
    Linear,
}

pub fn resolve_shot_type(num: u32) -> ShotType {
    match num {
        1 => ShotType::Circle,
        2 => ShotType::Linear,
        _ | 0 => ShotType::Random,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ShotBehavior {
    Normal,
    SpeedUp(f64, f64),   // 加速率(%)/加速イテレーション(per, by)
    SpeedDown(f64, f64), // 減速率(%)/減速イテレーション(per, by)
    Reflect(Option<u32>), // 画面反射(rest-reflect-count)
    Random, // ランダム角度
    Sleep(i32, i32), // スリープ弾(interval, timeout)
    Gravity(u32, f64), // 重力弾(向き0|1|2|3, 変化量)
}

pub fn resolve_shot_behavior(num: u32) -> ShotBehavior {
    match num {
        1 => ShotBehavior::SpeedUp(0., 0.),
        2 => ShotBehavior::SpeedDown(0., 0.),
        3 => ShotBehavior::Reflect(None),
        4 => ShotBehavior::Random,
        5 => ShotBehavior::Sleep(0, 0),
        6 => ShotBehavior::Gravity(0, 0.1),
        _ | 0 => ShotBehavior::Normal,
    }
}