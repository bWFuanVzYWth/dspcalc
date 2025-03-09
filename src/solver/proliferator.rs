#[derive(Clone)]
pub enum 增产剂 {
    MK1,
    MK2,
    MK3,
}

const INC_LEVEL_MAX: usize = 10;

const INC_TABLE: [f64; INC_LEVEL_MAX + 1] = [
    0.0, 0.125, 0.2, 0.225, 0.25, 0.275, 0.3, 0.325, 0.35, 0.375, 0.4,
];

const ACC_TABLE: [f64; INC_LEVEL_MAX + 1] =
    [0.0, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5];

const POWER_TABLE: [f64; INC_LEVEL_MAX + 1] =
    [1.0, 1.3, 1.7, 2.1, 2.5, 2.9, 3.3, 3.7, 4.1, 4.5, 4.9];

impl 增产剂 {
    pub const fn item_id(t: &Self) -> i16 {
        match t {
            增产剂::MK1 => 1141,
            增产剂::MK2 => 1142,
            增产剂::MK3 => 1143,
        }
    }

    pub const fn inc_level(t: &Self) -> u64 {
        match t {
            增产剂::MK1 => 1,
            增产剂::MK2 => 2,
            增产剂::MK3 => 4,
        }
    }

    pub const fn life(t: &Self, level: usize) -> usize {
        (Self::increase(level)
            * match t {
                增产剂::MK1 => 12.0,
                增产剂::MK2 => 24.0,
                增产剂::MK3 => 60.0,
            }) as usize
    }

    pub const fn increase(level: usize) -> f64 {
        1.0 + INC_TABLE[level]
    }

    pub const fn accelerate(level: usize) -> f64 {
        1.0 + ACC_TABLE[level]
    }

    pub const fn power(level: usize) -> f64 {
        POWER_TABLE[level]
    }
}
