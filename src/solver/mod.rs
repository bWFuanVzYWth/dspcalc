mod config;
mod constraint;
mod objective;
mod translator;

use std::collections::HashSet;

use config::config_solver;
use constraint::{constraint_needs, constraint_recipes};
use good_lp::{
    clarabel,
    constraint::ConstraintReference,
    solvers::clarabel::{ClarabelProblem, ClarabelSolution},
    variable, variables, Expression, Solution, SolverModel, Variable,
};
use objective::minimize_buildings_count;
use translator::{from_clarabel_solution, CalculatorSolution};

use crate::{
    dsp::{
        item::{Resource, ResourceType},
        recipe::Recipe,
    },
    error::DspCalError::{self, LpSolverError, UnknownLpVarId},
};

// TODO 检查这个hashSet能否去掉
fn find_all_production(recipes: &[Recipe]) -> Vec<ResourceType> {
    let mut items_type = HashSet::new();
    for recipe in recipes.iter() {
        for product in recipe.results.iter() {
            items_type.insert(product.resource_type);
        }
    }
    items_type.into_iter().collect()
}

pub fn solve(
    all_recipes: &[Recipe],
    needs: &[Resource],
) -> Result<Vec<CalculatorSolution>, DspCalError> {
    let all_productions = find_all_production(&all_recipes);

    // 声明变量，每个变量表示某个公式对应的建筑数量
    let mut model = variables!();
    let recipe_variables = all_recipes
        .iter()
        .map(|_| model.add(variable().min(0.0)))
        .collect::<Vec<_>>();

    // TODO 多种待优化目标，如最小化加权原矿，最小化占地
    let objective = minimize_buildings_count(&recipe_variables);

    // 这个方法就叫minimise，不是minimize，奇异搞笑
    let mut problem = model.minimise(objective).using(clarabel);

    // 设置线性规划求解精度
    config_solver(&mut problem);

    // 根据公式生成并设置相应的约束
    constraint_recipes(
        all_recipes,
        &recipe_variables,
        &mut problem,
        &all_productions,
    );

    // 根据需求列表生成并设置相应的约束
    let _constraint_need = constraint_needs(all_recipes, &recipe_variables, &mut problem, needs);

    // 求解
    let clarabel_solution = problem.solve().map_err(LpSolverError)?;

    // 把求解器的内部格式转换成通用的格式
    let solution = from_clarabel_solution(&recipe_variables, all_recipes, &clarabel_solution)?;

    Ok(solution)
}
