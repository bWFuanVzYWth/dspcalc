use good_lp::{Expression, Variable};

use super::RecipeExtra;

// TODO 把各种权重统一api

pub fn minimize_buildings_count(recipe_variables: &[Variable]) -> Expression {
    recipe_variables.iter().copied().sum::<Expression>()
}

pub fn minimize_by_weight(recipe: &[RecipeExtra]) -> Expression {
    recipe
        .iter()
        .map(|recipe| recipe.weight * recipe.variable)
        .sum::<Expression>()
}
