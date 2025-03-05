use std::collections::{HashMap, HashSet};

use crate::data::dsp::{
    item::{Cargo, IndirectResource, Resource, ResourceType},
    recipe::{flatten_recipes, Recipe},
};
use dspdb::{
    item::{items, ItemData, ItemProtoSet},
    recipe::{self, recipes},
};
use good_lp::{
    clarabel, solvers::clarabel::ClarabelProblem, variable, variables, Expression, Solution,
    SolverModel, Variable,
};

const 增产剂MK3_ID: i16 = 1143;

// 辅助函数：生成单个配方的表达式项
fn recipe_term(
    (i, recipe): (usize, &Recipe),
    production: &ResourceType,
    recipe_vars: &HashMap<usize, Variable>,
    resource_accessor: fn(&Recipe) -> &[Resource],
) -> Option<Expression> {
    let items = resource_accessor(recipe);
    items
        .iter()
        .find(|res| res.resource_type == *production)
        .map(|_| recipe_vars[&i] * stack(items, production) / recipe.time)
}

fn constraint_recipe(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    recipe_vars: &HashMap<usize, Variable>,
) {
    all_productions.iter().for_each(|production| {
        let production_expr: Expression = recipes
            .iter()
            .enumerate()
            .filter_map(|item| recipe_term(item, production, recipe_vars, |r| &r.results))
            .sum();

        let resource_expr: Expression = recipes
            .iter()
            .enumerate()
            .filter_map(|item| recipe_term(item, production, recipe_vars, |r| &r.items))
            .sum();

        problem.add_constraint(production_expr.geq(resource_expr));
    });
}

// 最小化建筑总数量
fn objective(recipe_vars: &HashMap<usize, Variable>) -> Expression {
    recipe_vars
        .iter()
        .map(|(_, variable)| variable)
        .sum::<Expression>()
}

fn find_all_production(recipes: &[Recipe]) -> HashSet<ResourceType> {
    let mut items_type = HashSet::new();
    recipes.iter().for_each(|recipe| {
        recipe.results.iter().for_each(|product| {
            items_type.insert(product.resource_type.clone());
        });
    });
    items_type
}

fn find_all_items(recipes: &[Recipe]) -> HashSet<ResourceType> {
    let mut items_type = HashSet::new();
    recipes.iter().for_each(|recipe| {
        recipe.items.iter().for_each(|resource| {
            items_type.insert(resource.resource_type.clone());
        });
    });
    items_type
}

fn stack(items: &[Resource], resource_type: &ResourceType) -> f64 {
    match items.iter().find(|r| r.resource_type == *resource_type) {
        Some(resource) => resource.num,
        None => 0.0,
    }
}

// TODO 低级喷涂
fn proliferator_recipes(all_items: &HashSet<ResourceType>) -> Vec<Recipe> {
    all_items
        .iter()
        .filter(|resource| match resource {
            ResourceType::Direct(cargo) => cargo.point > 0,
            _ => false,
        })
        .map(|resource| {
            let cargo = match resource {
                ResourceType::Direct(cargo) => cargo,
                _ => panic!("error"),
            };

            let stack = 4.0 * 4.0 / cargo.point as f64;

            Recipe {
                items: vec![
                    Resource::from_item_point(cargo.item_id.clone(), 0, stack),
                    Resource::from_item_point(增产剂MK3_ID, 4, 4.0 / 75.0),
                ],
                results: vec![Resource::from_item_point(
                    cargo.item_id.clone(),
                    cargo.point,
                    stack,
                )],
                time: 2.0,
            }
        })
        .collect()
}

// TODO 设置生产设备
// TODO 传入需求和约束，返回求解过程和结果
pub fn solve() {
    let raw_recipes = recipe::recipes();

    // 展平所有基础公式
    let flatten_basic_recipes = flatten_recipes(&raw_recipes.data_array);
    // 找出所有在公式中出现过的资源
    let flatten_basic_items = find_all_items(&flatten_basic_recipes);
    // 生成喷涂公式
    let proliferator_recipes = proliferator_recipes(&flatten_basic_items);
    let all_recipes = [flatten_basic_recipes, proliferator_recipes].concat();
    let all_productions = find_all_production(&all_recipes);

    // 定义变量，每个变量代表一个公式的调用次数
    let mut recipes_frequency = HashMap::new();
    let mut model = variables!();
    all_recipes.iter().enumerate().for_each(|(i, _)| {
        let frequency = model.add(variable().min(0.0));
        recipes_frequency.insert(i, frequency); // recipes_index -> recipes_frequency
    });

    // TODO 多种待优化目标，如最小化加权原矿，最小化占地
    let objective = objective(&recipes_frequency);

    // TODO 设置求解精度
    // 就叫minimise，不是minimize，奇异搞笑
    let mut problem = model.minimise(objective).using(clarabel);

    constraint_recipe(
        &mut problem,
        &all_recipes,
        &all_productions,
        &recipes_frequency,
    );

    let need_type = ResourceType::Direct(Cargo {
        item_id: 6004,
        point: 0,
    });
    assert!(all_productions.contains(&need_type)); // FIXME 确保待求解的物品存在，但是不要崩溃
    let need_frequency = 10000.0;

    let _constraint_need = constraint_need(
        &all_recipes,
        &recipes_frequency,
        &mut problem,
        need_type,
        need_frequency,
    );

    let solution = problem.solve().unwrap(); // FIXME 异常处理

    let raw_items = items();
    all_recipes.iter().enumerate().for_each(|(i, recipe)| {
        let num = solution.value(*recipes_frequency.get(&i).unwrap()); // TODO 此处虽然不太可能，还是还是需要提供报错
        if num > need_frequency * 0.001 {
            // println!("数量: {}, 公式: {:?}", num, recipe);
            print_recipe(num, recipe, &raw_items.data_array);
        }
    });
}

fn print_recipe(num: f64, recipe: &Recipe, items: &[ItemData]) {
    recipe
        .items
        .iter()
        .for_each(|resource| match resource.resource_type {
            ResourceType::Direct(cargo) => print!(
                "{} * {}.{}, ",
                num * resource.num / recipe.time,
                item_name(cargo.item_id, items),
                cargo.point
            ),
            ResourceType::Indirect(indirect_resource) => todo!(),
        });

    print!("-> ");

    recipe
        .results
        .iter()
        .for_each(|resource| match resource.resource_type {
            ResourceType::Direct(cargo) => print!(
                "{} * {}.{}, ",
                num * resource.num / recipe.time,
                item_name(cargo.item_id, items),
                cargo.point
            ),
            ResourceType::Indirect(indirect_resource) => todo!(),
        });

    print!("\n");
}

fn item_name(item_id: i16, items: &[ItemData]) -> String {
    items
        .iter()
        .find(|item| item.id == item_id)
        .unwrap()
        .name
        .clone()
}

fn constraint_need(
    all_recipes: &[Recipe],
    recipes_frequency: &HashMap<usize, Variable>,
    problem: &mut ClarabelProblem,
    need_type: ResourceType,
    need_num: f64,
) -> good_lp::constraint::ConstraintReference {
    problem.add_constraint(
        recipes_frequency
            .iter()
            .map(|(recipes_index, variable)| {
                let need_resource = all_recipes[*recipes_index]
                    .results
                    .iter()
                    .find(|product| product.resource_type == need_type);

                let time = all_recipes[*recipes_index].time;

                match need_resource {
                    Some(product) => {
                        // 计算单位时间产能
                        (product.num / time) * (*variable)
                    }
                    None => (0.0).into(),
                }
            })
            .sum::<Expression>()
            .geq(need_num),
    )
}
