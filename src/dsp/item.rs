use dspdb::item::ItemData;

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

// TODO 这个应该丢到dspdb里去
#[must_use]
pub fn item_name(item_id: i16, items: &[ItemData]) -> String {
    items.iter().find(|item| item.id == item_id).map_or_else(
        || format!("Unknown Item {item_id}"),
        |item| item.name.clone(),
    )
}
