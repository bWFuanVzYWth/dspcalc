use crate::calc;
use good_lp::{solvers::clarabel::ClarabelSolution, Solution};

use super::RecipeBinding;

pub fn from_clarabel_solution(
    recipes: &[RecipeBinding],
    clarabel_solution: &ClarabelSolution,
) -> Vec<calc::Solution> {
    let mut solutions = Vec::new();
    for recipe in recipes {
        let num = clarabel_solution.value(recipe.variable);
        if num > f64::from(f32::EPSILON) {
            let solution = calc::Solution {
                recipe: recipe.recipe.clone(),
                num,
            };
            solutions.push(solution);
        }
    }
    solutions
}
