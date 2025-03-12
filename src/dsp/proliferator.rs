#[derive(Debug, Clone)]
pub enum Proliferator {
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

impl Proliferator {
    pub const MAX_INC_LEVEL: u8 = Self::inc_level(&Self::MK3);

    #[must_use]
    pub const fn item_id(t: &Self) -> i16 {
        match t {
            Self::MK1 => 1141,
            Self::MK2 => 1142,
            Self::MK3 => 1143,
        }
    }

    #[must_use]
    pub const fn inc_level(t: &Self) -> u8 {
        match t {
            Self::MK1 => 1,
            Self::MK2 => 2,
            Self::MK3 => 4,
        }
    }

    #[must_use]
    pub const fn life(t: &Self, level: usize) -> usize {
        (Self::increase(level)
            * match t {
                Self::MK1 => 12.0,
                Self::MK2 => 24.0,
                Self::MK3 => 60.0,
            }) as usize
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub const fn increase(level: usize) -> f64 {
        let index = if level < INC_LEVEL_MAX {
            level
        } else {
            INC_LEVEL_MAX
        };
        1.0 + INC_TABLE[index]
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub const fn accelerate(level: usize) -> f64 {
        let index = if level < INC_LEVEL_MAX {
            level
        } else {
            INC_LEVEL_MAX
        };
        1.0 + ACC_TABLE[index]
    }

    #[must_use]
    #[allow(clippy::indexing_slicing)]
    pub const fn power(level: usize) -> f64 {
        let index = if level < INC_LEVEL_MAX {
            level
        } else {
            INC_LEVEL_MAX
        };
        POWER_TABLE[index]
    }
}
