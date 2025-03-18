use crate::dsp::{building::BuildingType, item::Resource, recipe::RecipeFmtInfo};
use super::Recipe;

impl Recipe {
    #[must_use]
    pub fn powers() -> Vec<Self> {
        const 金棒: i16 = 1804;
        const INV_TIME: f64 = 0.288 / 72.0;
        vec![Self {
            items: vec![Resource::from_item_level(金棒, 4, INV_TIME)],
            results: vec![Resource::power(BuildingType::小太阳.power())],
            time: 1.0,
            info: RecipeFmtInfo {
                name: String::from("小太阳烧lv4黄棒"),
                building_type: BuildingType::小太阳,
                ..RecipeFmtInfo::default()
            },
        }]
    }
}
