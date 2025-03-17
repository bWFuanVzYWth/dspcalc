use good_lp::{Expression, Variable};

// TODO 把各种权重统一api

pub fn minimize_buildings_count(recipe_variables: &[Variable]) -> Expression {
    recipe_variables.iter().copied().sum::<Expression>()
}

pub fn minimize_by_weight(recipe_variables: &[Variable], weights: &[f64]) -> Expression {
    weights
        .iter()
        .zip(recipe_variables.iter().copied())
        .map(|(weight, variable)| *weight * variable)
        .sum::<Expression>()
}
