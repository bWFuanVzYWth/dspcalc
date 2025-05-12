use super::Recipe;
use crate::{
    dsp::{building::BuildingType, item::Resource, recipe::RecipeFmtInfo},
    unit_convert::tick_from_sec,
};

impl Recipe {
    #[must_use]
    pub fn powers() -> Vec<Self> {
        const 金棒: i16 = 1804;
        vec![Self {
            items: vec![Resource::from_item_level(金棒, 4, 1.0)],
            results: vec![Resource::energy(72_000.0)],
            time: tick_from_sec(250.0),
            info: RecipeFmtInfo {
                name: String::from("小太阳烧lv4黄棒"),
                building_type: BuildingType::小太阳,
                ..RecipeFmtInfo::default()
            },
        }]
    }
}
