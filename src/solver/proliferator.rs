#[derive(Clone)]
pub enum 增产剂 {
    MK1,
    MK2,
    MK3,
}

impl 增产剂 {
    pub const fn item_id(t: &Self) -> i16 {
        match t {
            增产剂::MK1 => 1141,
            增产剂::MK2 => 1142,
            增产剂::MK3 => 1143,
        }
    }

    pub const fn point(t: &Self) -> u64 {
        match t {
            增产剂::MK1 => 1,
            增产剂::MK2 => 2,
            增产剂::MK3 => 4,
        }
    }

    pub const fn life(t: &Self, point: u64) -> u64 {
        (Self::extra(point)
            * match t {
                增产剂::MK1 => 12.0,
                增产剂::MK2 => 24.0,
                增产剂::MK3 => 60.0,
            }) as u64
    }

    pub const fn extra(point: u64) -> f64 {
        match point {
            1 => 1.125,
            2 => 1.2,
            3 => 1.225,
            4 => 1.25,
            _ => 1.0,
        }
    }

    pub const fn speed_up(point: u64) -> f64 {
        match point {
            1 => 1.25,
            2 => 1.5,
            3 => 1.75,
            4 => 2.0,
            _ => 1.0,
        }
    }

    pub const fn power(point: u64) -> f64 {
        match point {
            1 => 1.3,
            2 => 1.7,
            3 => 2.1,
            4 => 2.5,
            _ => 1.0,
        }
    }
}
