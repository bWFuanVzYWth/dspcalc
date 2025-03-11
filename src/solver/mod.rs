pub mod proliferator;

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
    error::DspCalError::{self, LpSolverError, UnknownLpVarId},
};

// FIXME 生成这个约束花了大概40%的时间，可以利用配方矩阵的稀疏性大幅度优化
/// 构建约束：对于每种产物，总产出速率 - 总消耗速率 ≥ 额外净需求速率
fn create_constraint(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    need: Resource,
) -> good_lp::constraint::ConstraintReference {
    let items_expr = recipe_variables
        .iter() // 这里，大量的迭代都是完全无关的。
        .enumerate()
        .map(|(recipes_index, variable)| {
            all_recipes[recipes_index]
                .items
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / all_recipes[recipes_index].time) * (*variable))
                // .map(|product| product.num * (*variable))
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
                // .map(|product| product.num * (*variable))
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
        constraints.push(create_constraint(
            all_recipes,
            recipe_variables,
            problem,
            *need,
        ));
    }
    constraints
}

fn constraint_recipes(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    all_productions: &[ResourceType],
) -> Vec<ConstraintReference> {
    let mut constraints = Vec::new();
    for production in all_productions {
        let resource = Resource {
            resource_type: *production,
            num: 0.0,
        };
        constraints.push(create_constraint(
            all_recipes,
            recipe_variables,
            problem,
            resource,
        ));
    }
    constraints
}

fn minimize_buildings_count(recipe_variables: &[Variable]) -> Expression {
    recipe_variables.iter().copied().sum::<Expression>()
}

pub fn solve(
    all_recipes: &[Recipe],
    all_productions: &[ResourceType],
    needs: &[Resource],
) -> Result<Vec<CalculatorSolution>, DspCalError> {
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
        all_productions,
    );

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
