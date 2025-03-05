use crate::data::dsp::item::{Cargo, IndirectResource};
use dspdb::recipe::{self, RecipeItem};

use super::item::{
    Resource,
    ResourceType::{Direct, Indirect},
};

#[derive(Clone, Debug)]
pub struct Recipe {
    pub items: Vec<Resource>,
    pub results: Vec<Resource>,
    pub time: f64,
}

// TODO 看看游戏源代码，检查是否有更优雅的写法
const fn speed_up_scale(point: u64) -> f64 {
    match point {
        1 => 1.0 / 1.25,
        2 => 1.0 / 1.5,
        3 => 1.0 / 1.75,
        4 => 1.0 / 2.0,
        _ => 1.0,
    }
}

fn speed_up(recipe_item: &RecipeItem, point: u64) -> Recipe {
    Recipe {
        items: recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, item_count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *item,
                    point,
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
                    point: 0,
                }),
                num: *result_count as f64,
            })
            .collect(),

        time: recipe_item.time_spend as f64,
    }
}

fn recipes_speed_up(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    recipes.push(speed_up(recipe_item, 1));
    recipes.push(speed_up(recipe_item, 2));
    recipes.push(speed_up(recipe_item, 3));
    recipes.push(speed_up(recipe_item, 4));
}

const fn productive_scale(point: u64) -> f64 {
    match point {
        1 => 1.125,
        2 => 1.2,
        3 => 1.225,
        4 => 1.25,
        _ => 1.0,
    }
}

// TODO 耗电量

fn productive(recipe_item: &RecipeItem, point: u64) -> Recipe {
    Recipe {
        items: recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, item_count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *item,
                    point,
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
                    point: 0,
                }),
                num: *result_count as f64 * productive_scale(point),
            })
            .collect(),

        time: recipe_item.time_spend as f64,
    }
}

fn recipes_productive(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    if recipe_item.non_productive == false {
        recipes.push(productive(recipe_item, 1));
        recipes.push(productive(recipe_item, 2));
        recipes.push(productive(recipe_item, 3));
        recipes.push(productive(recipe_item, 4));
    }
}

fn recipe_vanilla(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    recipes.push(Recipe {
        items: recipe_item
            .items
            .iter()
            .zip(recipe_item.item_counts.iter())
            .map(|(item, item_count)| Resource {
                resource_type: Direct(Cargo {
                    item_id: *item,
                    point: 0,
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
                    point: 0,
                }),
                num: *result_count as f64,
            })
            .collect(),

        time: recipe_item.time_spend as f64,
    });
}

pub fn flatten_recipes(basic_recipes: &[RecipeItem]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    basic_recipes.iter().for_each(|recipe_item| {
        recipe_vanilla(&mut recipes, recipe_item);
        recipes_productive(&mut recipes, recipe_item);
        recipes_speed_up(&mut recipes, recipe_item);
    });
    recipes
}
