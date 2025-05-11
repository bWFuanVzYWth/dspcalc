use good_lp::Expression;

use super::RecipeExtra;

/// 根据传入的权重列表，创建代价表达式
pub fn minimize_by_weight(recipe: &[RecipeExtra]) -> Expression {
    recipe
        .iter()
        .map(|recipe| recipe.weight * recipe.variable)
        .sum::<Expression>()
}
