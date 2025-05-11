use crate::calc;
use good_lp::{solvers::clarabel::ClarabelSolution, Solution};

use super::RecipeBinding;

// 根据阈值过滤解，并把解转换成求解器无关的格式
pub fn from_clarabel_solution(
    recipes: &[RecipeBinding],
    clarabel_solution: &ClarabelSolution,
) -> Vec<calc::Solution> {
    const THRESHOLD: f64 = f32::EPSILON as f64;
    recipes
        .iter()
        .filter_map(|recipe| {
            let num = clarabel_solution.value(recipe.variable);
            if num > THRESHOLD {
                Some(calc::Solution {
                    recipe: recipe.recipe.clone(),
                    num,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
