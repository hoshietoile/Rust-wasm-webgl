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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShotBehavior {
    Normal,
    SpeedUp,   // 加速率(%)/加速イテレーション
    SpeedDown, // 減速率(%)/減速イテレーション
    Reflect, // 画面反射
    Random, // ランダム角度
    Sleep(i32, i32), // スリープ弾(interval, timeout)
}

pub fn resolve_shot_behavior(num: u32) -> ShotBehavior {
    match num {
        1 => ShotBehavior::SpeedUp,
        2 => ShotBehavior::SpeedDown,
        3 => ShotBehavior::Reflect,
        4 => ShotBehavior::Random,
        5 => ShotBehavior::Sleep(0, 0),
        _ | 0 => ShotBehavior::Normal,
    }
}