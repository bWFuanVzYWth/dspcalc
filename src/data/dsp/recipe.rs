use crate::data::dsp::item::{Cargo, IndirectResource};

use super::item::{
    Item::*,
    Resource,
    ResourceType::{Direct, Indirect},
};

#[derive(Clone)]
pub struct Recipe {
    pub resources: Vec<Resource>,
    pub products: Vec<Resource>,
}

fn recipes_speed_up(basic_recipe: &BasicRecipe) -> Vec<Recipe> {
    todo!()
}

fn increase_production_scale(point: u64) -> f64 {
    match point {
        1 => 1.125,
        2 => 1.2,
        3 => 1.225,
        4 => 1.25,
        _ => panic!("unsupported point: {}", point),
    }
}

// TODO 把喷涂增产剂视为单独的公式
// TODO 耗电量

fn increase_production(basic_recipe: &BasicRecipe, point: u64) -> Recipe {
    Recipe {
        // 为所有原料喷涂增产剂
        resources: basic_recipe
            .resources
            .iter()
            .map(|resource| match &resource.resource_type {
                Direct(cargo) => Resource {
                    resource_type: Direct(Cargo {
                        item: cargo.item.clone(),
                        point,
                    }),
                    num: resource.num,
                },
                Indirect(_) => resource.clone(),
            })
            .collect(),

        // 增产
        products: basic_recipe
            .products
            .iter()
            .map(|product| match &product.resource_type {
                Direct(cargo) => Resource {
                    resource_type: Direct(Cargo {
                        item: cargo.item.clone(),
                        point,
                    }),
                    num: increase_production_scale(point) * product.num,
                },
                Indirect(indirect) => match indirect {
                    IndirectResource::Power => Resource {
                        resource_type: Indirect(IndirectResource::Power),
                        num: increase_production_scale(point) * product.num,
                    },
                    _ => product.clone(),
                },
            })
            .collect(),
    }
}

fn recipes_increase_production(receipes: &mut Vec<Recipe>, basic_recipe: &BasicRecipe) {
    if basic_recipe.increase_production {
        receipes.push(increase_production(basic_recipe, 1));
        receipes.push(increase_production(basic_recipe, 2));
        receipes.push(increase_production(basic_recipe, 3));
        receipes.push(increase_production(basic_recipe, 4));
    }
}

pub fn receipes(basic_recipes: &[BasicRecipe]) -> Vec<Recipe> {
    let mut receipes = Vec::new();
    basic_recipes.iter().for_each(|basic_recipe| {
        receipes.push(Recipe {
            // TODO 拆分
            resources: basic_recipe.resources.to_vec(),
            products: basic_recipe.products.to_vec(),
        });
        recipes_increase_production(&mut receipes, basic_recipe);
    });
    receipes
}

pub struct BasicRecipe<'a> {
    pub resources: &'a [Resource],
    pub products: &'a [Resource],
    pub speed_up: bool,
    pub increase_production: bool,
}

pub const BASIC_RECIPES: &[BasicRecipe] = &[
    BasicRecipe {
        resources: &[Resource::from_item(煤矿, 2.0), Resource::time(2.0)],
        products: &[Resource::from_item(高能石墨, 1.0)],
        speed_up: true,
        increase_production: true,
    },
    BasicRecipe {
        resources: &[
            Resource::from_item(精炼油, 1.0),
            Resource::from_item(氢, 2.0),
            Resource::time(4.0),
        ],
        products: &[
            Resource::from_item(氢, 3.0),
            Resource::from_item(高能石墨, 1.0),
        ],
        speed_up: true,
        increase_production: false,
    },
    BasicRecipe {
        resources: &[
            Resource::from_item(精炼油, 2.0),
            Resource::from_item(氢, 1.0),
            Resource::from_item(煤矿, 1.0),
            Resource::time(4.0),
        ],
        products: &[Resource::from_item(精炼油, 3.0)],
        speed_up: true,
        increase_production: false,
    },
];
