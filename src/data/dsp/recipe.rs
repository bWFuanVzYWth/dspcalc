use crate::{data::dsp::item::Cargo, solver::proliferator::Proliferator};
use dspdb::recipe::RecipeItem;

use super::item::{Resource, ResourceType::Direct};

#[derive(Clone, Debug)]
pub struct RecipeFmtInfo {
    pub name: String, // 公式的名字
    pub level: usize, // 使用的增产剂
    pub speed_up: bool,
    pub building_type: BuildingType, // 生产于什么建筑
}

impl Default for RecipeFmtInfo {
    fn default() -> Self {
        Self {
            name: "Unknown Building".to_string(),
            level: 0,
            speed_up: true,
            building_type: BuildingType::Unknown,
        }
    }
}

#[derive(Clone, Debug)]
pub enum BuildingType {
    熔炉 = 1,
    化工 = 2,
    精炼厂 = 3,
    制造台 = 4,
    对撞机 = 5,
    科研站 = 15,
    矿机,
    喷涂机,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct Recipe {
    pub items: Vec<Resource>,   // 原料
    pub results: Vec<Resource>, // 产物
    pub time: f64,              // 公式耗时，单位帧
    pub info: RecipeFmtInfo,    // 不参与计算的信息
}

const fn get_recipe_building(recipe_item: &RecipeItem) -> BuildingType {
    match recipe_item.type_ {
        1 => BuildingType::熔炉,
        2 => BuildingType::化工,
        3 => BuildingType::精炼厂,
        4 => BuildingType::制造台,
        5 => BuildingType::对撞机,
        15 => BuildingType::科研站,
        _ => BuildingType::Unknown,
    }
}

fn time_scale(building_type: &BuildingType) -> f64 {
    1.0 / match building_type {
        BuildingType::熔炉 => 3.0,
        BuildingType::化工 => 2.0,
        BuildingType::精炼厂 => 1.0,
        BuildingType::制造台 => 3.0,
        BuildingType::对撞机 => 1.0,
        BuildingType::科研站 => 3.0,
        BuildingType::矿机 => 1.0,
        BuildingType::喷涂机 => 1.0,
        BuildingType::Unknown => 1.0,
    }
}

fn create_recipe(
    recipe_item: &RecipeItem,
    level: usize,
    modify_result_num: impl Fn(f64) -> f64,
    modify_time: impl Fn(f64) -> f64,
    info: RecipeFmtInfo,
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
        time: modify_time(recipe_item.time_spend as f64)
            * time_scale(&get_recipe_building(recipe_item)),
        info,
    }
}

fn accelerate(recipe_item: &RecipeItem, level: usize) -> Recipe {
    let info = RecipeFmtInfo {
        name: recipe_item.name.clone(),
        level,
        speed_up: true,
        building_type: get_recipe_building(recipe_item),
    };
    create_recipe(
        recipe_item,
        level,
        |num| num,
        |time| time / Proliferator::accelerate(level),
        info,
    )
}

fn productive(recipe_item: &RecipeItem, level: usize) -> Recipe {
    let info = RecipeFmtInfo {
        name: recipe_item.name.clone(),
        level,
        speed_up: false,
        building_type: get_recipe_building(recipe_item),
    };
    create_recipe(
        recipe_item,
        level,
        |num| num * Proliferator::increase(level),
        |time| time,
        info,
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

fn recipe_vanilla(recipes: &mut Vec<Recipe>, recipe_item: &RecipeItem) {
    let info = RecipeFmtInfo {
        name: recipe_item.name.clone(),
        level: 0,
        speed_up: false,
        building_type: get_recipe_building(recipe_item),
    };
    recipes.push(create_recipe(recipe_item, 0, |num| num, |time| time, info));
}
// TODO 耗电量

#[must_use]
pub fn flatten_recipes(basic_recipes: &[RecipeItem]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    for recipe_item in basic_recipes {
        recipe_vanilla(&mut recipes, recipe_item);
        recipes_productive(&mut recipes, recipe_item);
        recipes_accelerate(&mut recipes, recipe_item);
    }
    recipes
}
