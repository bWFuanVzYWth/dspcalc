use std::collections::HashMap;

use strum::IntoEnumIterator;

use dspdb::item::ItemData;
use dspdb::recipe::RecipeItem;

use super::{ProliferatorType, Recipe, RecipeFmtInfo};
use crate::{
    dsp::{building::BuildingType, item::Resource, proliferator::Proliferator},
    error::DspCalError,
};

impl Recipe {
    fn create_recipe(
        recipe_item: &RecipeItem,
        items_level: u8,
        modify_result_num: impl Fn(f64) -> f64,
        modify_time: impl Fn(f64) -> f64,
        power_scale: f64,
        info: RecipeFmtInfo,
    ) -> Result<Self, DspCalError> {
        // 确实存在i64转f64丢失精度的风险，但是几乎不可能发生，除非出现了非常魔怔的数值，通常是无损的

        let time = modify_time(recipe_item.time_spend as f64)
            * get_building_type(recipe_item)?.time_scale();

        let energy = Resource::energy(get_building_type(recipe_item)?.power() * power_scale * time);

        let items: Vec<_> = recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, count)| Resource::from_item_level(*item, items_level, *count as f64))
            .chain(std::iter::once(energy))
            .collect();

        let results = recipe_item
            .results
            .iter()
            .zip(recipe_item.result_counts.iter())
            .map(|(result, count)| {
                Resource::from_item_level(*result, 0, modify_result_num(*count as f64))
            })
            .collect();

        Ok(Self {
            items,
            results,
            time,
            info,
        })
    }

    fn accelerate(recipe_item: &RecipeItem, items_level: u8) -> Result<Self, DspCalError> {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            proliferator_type: Some(ProliferatorType {
                level: items_level,
                is_speed_up: true,
            }),
            building_type: get_building_type(recipe_item)?,
        };
        Self::create_recipe(
            recipe_item,
            items_level,
            |num| num,
            |time| time / Proliferator::accelerate(items_level),
            Proliferator::power(items_level),
            info,
        )
    }

    fn productive(recipe_item: &RecipeItem, items_level: u8) -> Result<Self, DspCalError> {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            proliferator_type: Some(ProliferatorType {
                level: items_level,
                is_speed_up: false,
            }),
            building_type: get_building_type(recipe_item)?,
        };
        Self::create_recipe(
            recipe_item,
            items_level,
            |num| num * Proliferator::increase(items_level),
            |time| time,
            Proliferator::power(items_level),
            info,
        )
    }

    /// # Errors
    /// 如果配方的建筑类型未定义则返回错误
    pub fn recipes_accelerate(
        recipes: &mut Vec<Self>,
        recipe_item: &RecipeItem,
        cocktail: bool,
    ) -> Result<(), DspCalError> {
        if cocktail {
            for items_level in 1..=Proliferator::MAX_INC_LEVEL {
                recipes.push(Self::accelerate(recipe_item, items_level)?);
            }
        } else {
            for proliferator in Proliferator::iter() {
                recipes.push(Self::accelerate(recipe_item, proliferator.inc_level())?);
            }
        }
        Ok(())
    }

    // 预处理物品增产支持信息
    fn build_productive_map(items: &[ItemData]) -> HashMap<i16, bool> {
        items.iter().map(|i| (i.id, i.productive)).collect()
    }

    fn recipe_can_be_productive(
        recipe_item: &RecipeItem,
        productive_map: &HashMap<i16, bool>,
    ) -> Result<bool, DspCalError> {
        // 如果配方标记了不可增产，返回 false
        if recipe_item.non_productive {
            return Ok(false);
        }

        // 如果配方中存在某个原料标记了不可增产，返回 false
        for item_id in &recipe_item.items {
            if !*productive_map
                .get(item_id)
                .ok_or(DspCalError::UnknownItemId(*item_id))?
            {
                return Ok(false);
            }
        }

        // 剩下的都是可增产
        Ok(true)
    }

    /// # Errors
    /// 如果配方的建筑类型未定义则返回错误
    pub fn recipes_productive(
        recipes: &mut Vec<Self>,
        recipe_item: &RecipeItem,
        items: &[ItemData],
        cocktail: bool,
    ) -> Result<(), DspCalError> {
        let productive_map = Self::build_productive_map(items);
        if Self::recipe_can_be_productive(recipe_item, &productive_map)? {
            if cocktail {
                for items_level in 1..=Proliferator::MAX_INC_LEVEL {
                    recipes.push(Self::productive(recipe_item, items_level)?);
                }
            } else {
                for proliferator in Proliferator::iter() {
                    recipes.push(Self::productive(recipe_item, proliferator.inc_level())?);
                }
            }
        }

        Ok(())
    }

    /// # Errors
    /// 如果配方的建筑类型未定义则返回错误
    pub fn recipe_vanilla(
        recipes: &mut Vec<Self>,
        recipe_item: &RecipeItem,
    ) -> Result<(), DspCalError> {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            proliferator_type: Some(ProliferatorType {
                level: 0,
                is_speed_up: false,
            }),
            building_type: get_building_type(recipe_item)?,
        };
        recipes.push(Self::create_recipe(
            recipe_item,
            0,
            |num| num,
            |time| time,
            Proliferator::power(0),
            info,
        )?);

        Ok(())
    }
}

fn get_building_type(recipe_item: &RecipeItem) -> Result<BuildingType, DspCalError> {
    BuildingType::from_recipe_item(recipe_item)
        .ok_or(DspCalError::UnknownBuildingType(recipe_item.type_))
}
