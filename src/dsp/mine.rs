use super::{building::BuildingType, item::{Cargo, Resource}, recipe::{ Recipe, RecipeFmtInfo}};
use dspdb::item::ItemData;
use crate::dsp::item::ResourceType::Direct;

fn is_mine(item: &ItemData) -> bool {
    !item.mining_from.is_empty()
}

pub fn mines(raw_items: &dspdb::item::ItemProtoSet) -> Vec<Recipe> {
    let mut mines = Vec::new();
    for item in &raw_items.data_array {
        if is_mine(item) {
            let tmp = Recipe {
                items: Vec::new(),
                results: vec![Resource {
                    resource_type: Direct(Cargo {
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
