use crate::data::dsp::item::{Cargo, IndirectResource};

use super::item::{
    Item::{self, *},
    Resource,
    ResourceType::{Direct, Indirect},
};

#[derive(Clone, Debug)]
pub struct Recipe {
    pub resources: Vec<Resource>,
    pub products: Vec<Resource>,
}

// TODO 看看游戏源代码，检查是否有更优雅的写法
const fn speed_up_scale(point: u64) -> f64 {
    match point {
        1 => 1.0 / 1.25,
        2 => 1.0 / 1.5,
        3 => 1.0 / 1.75,
        4 => 1.0 / 2.0,
        _ => 1.0 / 1.0,
    }
}

fn speed_up(basic_recipe: &BasicRecipe, point: u64) -> Recipe {
    Recipe {
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
                Indirect(indirect_resource) => match indirect_resource {
                    IndirectResource::Time => Resource::time(resource.num * speed_up_scale(point)),
                    _ => resource.clone(),
                },
            })
            .collect(),

        products: basic_recipe.products.to_vec(),
    }
}

fn recipes_speed_up(recipes: &mut Vec<Recipe>, basic_recipe: &BasicRecipe) {
    if basic_recipe.speed_up {
        recipes.push(speed_up(basic_recipe, 1));
        recipes.push(speed_up(basic_recipe, 2));
        recipes.push(speed_up(basic_recipe, 3));
        recipes.push(speed_up(basic_recipe, 4));
    }
}

const fn increase_production_scale(point: u64) -> f64 {
    match point {
        1 => 1.125,
        2 => 1.2,
        3 => 1.225,
        4 => 1.25,
        _ => 1.0,
    }
}

// TODO 把喷涂增产剂视为单独的公式
// TODO 耗电量
// TODO 拆分过大的函数

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

fn recipes_increase_production(recipes: &mut Vec<Recipe>, basic_recipe: &BasicRecipe) {
    if basic_recipe.increase_production {
        recipes.push(increase_production(basic_recipe, 1));
        recipes.push(increase_production(basic_recipe, 2));
        recipes.push(increase_production(basic_recipe, 3));
        recipes.push(increase_production(basic_recipe, 4));
    }
}

fn recipe_vanilla(recipes: &mut Vec<Recipe>, basic_recipe: &BasicRecipe) {
    recipes.push(Recipe {
        resources: basic_recipe.resources.to_vec(),
        products: basic_recipe.products.to_vec(),
    });
}

// TODO 增产喷涂公式
pub fn recipes(basic_recipes: &[BasicRecipe]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    basic_recipes.iter().for_each(|basic_recipe| {
        recipe_vanilla(&mut recipes, basic_recipe);
        recipes_increase_production(&mut recipes, basic_recipe);
        recipes_speed_up(&mut recipes, basic_recipe);
    });
    recipes
}

pub struct BasicRecipe<'a> {
    pub resources: &'a [Resource],
    pub products: &'a [Resource],
    pub speed_up: bool,
    pub increase_production: bool,
}

pub const BASIC_RECIPES: &[BasicRecipe] = &[
    // 采矿
    BasicRecipe {
        resources: &[Resource::time(1.0)],
        products: &[Resource::from_item(煤矿, 1.0)],
        speed_up: false,
        increase_production: false,
    },
    // 喷涂
    BasicRecipe {
        resources: &[
            Resource::from_item(煤矿, 4.0),
            Resource::from_item_point(增产剂mk3, 4, 4.0 / 75.0),
            Resource::time(1.0 / 30.0),
        ],
        products: &[Resource::from_item_point(煤矿, 4, 4.0)],
        speed_up: false,
        increase_production: false,
    },
    BasicRecipe {
        resources: &[
            Resource::from_item(煤矿, 4.0),
            Resource::from_item_point(增产剂mk3, 4, 3.0 / 75.0),
            Resource::time(1.0 / 30.0),
        ],
        products: &[Resource::from_item_point(煤矿, 3, 4.0)],
        speed_up: false,
        increase_production: false,
    },
    BasicRecipe {
        resources: &[
            Resource::from_item(煤矿, 4.0),
            Resource::from_item_point(增产剂mk3, 4, 2.0 / 75.0),
            Resource::time(1.0 / 30.0),
        ],
        products: &[Resource::from_item_point(煤矿, 2, 4.0)],
        speed_up: false,
        increase_production: false,
    },
    BasicRecipe {
        resources: &[
            Resource::from_item(煤矿, 1.0),
            Resource::from_item_point(增产剂mk3, 4, 1.0 / 75.0),
            Resource::time(1.0 / 30.0),
        ],
        products: &[Resource::from_item_point(煤矿, 1, 4.0)],
        speed_up: false,
        increase_production: false,
    },
    // 生产
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
