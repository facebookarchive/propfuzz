// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;
use syn::spanned::Spanned;
use syn::{Block, FnArg, ItemFn, PatType};

pub(crate) fn propfuzz_impl(item: ItemFn) -> Result<TokenStream, TokenStream> {
    let propfuzz_fn = PropfuzzFn::new(&item)?;

    unimplemented!()
}

/// Processor for a propfuzz function.
struct PropfuzzFn<'a> {
    name: &'a Ident,
    args: Vec<&'a PatType>,
    body: &'a Block,
}

impl<'a> PropfuzzFn<'a> {
    /// Creates a new instance.
    fn new(item: &'a ItemFn) -> Result<Self, TokenStream> {
        let args = item.sig.inputs.iter().map(|arg|
            match arg {
                FnArg::Receiver(receiver) => {
                    Err(
                        quote_spanned! { receiver.span() => compile_error!("#[propfuzz] is only supported on top-level functions") },
                    )
                }
                FnArg::Typed(arg) => Ok(arg),
            }
        ).collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name: &item.sig.ident,
            args,
            body: &item.block,
        })
    }
}
