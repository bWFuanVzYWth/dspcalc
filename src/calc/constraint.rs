use good_lp::{
    constraint::ConstraintReference, solvers::clarabel::ClarabelProblem, Expression, SolverModel,
};

use crate::dsp::item::{Resource, ResourceType};

use super::{ProcessedRecipes, RecipeExtra};

// TODO 给公式命名

// 新constraint_recipes实现
pub fn constraint_recipes(
    processed: &ProcessedRecipes,
    problem: &mut ClarabelProblem,
    all_productions: &[ResourceType],
) -> Vec<ConstraintReference> {
    all_productions
        .iter()
        .map(|&prod| {
            let resource = Resource {
                resource_type: prod,
                num: 0.0,
            };
            create_constraint(processed, problem, resource)
        })
        .collect()
}

// 新constraint_needs实现
pub fn constraint_needs(
    processed: &ProcessedRecipes,
    problem: &mut ClarabelProblem,
    needs: &[Resource],
) -> Vec<ConstraintReference> {
    needs
        .iter()
        .map(|&need| create_constraint(processed, problem, need))
        .collect()
}

// 新create_constraint实现
fn create_constraint(
    processed: &ProcessedRecipes,
    problem: &mut ClarabelProblem,
    need: Resource,
) -> ConstraintReference {
    let (consumes, produces) = (
        processed.consumes.get(&need.resource_type),
        processed.produces.get(&need.resource_type),
    );

    let items_expr: Expression = consumes
        .into_iter()
        .flatten()
        .map(|(recipe, rate)| *rate * recipe.variable)
        .sum();

    let results_expr: Expression = produces
        .into_iter()
        .flatten()
        .map(|(recipe, rate)| *rate * recipe.variable)
        .sum();

    // problem.add_constraint_with_name(
    //     (results_expr - items_expr).geq(need.num),
    //     format!("{}_constraint", need.resource_type),
    // )
    problem.add_constraint((results_expr - items_expr).geq(need.num))
}
