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
}
