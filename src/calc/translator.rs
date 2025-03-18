use crate::calc;
use good_lp::{solvers::clarabel::ClarabelSolution, Solution};

use super::RecipeExtra;

pub fn from_clarabel_solution(
    all_recipes: &[RecipeExtra],
    clarabel_solution: &ClarabelSolution,
) -> Vec<calc::Solution> {
    let mut solutions = Vec::new();
    for recipe in all_recipes {
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
