use good_lp::{Expression, Variable};

pub fn minimize_buildings_count(recipe_variables: &[Variable]) -> Expression {
    recipe_variables.iter().copied().sum::<Expression>()
}

pub fn minimize_by_weight(recipe_variables: &[Variable], weigths: &[f64]) -> Expression {
    weigths
        .iter()
        .zip(recipe_variables.iter().copied())
        .map(|(weight, variable)| *weight * variable)
        .sum::<Expression>()
}
