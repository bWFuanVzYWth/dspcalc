pub mod proliferator;

use std::collections::HashSet;

use bimap::{BiHashMap, BiMap};
use good_lp::{
    clarabel,
    constraint::ConstraintReference,
    solvers::clarabel::{ClarabelProblem, ClarabelSolution},
    variable, variables, Expression, Solution, SolverModel, Variable,
};

use crate::{
    data::dsp::{
        item::{Resource, ResourceType},
        recipe::{flatten_recipes, Recipe},
    },
    error::DspCalError::{self, LpSolverError},
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
) -> Vec<ConstraintReference> {
    let mut constraints = Vec::new();
    for need in needs {
        constraints.push(constraint_need(
            all_recipes,
            recipes_frequency,
            problem,
            *need,
        ));
    }
    constraints
}

/// 构建生产约束：对于每种资源，总产出 ≥ 总消耗
fn constraint_recipe(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    recipe_vars: &bimap::BiHashMap<usize, Variable>,
    production: &ResourceType,
) -> Result<(), DspCalError> {
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

    Ok(())
}

fn constraint_recipes(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    recipe_vars: &BiMap<usize, Variable>,
) -> Result<(), DspCalError> {
    for production in all_productions.iter() {
        constraint_recipe(problem, recipes, recipe_vars, production)?;
    }
    Ok(())
}

fn minimize_buildings_count(recipe_vars: &BiMap<usize, Variable>) -> Expression {
    recipe_vars
        .iter()
        .map(|(_, variable)| *variable)
        .sum::<Expression>()
}



// TODO 把求解过程抽象出来，生成约束的过程也是，这样可以更方便的拓展到其它游戏/mod

// TODO 设置生产设备
// TODO 传入约束，返回求解过程和结果
// FIXME 库无关的返回类型
pub fn solve(
    all_recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    needs: &[Resource],
    mines: &[ResourceType],
) -> Result<Vec<CalculatorSolution>, DspCalError> {
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
    config_solver(&mut problem);

    constraint_recipes(
        &mut problem,
        &all_recipes,
        &all_productions,
        &recipes_frequency,
    )?;

    let _constraint_need = constraint_needs(&all_recipes, &recipes_frequency, &mut problem, needs);

    let clarabel_solution = problem.solve().map_err(LpSolverError)?;
    let solution = from_clarabel_solution(&recipes_frequency, &all_recipes, &clarabel_solution);
    Ok(solution)
    // let solution = solve.unwrap(); // FIXME 异常处理

    // all_recipes.iter().enumerate().for_each(|(i, recipe)| {
    //     let num = solution.value(*recipes_frequency.get_by_left(&i).unwrap()); // FIXME 此处虽然不太可能，还是还是需要提供报错
    //     if num > f64::from(f32::EPSILON) {
    //         print_recipe(num, recipe, &raw_items.data_array);
    //     }
    // });
}

pub struct CalculatorSolution {
    pub recipe: Recipe,
    pub num: f64,
}

pub fn from_clarabel_solution(
    recipes_frequency: &BiHashMap<usize, Variable>,
    all_recipes: &[Recipe],
    clarabel_solution: &ClarabelSolution,
) -> Vec<CalculatorSolution> {
    let mut solutions = Vec::new();
    for (i, recipe) in all_recipes.iter().enumerate() {
        let num = clarabel_solution.value(*recipes_frequency.get_by_left(&i).unwrap());
        if num > f64::from(f32::EPSILON) {
            let solution = CalculatorSolution {
                recipe: recipe.clone(),
                num,
            };
            solutions.push(solution);
        }
    }
    solutions
}

fn config_solver(problem: &mut ClarabelProblem) {
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
}
