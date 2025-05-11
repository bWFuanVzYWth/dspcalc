use good_lp::Expression;

use super::RecipeBinding;

/// 根据传入的权重列表，创建代价表达式
pub fn minimize_by_weight(recipe: &[RecipeBinding]) -> Expression {
    recipe
        .iter()
        .map(|recipe| recipe.weight * recipe.variable)
        .sum::<Expression>()
}
