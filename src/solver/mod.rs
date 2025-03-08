use std::collections::{HashMap, HashSet};

use crate::data::dsp::{
    item::{Cargo, Resource, ResourceType},
    recipe::{flatten_recipes, Recipe},
};
use dspdb::{
    item::{items, ItemData},
    recipe::{self},
};
use good_lp::{
    clarabel, solvers::clarabel::ClarabelProblem, variable, variables, Expression, Solution,
    SolverModel, Variable,
};

const 增产剂MK1_ID: i16 = 1141;
const 增产剂MK2_ID: i16 = 1142;
const 增产剂MK3_ID: i16 = 1143;

enum 增产剂 {
    MK1 = 1141,
    MK2 = 1142,
    MK3 = 1143,
}

impl 增产剂 {
    pub const fn point(t: &Self) -> u64 {
        match t {
            增产剂::MK1 => 1,
            增产剂::MK2 => 2,
            增产剂::MK3 => 4,
        }
    }

    pub const fn life(t: &Self, point: u64) -> u64 {
        (Self::extra(point)
            * match t {
                增产剂::MK1 => 12.0,
                增产剂::MK2 => 24.0,
                增产剂::MK3 => 60.0,
            }) as u64
    }

    pub const fn extra(point: u64) -> f64 {
        match point {
            1 => 1.125,
            2 => 1.2,
            3 => 1.225,
            4 => 1.25,
            _ => 1.0,
        }
    }

    pub const fn speed_up(point: u64) -> f64 {
        match point {
            1 => 1.25,
            2 => 1.50,
            3 => 1.75,
            4 => 2.00,
            _ => 1.0,
        }
    }

    pub const fn power(point: u64) -> f64 {
        match point {
            1 => 1.3,
            2 => 1.7,
            3 => 2.1,
            4 => 2.5,
            _ => 1.0,
        }
    }
}

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
            items_type.insert(product.resource_type);
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

fn proliferator_recipes(items_data: &[ItemData]) -> Vec<Recipe> {
    let mut recipes = Vec::new();
    items_data.iter().for_each(|item_data| {
        proliferator_mk3(&mut recipes, item_data);
        proliferator_mk2(&mut recipes, item_data);
        proliferator_mk1(&mut recipes, item_data);
    });
    recipes
}

const STACK: f64 = 4.0;

fn proliferator_mk3(recipes: &mut Vec<Recipe>, item_data: &ItemData) {
    (1..=4).for_each(|cargo_point| {
        (0..=4).for_each(|proliferator_point| {
            recipes.push(Recipe {
                items: vec![
                    Resource::from_item_point(item_data.id, 0, STACK),
                    Resource::from_item_point(
                        增产剂::MK3 as i16,
                        proliferator_point,
                        STACK / (增产剂::life(&增产剂::MK3, proliferator_point) as f64),
                    ),
                ],
                results: vec![Resource::from_item_point(item_data.id, cargo_point, STACK)],
                time: 2.0,
            });
        });
    });
}

fn proliferator_mk2(recipes: &mut Vec<Recipe>, item_data: &ItemData) {
    (1..=2).for_each(|cargo_point| {
        (1..=4).for_each(|proliferator_point| {
            recipes.push(Recipe {
                items: vec![
                    Resource::from_item_point(item_data.id, 0, STACK),
                    Resource::from_item_point(
                        增产剂::MK2 as i16,
                        proliferator_point,
                        4.0 / (增产剂::life(&增产剂::MK2, proliferator_point) as f64),
                    ),
                ],
                results: vec![Resource::from_item_point(item_data.id, cargo_point, STACK)],
                time: 2.0,
            });
        });
    });
}

fn proliferator_mk1(recipes: &mut Vec<Recipe>, item_data: &ItemData) {
    const CARGO_POINT: u64 = 1;
    (1..=4).for_each(|proliferator_point| {
        recipes.push(Recipe {
            items: vec![
                Resource::from_item_point(item_data.id, 0, STACK),
                Resource::from_item_point(
                    增产剂::MK1 as i16,
                    proliferator_point,
                    4.0 / (增产剂::life(&增产剂::MK1, proliferator_point) as f64),
                ),
            ],
            results: vec![Resource::from_item_point(item_data.id, CARGO_POINT, STACK)],
            time: 2.0,
        });
    });
}

// TODO 设置生产设备
// TODO 传入需求和约束，返回求解过程和结果
pub fn solve() {
    let raw_recipes = recipe::recipes();
    let raw_items = items();

    // TODO 简化这里的逻辑，现在无法计算增产剂

    // 展平所有基础公式
    let flatten_basic_recipes = flatten_recipes(&raw_recipes.data_array);
    let proliferator_recipes = proliferator_recipes(&raw_items.data_array);

    // 找出所有在公式中出现过的资源
    // let flatten_basic_items = find_all_items(&flatten_basic_recipes);
    // 生成喷涂公式
    // let proliferator_recipes = proliferator_recipes(&flatten_basic_items);
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
    // 设置求解精度
    problem
        .settings()
        .verbose(true) // 启用详细输出
        .tol_gap_abs(f64::EPSILON) // 绝对对偶间隙容差
        .tol_gap_rel(f64::EPSILON) // 相对对偶间隙容差
        .tol_feas(f64::EPSILON) // 可行性容差
        .max_iter(u32::MAX); // 最大迭代次数

    constraint_recipe(
        &mut problem,
        &all_recipes,
        &all_productions,
        &recipes_frequency,
    );

    let need_type = ResourceType::Direct(Cargo {
        item_id: 6006,
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

    all_recipes.iter().enumerate().for_each(|(i, recipe)| {
        let num = solution.value(*recipes_frequency.get(&i).unwrap()); // TODO 此处虽然不太可能，还是还是需要提供报错
        if num > need_frequency * f64::from(f32::EPSILON) {
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
                "{:.6} * {}_{}, ",
                num * resource.num / recipe.time,
                item_name(cargo.item_id, items),
                cargo.point
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
                cargo.point
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
