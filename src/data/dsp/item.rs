use strum_macros::{Display, EnumCount, EnumIter, EnumString};
use dspdb::item;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum IndirectResource {
    Power,
    Area,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub item_id: i16,
    pub point: u64,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ResourceType {
    Direct(Cargo),
    Indirect(IndirectResource),
}

#[derive(Copy, Clone, Debug)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub num: f64,
}

impl Resource {
    pub const fn from_item(item_id: i16, num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Direct(Cargo { item_id, point: 0 }),
            num,
        }
    }

    pub const fn from_item_point(item_id: i16, point: u64, num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Direct(Cargo { item_id, point }),
            num,
        }
    }

    pub const fn area(num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Indirect(IndirectResource::Area),
            num,
        }
    }

    pub const fn power(num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Indirect(IndirectResource::Power),
            num,
        }
    }
}
