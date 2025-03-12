use crate::{dsp::recipe::Recipe, error::DspCalError, error::DspCalError::UnknownLpVarId};
use good_lp::{solvers::clarabel::ClarabelSolution, Solution};

pub struct CalculatorSolution {
    pub recipe: Recipe,
    pub num: f64,
}

pub fn from_clarabel_solution(
    recipe_variables: &[good_lp::Variable],
    all_recipes: &[Recipe],
    clarabel_solution: &ClarabelSolution,
) -> Result<Vec<CalculatorSolution>, DspCalError> {
    let mut solutions = Vec::new();
    for (i, recipe) in all_recipes.iter().enumerate() {
        let num = clarabel_solution.value(*recipe_variables.get(i).ok_or(UnknownLpVarId(i))?);
        if num > f64::from(f32::EPSILON) {
            let solution = CalculatorSolution {
                recipe: recipe.clone(),
                num,
            };
            solutions.push(solution);
        }
    }
    Ok(solutions)
}
