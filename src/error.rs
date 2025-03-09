use thiserror::Error;

#[derive(Error, Debug)]
pub enum DspCalError {
    #[error("unknown bi map")]
    UnknownBiMap,
}
