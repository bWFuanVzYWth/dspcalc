#[must_use]
pub const fn in_sec(tick: f64) -> f64 {
    tick / 60.0
}

#[must_use]
pub const fn in_min(tick: f64) -> f64 {
    tick / 3600.0
}
