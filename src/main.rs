use std::collections::HashSet;

use dspdb::{
    item::{items, ItemData},
    recipe,
};
use dspcalc::{
    data::dsp::{
        item::{
            Cargo, Resource,
            ResourceType::{self, Direct},
        },
        recipe::{flatten_recipes, Recipe},
    },
    solver::proliferator::Proliferator,
};

fn find_all_production(recipes: &[Recipe]) -> Vec<ResourceType> {
    let mut items_type = HashSet::new();
    for recipe in recipes.iter() {
        for product in recipe.results.iter() {
            items_type.insert(product.resource_type);
        }
    }
    items_type.into_iter().collect()
}

fn proliferator_recipes(items_data: &[ItemData]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    for item_data in items_data.iter() {
        generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK3);
        generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK2);
        generate_proliferator_recipe(&mut recipes, item_data, &Proliferator::MK1);
    }
    recipes
}

const STACK: f64 = 4.0;
const PROLIFERATOR_TIME: f64 = 2.0;

fn generate_proliferator_recipe(
    recipes: &mut Vec<Recipe>,
    item_data: &ItemData,
    proliferator: &Proliferator,
) {
    const INC_LEVEL_MK3: usize = Proliferator::inc_level(&Proliferator::MK3);
    for cargo_level in 1..=Proliferator::inc_level(proliferator) {
        for proliferator_level in 0..=INC_LEVEL_MK3 {
            let life = Proliferator::life(proliferator, proliferator_level) as f64;
            recipes.push(Recipe {
                items: vec![
                    Resource::from_item_level(item_data.id, 0, STACK),
                    Resource::from_item_level(
                        Proliferator::item_id(proliferator),
                        proliferator_level,
                        STACK / life,
                    ),
                ],
                results: vec![Resource::from_item_level(item_data.id, cargo_level, STACK)],
                time: PROLIFERATOR_TIME,
            });
        }
    }
}

fn is_mine(item: &ItemData) -> bool {
    !item.mining_from.is_empty()
}

fn main() {
    let need_white_cube = Resource {
        resource_type: ResourceType::Direct(Cargo {
            item_id: 6006,
            level: 4,
        }),
        num: 10000.0,
    };

    let need_proliferator_mk3 = Resource {
        resource_type: ResourceType::Direct(Cargo {
            item_id: 1143,
            level: 4,
        }),
        num: 10000.0,
    };

    let raw_recipes = recipe::recipes();
    let raw_items = items();

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
                    num: 200.0, // TODO 根据采矿等级设置成本，或者增加原矿化标记字段，不计成本
                }],
                time: 1.0,
            };
            mines.push(tmp);
        }
    }
    // dbg!(mines);

    // 展平所有基础公式
    let flatten_basic_recipes = flatten_recipes(&raw_recipes.data_array);
    // 所有的喷涂公式
    let proliferator_recipes = proliferator_recipes(&raw_items.data_array);

    // 找出所有在公式中出现过的资源
    let all_recipes = [flatten_basic_recipes, proliferator_recipes, mines].concat();
    let all_productions = find_all_production(&all_recipes);

    let needs = vec![need_white_cube, need_proliferator_mk3];

    // FIXME 消除这个unwarp
    let solutions =
        dspcalc::solver::solve(&all_recipes, &all_productions, &needs).unwrap();
    for solution in solutions {
        print_recipe(solution.num, &solution.recipe, &raw_items.data_array);
    }
}

fn print_recipe(num: f64, recipe: &Recipe, items: &[ItemData]) {
    recipe
        .items
        .iter()
        .for_each(|resource| match resource.resource_type {
            ResourceType::Direct(cargo) => print!(
                "{:.6} * {}_{}, ",
                num * resource.num / recipe.time,
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
                num * resource.num / recipe.time,
                item_name(cargo.item_id, items),
                cargo.level
            ),
            ResourceType::Indirect(_indirect_resource) => todo!(),
        });

    println!();
}

fn item_name(item_id: i16, items: &[ItemData]) -> String {
    items
        .iter()
        .find(|item| item.id == item_id)
        .map(|item| item.name.clone())
        .unwrap_or_else(|| format!("Unknown Item {}", item_id))
}
