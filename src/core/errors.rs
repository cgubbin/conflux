/*!
This module defines the errors for the core crate
*/

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
/// Error type for the crate
pub enum FixedPointError {
    #[error("Unimplemented operation")]
    #[diagnostic(help("Check the default implementation for FixedPoint"), url(docsrs))]
    /// Error warns when a required trait or operation is unimplemented
    UnimplementedOperation,
    #[error("Unexpected outcome")]
    #[diagnostic(help("An error i have not forseen has occurred"), url(docsrs))]
    /// This error is called if the iteration terminates but should not have
    UnexpectedOutcome,
    #[error("Exceeded maximum iterations")]
    #[diagnostic(
        help("Maybe try to relax your target tolerance, or adjust solver parameters"),
        url(docsrs)
    )]
    /// Error to warn when the iteration exceeds the maximum prescribed number
    /// The current cost is attached so as the degree of failure can be assessed
    TooManyIterations(f64),
    #[error("Failed to evaluate update")]
    #[diagnostic(
        help("Evaluating the update function failed, check in the upstream crate"),
        url(docsrs)
    )]
    /// Error to warn when evaluation of the update, defined in the calling crate, fails
    UpdateFailed,
    #[error("Solution diverged")]
    #[diagnostic(
        help("The solution now contains NaN values, indicating it has overflown"),
        url(docsrs)
    )]
    /// Error to warn when the solution vector has overflown
    NumericalDivergence,
}
