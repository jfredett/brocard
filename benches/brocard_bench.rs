// This benchmark will verify the first billion `n` for the Brocard problem.
// It will use different implementations of this crate, and collect performance
// data and timing.
//
// The intent is to identify the most efficient way to verify values are not solutions.
// In the eventual distribution of this across multiple machines, the underlying verifier will
// undergo iteration to improve it's efficiency; so having this benchmark available and run on the
// hardware that will eventually be attacking this will make it easy to see which implementation
// is the one to use.
