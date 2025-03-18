use good_lp::{
    constraint::ConstraintReference, solvers::clarabel::ClarabelProblem, Expression, SolverModel,
};

use crate::dsp::item::{Resource, ResourceType};

use super::RecipeExtra;

// TODO 给公式命名

// 需要两个lut，一个是物品 -> 相关公式 -> 公式变量
// FIXME 生成这个约束花了大概40%的时间，可以利用配方矩阵的稀疏性大幅度优化
fn create_constraint(
    recipes: &[RecipeExtra],
    problem: &mut ClarabelProblem,
    need: Resource,
) -> good_lp::constraint::ConstraintReference {
    let items_expr: Expression = recipes
        .iter()
        .map(|recipe| {
            recipe
                .recipe
                .items
                .iter()
                .filter(|item| item.resource_type == need.resource_type)
                .map(|item| item.num / recipe.recipe.time * recipe.variable)
                .sum::<Expression>()
        })
        .sum();

    let results_expr: Expression = recipes
        .iter()
        .map(|recipe| {
            recipe
                .recipe
                .results
                .iter()
                .filter(|result| result.resource_type == need.resource_type)
                .map(|result| result.num / recipe.recipe.time * recipe.variable)
                .sum::<Expression>()
        })
        .sum();

    // 构建约束：对于某种物品，总产出速率 - 总消耗速率 ≥ 额外净需求速率
    problem.add_constraint((results_expr - items_expr).geq(need.num))
}

pub fn constraint_needs(
    all_recipes: &[RecipeExtra],
    problem: &mut ClarabelProblem,
    needs: &[Resource],
) -> Vec<ConstraintReference> {
    let mut constraints = Vec::new();
    for need in needs {
        constraints.push(create_constraint(all_recipes, problem, *need));
    }
    constraints
}

pub fn constraint_recipes(
    recipes: &[RecipeExtra],
    problem: &mut ClarabelProblem,
    all_productions: &[ResourceType],
) -> Vec<ConstraintReference> {
    let mut constraints = Vec::new();
    for production in all_productions {
        let resource = Resource {
            resource_type: *production,
            num: 0.0,
        };
        constraints.push(create_constraint(recipes, problem, resource));
    }
    constraints
}
