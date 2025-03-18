use good_lp::Expression;

use super::RecipeExtra;

pub fn minimize_by_weight(recipe: &[RecipeExtra]) -> Expression {
    recipe
        .iter()
        .map(|recipe| recipe.weight * recipe.variable)
        .sum::<Expression>()
}
