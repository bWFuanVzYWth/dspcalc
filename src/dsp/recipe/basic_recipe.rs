use dspdb::recipe::RecipeItem;

use crate::dsp::{
    building::BuildingType,
    item::{Cargo, Resource, ResourceType},
    proliferator::Proliferator,
};

use super::{Recipe, RecipeFmtInfo};

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
                    num: modify_result_num(*count as f64),
                })
                .collect(),
            time: modify_time(recipe_item.time_spend as f64)
                * BuildingType::from_recipe_item(recipe_item).time_scale(),
            info,
        }
    }

    fn accelerate(recipe_item: &RecipeItem, level: u8) -> Self {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            level,
            speed_up: true,
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
            level,
            speed_up: false,
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

    pub fn recipes_accelerate(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        for level in 1..=Proliferator::MAX_INC_LEVEL {
            recipes.push(Self::accelerate(recipe_item, level));
        }
    }

    pub fn recipes_productive(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        if !recipe_item.non_productive {
            for level in 1..=Proliferator::MAX_INC_LEVEL {
                recipes.push(Self::productive(recipe_item, level));
            }
        }
    }

    pub fn recipe_vanilla(recipes: &mut Vec<Self>, recipe_item: &RecipeItem) {
        let info = RecipeFmtInfo {
            name: recipe_item.name.clone(),
            level: 0,
            speed_up: false,
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
