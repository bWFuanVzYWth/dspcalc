use good_lp::{
    constraint::ConstraintReference, solvers::clarabel::ClarabelProblem, Expression, SolverModel,
};

use super::ProcessedRecipes;
use crate::dsp::item::{Resource, ResourceType};

// TODO 合并 constraint_recipes 和 constraint_needs 逻辑

pub fn constraint_recipes(
    processed: &ProcessedRecipes,
    problem: &mut ClarabelProblem,
    productions: &[ResourceType],
) -> Vec<ConstraintReference> {
    productions
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

    problem.add_constraint((results_expr - items_expr).geq(need.num))
}
