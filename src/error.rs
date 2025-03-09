use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspCalError {
    #[error("unknown bi map")]
    UnknownBiMap,
    #[error("lp solver error: ")]
    LpSolverError(good_lp::ResolutionError)
}
