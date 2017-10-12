use ThreadPool;

/// How should the coordinate transformation be parallelized?
pub enum MultithreadingStrategy {
    /// Choose the single-core implementation
    SingleCore,
    /// Choose the multi-core implementation
    MultiCore(ThreadPool),
    /// Only try OpenCL
    OpenClOnly,
    /// Choose multithreading if initializing OpenCL doesn't work 
    OpenClThenMultithreading(ThreadPool),
    /// Choose a single thread if initializing OpenCL doesn't work 
    OpenClThenSinglethreading,
}
