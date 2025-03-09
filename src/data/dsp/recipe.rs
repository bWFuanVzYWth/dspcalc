use crate::{data::dsp::item::Cargo, solver::proliferator::Proliferator};
use dspdb::recipe::RecipeItem;

use super::item::{Resource, ResourceType::Direct};

#[derive(Clone, Debug)]
pub struct Recipe {
    pub items: Vec<Resource>,
    pub results: Vec<Resource>,
    pub time: f64,
}

fn create_recipe(
    recipe_item: &RecipeItem,
    level: usize,
    modify_result_num: impl Fn(f64) -> f64,
    modify_time: impl Fn(f64) -> f64,
) -> Recipe {
    Recipe {
        items: recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *item,
                    level,
                }),
                num: *count as f64,
            })
            .collect(),
        results: recipe_item
            .results
            .iter()
            .zip(recipe_item.result_counts.iter())
            .map(|(res, count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *res,
                    level: 0,
                }),
                num: modify_result_num(*count as f64),
            })
            .collect(),
        time: modify_time(recipe_item.time_spend as f64),
    }
}

fn accelerate(recipe_item: &RecipeItem, level: usize) -> Recipe {
    create_recipe(
        recipe_item,
        level,
        |num| num,
        |time| time / Proliferator::accelerate(level),
    )
}

fn productive(recipe_item: &RecipeItem, level: usize) -> Recipe {
    create_recipe(
        recipe_item,
        level,
        |num| num * Proliferator::increase(level),
        |time| time,
    )
}

const MK3_INC_LEVEL: usize = Proliferator::inc_level(&Proliferator::MK3);

fn recipes_accelerate(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    for level in 1..=MK3_INC_LEVEL {
        recipes.push(accelerate(recipe_item, level));
    }
}

fn recipes_productive(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    if !recipe_item.non_productive {
        for level in 1..=MK3_INC_LEVEL {
            recipes.push(productive(recipe_item, level));
        }
    }
}

// TODO 耗电量

fn recipe_vanilla(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    recipes.push(Recipe {
        items: recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, item_count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *item,
                    level: 0,
                }),
                num: *item_count as f64,
            })
            .collect(),

        results: recipe_item
            .results
            .iter()
            .zip(recipe_item.result_counts.iter())
            .map(|(result, result_count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *result,
                    level: 0,
                }),
                num: *result_count as f64,
            })
            .collect(),

        time: recipe_item.time_spend as f64,
    });
}

#[must_use]
pub fn flatten_recipes(basic_recipes: &[RecipeItem]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    for recipe_item in basic_recipes.iter() {
        recipe_vanilla(&mut recipes, recipe_item);
        recipes_productive(&mut recipes, recipe_item);
        recipes_accelerate(&mut recipes, recipe_item);
    }
    recipes
}
