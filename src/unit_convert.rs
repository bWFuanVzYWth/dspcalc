/// sec -> tick
#[must_use]
pub const fn sec_from_tick(tick: f64) -> f64 {
    tick / 60.0
}

/// min -> tick
#[must_use]
pub const fn min_from_tick(tick: f64) -> f64 {
    tick / 3600.0
}

/// tick -> sec
#[must_use]
pub const fn tick_from_sec(sec: f64) -> f64 {
    sec * 60.0
}

/// min -> tick
#[must_use]
pub const fn tick_from_min(min: f64) -> f64 {
    min * 3600.0
}
