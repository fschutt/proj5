#[cfg(all(not(target_arch = "wasm32"), feature = "scoped_threadpool"))]
use ThreadPool;

/// How should the coordinate transformation be parallelized?
#[repr(C)]
pub enum MultithreadingStrategy {
    /// Choose the single-core implementation
    SingleCore,
    /// Choose the multi-core implementation
    #[cfg(all(not(target_arch = "wasm32"), feature = "scoped_threadpool"))]
    MultiCore(ThreadPool),
}
