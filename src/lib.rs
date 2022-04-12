#![warn(missing_docs)]
/*!
Fixed point iterations library for SCF problems
*/

#[macro_use]
/// Core operations and mathematics
pub mod core;

/// Mixers from the solver module
pub mod solvers;

/// Everything our users need
pub mod prelude;

// #[cfg(test)]
// Top level testing
// mod testing;
