use good_lp::{
    constraint::ConstraintReference, solvers::clarabel::ClarabelProblem, Expression, SolverModel,
};

use super::ProcessedRecipes;
use crate::dsp::item::{Resource, ResourceType};

/// 创建所有公式约束
///
/// 本质是把公式约束视为需求量为0的需求约束
pub fn constraint_recipes(
    processed: &ProcessedRecipes,
    problem: &mut ClarabelProblem,
    production_types: &[ResourceType],
) -> Vec<ConstraintReference> {
    let needs = production_types
        .iter()
        .map(|&production| Resource {
            resource_type: production,
            num: 0.0,
        })
        .collect::<Vec<_>>();
    constraint_needs(processed, problem, &needs)
}

/// 创建所有需求约束
///
/// 对需求列表中的每一项资源创建一个需求约束，返回相应的约束引用列表
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

/// 创建一个需求约束
///
/// 对所有出现的配方，产出量**总和** - 消耗量**总和** >= 需求量
fn create_constraint(
    processed: &ProcessedRecipes,
    problem: &mut ClarabelProblem,
    need: Resource,
) -> ConstraintReference {
    let (consumes, produces) = (
        // 当 get() 返回 None 时，sum() 会默认为 0
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
