// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::fmt;

/// The `Result` type.
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

pub enum Error {
    Syn(syn::Error),
    Darling(darling::Error),
    TokenStream(TokenStream),
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::Syn(err)
    }
}

impl From<darling::Error> for Error {
    fn from(err: darling::Error) -> Self {
        Error::Darling(err)
    }
}

impl From<TokenStream> for Error {
    fn from(token_stream: TokenStream) -> Self {
        Error::TokenStream(token_stream)
    }
}

impl Error {
    pub fn new<T: fmt::Display>(span: Span, message: T) -> Self {
        Error::Syn(syn::Error::new(span, message))
    }

    pub fn new_spanned<T: ToTokens, U: fmt::Display>(tokens: T, message: U) -> Self {
        Error::Syn(syn::Error::new_spanned(tokens, message))
    }

    pub fn into_token_stream(self) -> TokenStream {
        match self {
            Error::Syn(err) => err.to_compile_error(),
            Error::Darling(err) => err.write_errors(),
            Error::TokenStream(token_stream) => token_stream,
        }
    }
}
