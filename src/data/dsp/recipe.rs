use crate::data::dsp::item::Cargo;
use dspdb::recipe::RecipeItem;

use super::item::{Resource, ResourceType::Direct};

#[derive(Clone, Debug)]
pub struct Recipe {
    pub items: Vec<Resource>,
    pub results: Vec<Resource>,
    pub time: f64,
}

const fn speed_up_scale(point: u64) -> f64 {
    match point {
        0 => 1.0,
        1 => 1.0 / 1.25,
        2 => 1.0 / 1.5,
        3 => 1.0 / 1.75,
        4 => 1.0 / 2.0,
        _ => panic!("fatal error: un support point!"),
    }
}

const fn productive_scale(point: u64) -> f64 {
    match point {
        0 => 1.0,
        1 => 1.125,
        2 => 1.2,
        3 => 1.225,
        4 => 1.25,
        _ => panic!("fatal error: un support point!"),
    }
}

fn create_recipe(
    recipe_item: &RecipeItem,
    point: u64,
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
                    point,
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
                    point: 0,
                }),
                num: modify_result_num(*count as f64),
            })
            .collect(),
        time: modify_time(recipe_item.time_spend as f64),
    }
}

fn speed_up(recipe_item: &RecipeItem, point: u64) -> Recipe {
    create_recipe(
        recipe_item,
        point,
        |num| num,
        |time| time * speed_up_scale(point),
    )
}

fn productive(recipe_item: &RecipeItem, point: u64) -> Recipe {
    create_recipe(
        recipe_item,
        point,
        |num| num * productive_scale(point),
        |time| time,
    )
}

fn recipes_speed_up(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    for point in 1..=4 {
        recipes.push(speed_up(recipe_item, point));
    }
}

fn recipes_productive(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    if !recipe_item.non_productive {
        for point in 1..=4 {
            recipes.push(productive(recipe_item, point));
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
