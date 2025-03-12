use good_lp::{constraint::ConstraintReference, solvers::clarabel::ClarabelProblem, Expression, SolverModel, Variable};

use crate::dsp::{item::{Resource, ResourceType}, recipe::Recipe};

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

pub fn constraint_needs(
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

pub fn constraint_recipes(
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