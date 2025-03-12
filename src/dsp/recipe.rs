use crate::dsp::building::get_recipe_building;
use crate::dsp::item::{item_name, Cargo, ResourceType};
use dspdb::{item::ItemData, recipe::RecipeItem};

use super::{
    building::{time_scale, BuildingType},
    item::{Resource, ResourceType::Direct},
    proliferator::Proliferator,
};

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
pub struct Recipe {
    pub items: Vec<Resource>,   // 原料
    pub results: Vec<Resource>, // 产物
    pub time: f64,              // 公式耗时，单位帧
    pub info: RecipeFmtInfo,    // 不参与计算的信息
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

pub fn print_recipe(num_scale: f64, recipe: &Recipe, items: &[ItemData]) {
    if recipe.info.level >= 1 {
        print!(
            "({}_{})\t",
            if recipe.info.speed_up {
                "加速"
            } else {
                "增产"
            },
            recipe.info.level
        );
    } else {
        print!("(不增产)\t");
    }

    // FIXME magic number
    print!("{:.3?}\t", num_scale / 3600.0);
    print!("{:.3?}s\t", recipe.time / 60.0);

    recipe
        .items
        .iter()
        .for_each(|resource| match resource.resource_type {
            ResourceType::Direct(cargo) => print!(
                "{:.6} * {}_{}, ",
                num_scale * resource.num / recipe.time,
                item_name(cargo.item_id, items),
                cargo.level
            ),
            ResourceType::Indirect(_indirect_resource) => todo!(),
        });

    print!("-> ");

    recipe
        .results
        .iter()
        .for_each(|resource| match resource.resource_type {
            ResourceType::Direct(cargo) => print!(
                "{:.6} * {}_{}, ",
                num_scale * resource.num / recipe.time,
                item_name(cargo.item_id, items),
                cargo.level
            ),
            ResourceType::Indirect(_indirect_resource) => todo!(),
        });

    println!();
}

pub fn proliferator_recipes(items_data: &[ItemData]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    for item_data in items_data.iter() {
        generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK3);
        generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK2);
        generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK1);
    }
    recipes
}

fn generate_proliferator_recipe(
    recipes: &mut Vec<Recipe>,
    item_data: &ItemData,
    proliferator: &Proliferator,
) {
    const STACK: f64 = 4.0;
    const PROLIFERATOR_TIME: f64 = 2.0;
    const INC_LEVEL_MK3: usize = Proliferator::inc_level(&Proliferator::MK3);
    for cargo_level in 1..=Proliferator::inc_level(proliferator) {
        for proliferator_level in 0..=INC_LEVEL_MK3 {
            recipes.push(Recipe {
                items: vec![
                    Resource::from_item_level(item_data.id, 0, STACK),
                    Resource::from_item_level(
                        Proliferator::item_id(proliferator),
                        proliferator_level,
                        ((Proliferator::inc_level(proliferator) as f64) / (cargo_level as f64))
                            * STACK
                            / (Proliferator::life(proliferator, proliferator_level) as f64),
                    ),
                ],
                results: vec![Resource::from_item_level(item_data.id, cargo_level, STACK)],
                time: PROLIFERATOR_TIME,
                info: RecipeFmtInfo {
                    name: "喷涂".to_string(),
                    building_type: BuildingType::喷涂机,
                    ..RecipeFmtInfo::default()
                },
            });
        }
    }
}
