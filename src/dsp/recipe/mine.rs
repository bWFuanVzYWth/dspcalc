use dspdb::item::ItemData;

use super::{Recipe, RecipeFmtInfo};
use crate::dsp::{building::BuildingType, item::Resource};

// TODO 根据采矿等级设置num
// FIXME 耗电量计算不正确
impl Recipe {
    #[must_use]
    pub fn mines(items: &[dspdb::item::ItemData]) -> Vec<Self> {
        let mut mines = Vec::new();
        for item in items {
            let is_mine = |test_item: &ItemData| !test_item.mining_from.is_empty();
            if is_mine(item) {
                let tmp = Self {
                    items: vec![Resource::power(BuildingType::矿机.power())],
                    results: vec![Resource::from_item_level(item.id, 0, 9.0 * 4.0)],
                    time: 1.0,
                    info: RecipeFmtInfo {
                        name: String::from("采矿"),
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
