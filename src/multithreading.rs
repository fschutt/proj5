#[cfg(not(target_arch = "wasm32"))]
use ThreadPool;

/// How should the coordinate transformation be parallelized?
#[repr(C)]
pub enum MultithreadingStrategy {
    /// Choose the single-core implementation
    SingleCore,
    /// Choose the multi-core implementation
    #[cfg(not(target_arch = "wasm32"))]
    MultiCore(ThreadPool),
    /// Only try OpenCL
    #[cfg(not(target_arch = "wasm32"))]
    OpenClOnly,
    /// Choose multithreading if initializing OpenCL doesn't work
    #[cfg(not(target_arch = "wasm32"))]
    OpenClThenMultithreading(ThreadPool),
    /// Choose a single thread if initializing OpenCL doesn't work 
    #[cfg(not(target_arch = "wasm32"))]
    OpenClThenSinglethreading,
}
