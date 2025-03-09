#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum IndirectResource {
    Power,
    Area,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Cargo {
    pub item_id: i16,
    pub level: usize,
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
    pub const fn from_item_level(item_id: i16, level: usize, num: f64) -> Self {
        Resource {
            resource_type: ResourceType::Direct(Cargo { item_id, level }),
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
