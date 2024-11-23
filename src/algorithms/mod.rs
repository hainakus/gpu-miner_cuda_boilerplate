mod cuda;


// src/algorithms/mod.rs
use crate::gpu::GpuContext;
use crate::error::MinerError;

pub trait MiningAlgorithm {
    fn initialize(&mut self, gpu_context: &GpuContext) -> Result<(), MinerError>;
    fn mine(&mut self) -> Result<(), MinerError>;
    // Add other necessary methods
}