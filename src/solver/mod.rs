pub mod proliferator;

use std::collections::HashSet;

use good_lp::{
    clarabel,
    constraint::ConstraintReference,
    solvers::clarabel::{ClarabelProblem, ClarabelSolution},
    variable, variables, Expression, Solution, SolverModel, Variable,
};

use crate::{
    data::dsp::{
        item::{Resource, ResourceType},
        recipe::Recipe,
    },
    error::DspCalError::{self, *},
};

fn get_stack(items: &[Resource], resource_type: &ResourceType) -> f64 {
    items
        .iter()
        .filter(|r| r.resource_type == *resource_type)
        .map(|r| r.num)
        .sum()
}

/// 构建需求约束：对于每种净需求，总产出 - 总消耗 ≥ 净需求
fn constraint_need(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    need: Resource,
) -> good_lp::constraint::ConstraintReference {
    let items_expr = recipe_variables
        .iter()
        .enumerate()
        .map(|(recipes_index, variable)| {
            all_recipes[recipes_index]
                .items
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / all_recipes[recipes_index].time) * (*variable))
                .sum::<Expression>()
        })
        .sum::<Expression>();

    let results_expr = recipe_variables
        .iter()
        .enumerate()
        .map(|(recipes_index, variable)| {
            all_recipes[recipes_index]
                .results
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / all_recipes[recipes_index].time) * (*variable))
                .sum::<Expression>()
        })
        .sum::<Expression>();

    problem.add_constraint((results_expr - items_expr).geq(need.num))
}

fn constraint_needs(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    needs: &[Resource],
) -> Vec<ConstraintReference> {
    let mut constraints = Vec::new();
    for need in needs {
        constraints.push(constraint_need(
            all_recipes,
            recipe_variables,
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
    recipe_variables: &[Variable],
    production: &ResourceType,
) -> Result<(), DspCalError> {
    let items_expr: Expression = recipes
        .iter()
        .enumerate()
        .map(|(i, recipe)| {
            let var = *recipe_variables.get(i).ok_or(UnknownLpVarId(i))?;
            let expr = recipe
                .items
                .iter()
                .filter(|res| res.resource_type == *production)
                .map(|_| var * get_stack(&recipe.items, production) / recipe.time)
                .sum::<Expression>();
            Ok(expr)
        })
        .collect::<Result<Vec<Expression>, _>>()?
        .into_iter()
        .sum();

    let results_expr: Expression = recipes
        .iter()
        .enumerate()
        .map(|(i, recipe)| {
            // 获取变量，若失败则返回错误
            let variable = *recipe_variables.get(i).ok_or(UnknownLpVarId(i))?;
            // 计算当前recipe的贡献
            let expr = recipe
                .results
                .iter()
                .filter(|res| res.resource_type == *production)
                .map(|_| variable * get_stack(&recipe.results, production) / recipe.time)
                .sum::<Expression>();
            // TODO 这个闭包里的异常处理感觉逻辑上有点绕，可以考虑优化一下
            // TODO 这里两个表达式的构建都有点重复，检查是否有必要抽象出共同的结构
            Ok(expr)
        })
        .collect::<Result<Vec<Expression>, _>>()?
        .into_iter()
        .sum();

    problem.add_constraint(results_expr.geq(items_expr));

    Ok(())
}

fn constraint_recipes(
    problem: &mut ClarabelProblem,
    recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    recipe_variables: &[Variable],
) -> Result<(), DspCalError> {
    for production in all_productions.iter() {
        constraint_recipe(problem, recipes, recipe_variables, production)?;
    }
    Ok(())
}

fn minimize_buildings_count(recipe_variables: &[Variable]) -> Expression {
    // TODO 读取生产设备，计算速度倍率，现在这个只能计算1x倍率的最小化建筑
    recipe_variables
        .iter()
        .map(|variable| *variable)
        .sum::<Expression>()
}

pub fn solve(
    all_recipes: &[Recipe],
    all_productions: &HashSet<ResourceType>,
    needs: &[Resource],
) -> Result<Vec<CalculatorSolution>, DspCalError> {
    // 定义变量，每个变量代表一个公式的调用次数
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
        &mut problem,
        all_recipes,
        all_productions,
        &recipe_variables,
    )?;

    // 根据需求列表生成并设置相应的约束
    let _constraint_need = constraint_needs(all_recipes, &recipe_variables, &mut problem, needs);

    // 求解
    let clarabel_solution = problem.solve().map_err(LpSolverError)?;

    // 把求解器的内部格式转换成通用的格式
    let solution = from_clarabel_solution(&recipe_variables, all_recipes, &clarabel_solution)?;

    Ok(solution)
}

pub struct CalculatorSolution {
    pub recipe: Recipe,
    pub num: f64,
}

pub fn from_clarabel_solution(
    recipe_variables: &[Variable],
    all_recipes: &[Recipe],
    clarabel_solution: &ClarabelSolution,
) -> Result<Vec<CalculatorSolution>, DspCalError> {
    let mut solutions = Vec::new();
    for (i, recipe) in all_recipes.iter().enumerate() {
        let num = clarabel_solution.value(*recipe_variables.get(i).ok_or(UnknownLpVarId(i))?);
        if num > f64::from(f32::EPSILON) {
            let solution = CalculatorSolution {
                recipe: recipe.clone(),
                num,
            };
            solutions.push(solution);
        }
    }
    Ok(solutions)
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
