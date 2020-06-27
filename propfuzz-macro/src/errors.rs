// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::mem;
pub use syn::Error;

/// The `Result` type.
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub(crate) enum ErrorList {
    None,
    Some(Error),
}

impl ErrorList {
    pub(crate) fn new() -> Self {
        ErrorList::None
    }

    /// Combine this error with the existing list of errors.
    pub(crate) fn combine(&mut self, error: Error) {
        match self {
            ErrorList::None => {
                mem::replace(self, ErrorList::Some(error));
            }
            ErrorList::Some(original) => original.combine(error),
        }
    }

    /// Combine this error and return the consolidated list of errors, consuming `self`.
    pub(crate) fn combine_finish(self, error: Error) -> Error {
        match self {
            ErrorList::None => error,
            ErrorList::Some(mut original) => {
                original.combine(error);
                original
            }
        }
    }

    pub(crate) fn combine_fn<F>(&mut self, f: F)
    where
        F: FnOnce() -> Result<()>,
    {
        if let Err(error) = f() {
            self.combine(error);
        }
    }

    pub(crate) fn combine_opt<F, T>(&mut self, f: F) -> Option<T>
    where
        F: FnOnce() -> Result<T>,
    {
        match f() {
            Ok(val) => Some(val),
            Err(error) => {
                self.combine(error);
                None
            }
        }
    }

    pub(crate) fn finish(self) -> Result<()> {
        match self {
            ErrorList::None => Ok(()),
            ErrorList::Some(error) => Err(error),
        }
    }
}
