#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum IndirectResource {
    Power,
    Area,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub item_id: i16,
    pub level: u8,
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
    #[must_use]
    pub const fn from_item_level(item_id: i16, level: u8, num: f64) -> Self {
        Self {
            resource_type: ResourceType::Direct(Cargo { item_id, level }),
            num,
        }
    }

    #[must_use]
    pub const fn area(num: f64) -> Self {
        Self {
            resource_type: ResourceType::Indirect(IndirectResource::Area),
            num,
        }
    }

    #[must_use]
    pub const fn power(num: f64) -> Self {
        Self {
            resource_type: ResourceType::Indirect(IndirectResource::Power),
            num,
        }
    }
}
