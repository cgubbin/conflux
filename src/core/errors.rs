//! This module defines the errors for the core crate

use thiserror::Error;
use miette::Diagnostic;

#[derive(Error, Diagnostic, Debug)]
pub enum FixedPointError {
    #[error(transparent)]
    #[diagnostic(code(my_lib::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Oops it blew up")]
    #[diagnostic(code(my_lib::bad_code))]
    BadThingHappened,

    #[error("Unimplemented operation")]
    #[diagnostic(
        help("Check the default implementation for FixedPoint"),
        url(docsrs)
    )]
    UnimplementedOperation,
}
