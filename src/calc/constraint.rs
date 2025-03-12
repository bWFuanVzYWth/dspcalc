use good_lp::{
    constraint::ConstraintReference, solvers::clarabel::ClarabelProblem, Expression, SolverModel,
    Variable,
};

use crate::{
    dsp::{
        item::{Resource, ResourceType},
        recipe::Recipe,
    },
    error::DspCalError,
};

// FIXME 生成这个约束花了大概40%的时间，可以利用配方矩阵的稀疏性大幅度优化
/// 构建约束：对于每种产物，总产出速率 - 总消耗速率 ≥ 额外净需求速率
fn create_constraint(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    need: Resource,
) -> Result<good_lp::constraint::ConstraintReference, DspCalError> {
    // 处理 items_expr
    let items_expr: Expression = recipe_variables
        .iter()
        .enumerate()
        .map(|(recipes_index, variable)| {
            // 获取 recipe，错误时直接返回
            let recipe = all_recipes
                .get(recipes_index)
                .ok_or(DspCalError::UnknownLpVarId(recipes_index))?;

            // 计算当前 recipe 的贡献
            let expr = recipe
                .items
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / recipe.time) * (*variable))
                .sum::<Expression>();

            Ok(expr)
        })
        // 收集所有 Result 并处理错误
        .collect::<Result<Vec<Expression>, _>>()?
        // 将所有表达式求和
        .into_iter()
        .sum();

    // 处理 results_expr（逻辑与 items_expr 相同）
    let results_expr: Expression = recipe_variables
        .iter()
        .enumerate()
        .map(|(recipes_index, variable)| {
            let recipe = all_recipes
                .get(recipes_index)
                .ok_or(DspCalError::UnknownLpVarId(recipes_index))?;

            let expr = recipe
                .results
                .iter()
                .filter(|product| product.resource_type == need.resource_type)
                .map(|product| (product.num / recipe.time) * (*variable))
                .sum::<Expression>();

            Ok(expr)
        })
        .collect::<Result<Vec<Expression>, _>>()?
        .into_iter()
        .sum();

    // 添加约束
    Ok(problem.add_constraint((results_expr - items_expr).geq(need.num)))
}

pub fn constraint_needs(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    needs: &[Resource],
) -> Result<Vec<ConstraintReference>, DspCalError> {
    let mut constraints = Vec::new();
    for need in needs {
        constraints.push(create_constraint(
            all_recipes,
            recipe_variables,
            problem,
            *need,
        )?);
    }
    Ok(constraints)
}

pub fn constraint_recipes(
    all_recipes: &[Recipe],
    recipe_variables: &[Variable],
    problem: &mut ClarabelProblem,
    all_productions: &[ResourceType],
) -> Result<Vec<ConstraintReference>, DspCalError> {
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
        )?);
    }
    Ok(constraints)
}
