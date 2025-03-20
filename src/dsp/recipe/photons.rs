use super::Recipe;
use crate::dsp::{building::BuildingType, item::Resource, recipe::RecipeFmtInfo};

impl Recipe {
    #[must_use]
    pub fn photons() -> Vec<Self> {
        const 临界光子: i16 = 1208;
        const 引力透镜: i16 = 1209;
        const TIME: f64 = 2.5 * 60.0;
        const COST: f64 = 2.5 / (60.0 * 10.0);
        vec![Self {
            items: vec![Resource::from_item_level(引力透镜, 4, COST)],
            results: vec![Resource::from_item_level(临界光子, 0, 1.0)],
            time: TIME,
            info: RecipeFmtInfo {
                name: String::from("透镜光子"),
                building_type: BuildingType::锅盖,
                ..RecipeFmtInfo::default()
            },
        }]
    }
}
