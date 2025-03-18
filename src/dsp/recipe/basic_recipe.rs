use std::collections::HashMap;

use strum::IntoEnumIterator;

use dspdb::item::ItemData;
use dspdb::recipe::RecipeItem;

use crate::dsp::{
    building::BuildingType,
    item::{Cargo, Resource, ResourceType},
    proliferator::Proliferator,
};

use super::{ProliferatorType, Recipe, RecipeFmtInfo};

impl Recipe {
    fn create_recipe(
        recipe_item: &RecipeItem,
        level: u8,
        modify_result_num: impl Fn(f64) -> f64,
        modify_time: impl Fn(f64) -> f64,
        power_scale: f64,
        info: RecipeFmtInfo,
    ) -> Self {
        let power =
            Resource::power(BuildingType::from_recipe_item(recipe_item).power() * power_scale);
        let mut items: Vec<_> = recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, count)| Resource {
                resource_type: ResourceType::Direct(Cargo {
                    item_id: *item,
                    level,
                }),
                #[allow(clippy::cast_precision_loss)]
                num: *count as f64,
            })
            .collect();
        items.push(power);

        Self {
            items,
            results: recipe_item
                .results
                .iter()
                .zip(recipe_item.result_counts.iter())
                .map(|(res, count)| Resource {
                    resource_type: ResourceType::Direct(Cargo {
                        item_id: *res,
                        level: 0,
                    }),
                    #[allow(clippy::cast_precision_loss)]
                    num: modify_result_num(*count as f64),
                })
                .collect(),
            #[allow(clippy::cast_precision_loss)]
            time: modify_time(recipe_item.time_spend as f64)
                * BuildingType::from_recipe_item(recipe_item).time_scale(),
            info,
        }
    }

    fn accelerate(recipe_item: &RecipeItem, level: u8) -> Self {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            proliferator_type: Some(ProliferatorType {
                level,
                is_speed_up: true,
            }),
            building_type: BuildingType::from_recipe_item(recipe_item),
        };
        Self::create_recipe(
            recipe_item,
            level,
            |num| num,
            |time| time / Proliferator::accelerate(level),
            Proliferator::power(level),
            info,
        )
    }

    fn productive(recipe_item: &RecipeItem, level: u8) -> Self {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            proliferator_type: Some(ProliferatorType {
                level,
                is_speed_up: false,
            }),
            building_type: BuildingType::from_recipe_item(recipe_item),
        };
        Self::create_recipe(
            recipe_item,
            level,
            |num| num * Proliferator::increase(level),
            |time| time,
            Proliferator::power(level),
            info,
        )
    }

    pub fn recipes_accelerate(recipes: &mut Vec<Self>, recipe_item: &RecipeItem, cocktail: bool) {
        if cocktail {
            for level in 1..=Proliferator::MAX_INC_LEVEL {
                recipes.push(Self::accelerate(recipe_item, level));
            }
        } else {
            Proliferator::iter().for_each(|proliferator| {
                recipes.push(Self::accelerate(recipe_item, proliferator.inc_level()));
            });
        }
    }

    // 预处理物品增产支持信息
    fn build_productive_map(items: &[ItemData]) -> HashMap<i16, bool> {
        items.iter().map(|i| (i.id, i.productive)).collect()
    }

    fn recipe_can_be_productive(
        recipe_item: &RecipeItem,
        productive_map: &HashMap<i16, bool>,
    ) -> bool {
        if recipe_item.non_productive {
            return false;
        }

        recipe_item
            .items
            .iter()
            .all(|id| productive_map.get(id).copied().unwrap_or(false))
    }

    pub fn recipes_productive(
        recipes: &mut Vec<Self>,
        recipe_item: &RecipeItem,
        items: &[ItemData],
        cocktail: bool,
    ) {
        let productive_map = Self::build_productive_map(items);
        if Self::recipe_can_be_productive(recipe_item, &productive_map) {
            if cocktail {
                for level in 1..=Proliferator::MAX_INC_LEVEL {
                    recipes.push(Self::productive(recipe_item, level));
                }
            } else {
                Proliferator::iter().for_each(|proliferator| {
                    recipes.push(Self::productive(recipe_item, proliferator.inc_level()));
                });
            }
        }
    }

    pub fn recipe_vanilla(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            proliferator_type: Some(ProliferatorType {
                level: 0,
                is_speed_up: false,
            }),
            building_type: BuildingType::from_recipe_item(recipe_item),
        };
        recipes.push(Self::create_recipe(
            recipe_item,
            0,
            |num| num,
            |time| time,
            Proliferator::power(0),
            info,
        ));
    }
}
