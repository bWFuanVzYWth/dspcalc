use super::Recipe;
use crate::{
    dsp::{building::BuildingType, item::Resource, recipe::RecipeFmtInfo},
    unit_convert::{tick_from_min, tick_from_sec},
};

impl Recipe {
    #[must_use]
    pub fn photons() -> Vec<Self> {
        const 临界光子: i16 = 1208;
        const 引力透镜: i16 = 1209;
        vec![Self {
            items: vec![Resource::from_item_level(引力透镜, 4, 1.0)],
            results: vec![Resource::from_item_level(临界光子, 0, 240.0)],
            time: tick_from_min(10.0),
            info: RecipeFmtInfo {
                name: String::from("透镜光子"),
                building_type: BuildingType::锅盖,
                ..RecipeFmtInfo::default()
            },
        }]
    }
}
