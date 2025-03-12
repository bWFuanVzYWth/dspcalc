use good_lp::{Expression, Variable};

pub fn minimize_buildings_count(recipe_variables: &[Variable]) -> Expression {
    recipe_variables.iter().copied().sum::<Expression>()
}
