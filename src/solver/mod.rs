pub mod proliferator;

use std::collections::HashSet;

use bimap::BiMap;
use good_lp::{
    clarabel, solvers::clarabel::ClarabelProblem, variable, variables, Expression, Solution,
    SolverModel, Variable,
};

use crate::data::dsp::{
    item::{Cargo, Resource, ResourceType},
    recipe::{flatten_recipes, Recipe},
};
use dspdb::{
    item::{items, ItemData},
    recipe::{self},
};
use proliferator::Proliferator;

fn stack(items: &[Resource], resource_type: &ResourceType) -> f64 {
    items
        .iter()
        .filter(|r| r.resource_type == *resource_type)
        .map(|r| r.num)
        .sum()
}

/// 构建需求约束：对于每种净需求，总产出 - 总消耗 ≥ 净需求
fn constraint_need(
    all_recipes: &[Recipe],
    recipes_frequency: &BiMap<usize, Variable>,
    problem: &mut ClarabelProblem,
    need: Resource,
) -> good_lp::constraint::ConstraintReference {
    let items = recipes_frequency
        .iter()
        .map(|(recipes_index, variable)| {
            all_recipes[*recipes_index]
                .items
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / all_recipes[*recipes_index].time) * (*variable))
                .sum::<Expression>()
        })
        .sum::<Expression>();

    let results = recipes_frequency
        .iter()
        .map(|(recipes_index, variable)| {
            all_recipes[*recipes_index]
                .results
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / all_recipes[*recipes_index].time) * (*variable))
                .sum::<Expression>()
        })
        .sum::<Expression>();

    problem.add_constraint((results - items).geq(need.num))
}

fn constraint_needs(
    all_recipes: &[Recipe],
    recipes_frequency: &BiMap<usize, Variable>,
    problem: &mut ClarabelProblem,
    needs: &[Resource],
) {
    for need in needs {
        constraint_need(all_recipes, recipes_frequency, problem, *need);
    }
}

/// 构建生产约束：对于每种资源，总产出 ≥ 总消耗
fn constraint_recipe(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    recipe_vars: &bimap::BiHashMap<usize, Variable>,
    production: &ResourceType,
) {
    let production_expr: Expression = recipes
        .iter()
        .enumerate()
        .map(|(i, recipe)| {
            recipe
                .results
                .iter()
                .filter(|res| res.resource_type == *production)
                .map(|_| {
                    let var = *recipe_vars.get_by_left(&i).unwrap();
                    var * stack(&recipe.results, production) / recipe.time
                })
                .sum::<Expression>()
        })
        .sum();

    let resource_expr: Expression = recipes
        .iter()
        .enumerate()
        .map(|(i, recipe)| {
            recipe
                .items
                .iter()
                .filter(|res| res.resource_type == *production)
                .map(|_| {
                    let var = *recipe_vars.get_by_left(&i).unwrap();
                    var * stack(&recipe.items, production) / recipe.time
                })
                .sum::<Expression>()
        })
        .sum();

    problem.add_constraint(production_expr.geq(resource_expr));
}

fn constraint_recipes(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    recipe_vars: &BiMap<usize, Variable>,
) {
    all_productions.iter().for_each(|production| {
        constraint_recipe(problem, recipes, recipe_vars, production);
    });
}

fn minimize_buildings_count(recipe_vars: &BiMap<usize, Variable>) -> Expression {
    recipe_vars
        .iter()
        .map(|(_, variable)| *variable)
        .sum::<Expression>()
}

fn find_all_production(recipes: &[Recipe]) -> HashSet<ResourceType> {
    let mut items_type = HashSet::new();
    recipes.iter().for_each(|recipe| {
        recipe.results.iter().for_each(|product| {
            items_type.insert(product.resource_type);
        });
    });
    items_type
}

fn proliferator_recipes(items_data: &[ItemData]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    items_data.iter().for_each(|item_data| {
        generate_proliferator_recipe(&mut recipes, item_data, Proliferator::MK3);
        generate_proliferator_recipe(&mut recipes, item_data, Proliferator::MK2);
        generate_proliferator_recipe(&mut recipes, item_data, Proliferator::MK1);
    });
    recipes
}

const STACK: f64 = 4.0;
const PROLIFERATOR_TIME: f64 = 2.0;

fn generate_proliferator_recipe(
    recipes: &mut Vec<Recipe>,
    item_data: &ItemData,
    proliferator: Proliferator,
) {
    const INC_LEVEL_MK3: usize = Proliferator::inc_level(&Proliferator::MK3);
    for cargo_level in 1..=Proliferator::inc_level(&proliferator) {
        for proliferator_level in 0..=INC_LEVEL_MK3 {
            let life = Proliferator::life(&proliferator, proliferator_level) as f64;
            recipes.push(Recipe {
                items: vec![
                    Resource::from_item_level(item_data.id, 0, STACK),
                    Resource::from_item_level(
                        Proliferator::item_id(&proliferator),
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

// 调用方式

// TODO 把求解过程抽象出来，生成约束的过程也是，这样可以更方便的拓展到其它游戏/mod

// TODO 设置生产设备
// TODO 传入需求和约束，返回求解过程和结果
pub fn solve() {
    let raw_recipes = recipe::recipes();
    let raw_items = items();

    // TODO 原矿化、采矿公式

    // 展平所有基础公式
    let flatten_basic_recipes = flatten_recipes(&raw_recipes.data_array);
    // 所有的喷涂公式
    let proliferator_recipes = proliferator_recipes(&raw_items.data_array);

    // 找出所有在公式中出现过的资源
    let all_recipes = [flatten_basic_recipes, proliferator_recipes].concat();
    let all_productions = find_all_production(&all_recipes);

    // 定义变量，每个变量代表一个公式的调用次数
    let mut recipes_frequency = bimap::BiMap::new();
    let mut model = variables!();
    all_recipes.iter().enumerate().for_each(|(i, _)| {
        let frequency = model.add(variable().min(0.0));
        recipes_frequency.insert(i, frequency); // recipes_index -> recipes_frequency
    });

    // TODO 多种待优化目标，如最小化加权原矿，最小化占地
    let objective = minimize_buildings_count(&recipes_frequency);

    // 这个方法就叫minimise，不是minimize，奇异搞笑
    let mut problem = model.minimise(objective).using(clarabel);

    // 设置线性规划求解精度
    problem
        .settings()
        .verbose(true) // 启用详细输出
        .tol_gap_abs(f64::EPSILON)
        .tol_gap_rel(f64::EPSILON)
        .tol_feas(f64::EPSILON)
        .tol_infeas_abs(f64::EPSILON)
        .tol_infeas_rel(f64::EPSILON)
        .static_regularization_constant(f64::EPSILON)
        .dynamic_regularization_eps(f64::EPSILON)
        .max_iter(u32::MAX);

    constraint_recipes(
        &mut problem,
        &all_recipes,
        &all_productions,
        &recipes_frequency,
    );

    let need_type = ResourceType::Direct(Cargo {
        item_id: 6006,
        // item_id: 1143,
        level: 4,
    });

    let need = Resource {
        resource_type: need_type,
        num: 10000.0,
    };

    let needs = vec![need];

    assert!(all_productions.contains(&need_type)); // FIXME 确保待求解的物品存在，但是不要崩溃
    let need_frequency = 10000.0;

    let _constraint_need = constraint_needs(&all_recipes, &recipes_frequency, &mut problem, &needs);

    let solution = problem.solve().unwrap(); // FIXME 异常处理

    all_recipes.iter().enumerate().for_each(|(i, recipe)| {
        let num = solution.value(*recipes_frequency.get_by_left(&i).unwrap()); // FIXME 此处虽然不太可能，还是还是需要提供报错
        if num > need_frequency * f64::from(f32::EPSILON) {
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
        .unwrap()
        .name
        .clone()
}
