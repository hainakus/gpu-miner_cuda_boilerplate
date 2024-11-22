// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MinerError {
    #[error("CUDA error: {0}")]
    CudaError(#[from] cust::error::CudaError),
    #[error("Algorithm error: {0}")]
    AlgorithmError(String),
    // Add other error variants as needed
}