use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspCalError {
    #[error("mismatched recipe weights: recipes count:{0}, weights count:{0}")]
    MismatchedRecipeWeights(usize, usize),
    #[error("unknown building id: {0}")]
    UnknownBuildingType(i64),
    #[error("lp solver error: {0}")]
    LpSolverError(good_lp::ResolutionError),
}
