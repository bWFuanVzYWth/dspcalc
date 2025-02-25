use super::{
    item::{IndirectResource::*, Item::*, Resource},
    recipe::Resource::{Direct, Indirect},
};

pub struct Cargo {
    pub resource: Resource,
    pub num: f64,
}

pub struct Recipe<'a> {
    pub resources: &'a [Cargo],
    pub products: &'a [Cargo],
    pub only_speed_up: bool,
}

pub const BASIC_RECIPES: &[Recipe] = &[
    Recipe {
        resources: &[
            Cargo {
                resource: Direct(煤矿),
                num: 2.0,
            },
            Cargo {
                resource: Indirect(Time),
                num: 2.0,
            },
        ],
        products: &[Cargo {
            resource: Direct(高能石墨),
            num: 1.0,
        }],
        only_speed_up: false,
    },
    Recipe {
        resources: &[
            Cargo {
                resource: Direct(精炼油),
                num: 1.0,
            },
            Cargo {
                resource: Direct(氢),
                num: 2.0,
            },
            Cargo {
                resource: Indirect(Time),
                num: 4.0,
            },
        ],
        products: &[
            Cargo {
                resource: Direct(氢),
                num: 3.0,
            },
            Cargo {
                resource: Direct(高能石墨),
                num: 1.0,
            },
        ],
        only_speed_up: true,
    },
    Recipe {
        resources: &[
            Cargo {
                resource: Direct(精炼油),
                num: 2.0,
            },
            Cargo {
                resource: Direct(氢),
                num: 1.0,
            },
            Cargo {
                resource: Direct(煤矿),
                num: 1.0,
            },
            Cargo {
                resource: Indirect(Time),
                num: 4.0,
            },
        ],
        products: &[Cargo {
            resource: Direct(精炼油),
            num: 3.0,
        }],
        only_speed_up: true,
    },
];
