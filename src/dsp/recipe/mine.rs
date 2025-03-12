use dspdb::item::ItemData;

use crate::dsp::{
    building::BuildingType,
    item::{Cargo, Resource, ResourceType},
};

use super::{Recipe, RecipeFmtInfo};

impl Recipe {
    #[must_use]
    pub fn mines(raw_items: &dspdb::item::ItemProtoSet) -> Vec<Self> {
        let mut mines = Vec::new();
        for item in &raw_items.data_array {
            let is_mine = |item: &ItemData| !item.mining_from.is_empty();
            if is_mine(item) {
                let tmp = Self {
                    items: Vec::new(),
                    results: vec![Resource {
                        resource_type: ResourceType::Direct(Cargo {
                            item_id: item.id,
                            level: 0,
                        }),
                        num: 4.0, // TODO 根据采矿等级设置成本，或者增加原矿化标记字段，不计成本
                    }],
                    time: 1.0,
                    info: RecipeFmtInfo {
                        name: "采矿".to_string(),
                        building_type: BuildingType::矿机,
                        ..RecipeFmtInfo::default()
                    },
                };
                mines.push(tmp);
            }
        }
        mines
    }
}
