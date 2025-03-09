use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspCalError {
    #[error("unknown lp variable id:{0}")]
    UnknownLpVarId(usize),
    #[error("lp solver error: ")]
    LpSolverError(good_lp::ResolutionError),
}
