// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Configuration for propfuzz macros.

use crate::errors::{Error, ErrorList, Result};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Lit, Meta, MetaNameValue, NestedMeta};

/// Configuration for a propfuzz target.
#[derive(Debug, Default)]
pub(crate) struct PropfuzzConfigBuilder {
    cases: Option<u32>,
    fork: Option<bool>,
}

impl PropfuzzConfigBuilder {
    /// Adds arguments.
    pub(crate) fn apply_args<'a>(
        &mut self,
        args: impl IntoIterator<Item = &'a NestedMeta>,
        errors: &mut ErrorList,
    ) {
        for arg in args {
            self.apply_arg(arg, errors)
        }
    }

    fn apply_arg(&mut self, arg: &NestedMeta, errors: &mut ErrorList) {
        match arg {
            NestedMeta::Meta(meta) => {
                if meta.path().is_ident("cases") {
                    errors.combine_fn(|| {
                        let cases = read_u32(meta)?;
                        replace_if_empty(meta.span(), &mut self.cases, cases)
                    });
                } else if meta.path().is_ident("fork") {
                    errors.combine_fn(|| {
                        let fork = read_bool(meta)?;
                        replace_if_empty(meta.span(), &mut self.fork, fork)
                    });
                } else {
                    errors.combine(Error::new_spanned(meta.path(), "argument not recognized"));
                }
            }
            NestedMeta::Lit(meta) => {
                errors.combine(Error::new_spanned(meta, "expected key = value format"));
            }
        }
    }

    /// Completes building args and returns a `PropfuzzConfig`.
    pub(crate) fn finish(self) -> PropfuzzConfig {
        let cases = self.cases;
        let fork = self.fork.unwrap_or(false);

        PropfuzzConfig {
            proptest: ProptestConfig { cases, fork },
        }
    }
}

/// Overall config for a single propfuzz function, fully built.
#[derive(Debug)]
pub(crate) struct PropfuzzConfig {
    pub(crate) proptest: ProptestConfig,
}

/// Proptest config for a single propfuzz function.
#[derive(Debug, Default)]
pub(crate) struct ProptestConfig {
    pub(crate) cases: Option<u32>,
    pub(crate) fork: bool,
}

/// Generates a ProptestConfig for this function.
impl ToTokens for ProptestConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { cases, fork } = self;

        tokens.extend(quote! {
            let mut config = ::propfuzz::test::test_runner::Config::default();
            config.fork = #fork;
            config.source_file = Some(file!());
        });

        if let Some(cases) = cases {
            tokens.extend(quote! {
                config.cases = #cases;
            });
        }
        tokens.extend(quote! { config })
    }
}

fn read_bool(meta: &Meta) -> Result<bool> {
    let name_value = name_value(meta)?;
    match &name_value.lit {
        Lit::Bool(lit) => Ok(lit.value),
        _ => Err(Error::new_spanned(&name_value.lit, "expected bool")),
    }
}

fn read_u32(meta: &Meta) -> Result<u32> {
    let name_value = name_value(meta)?;
    match &name_value.lit {
        Lit::Int(lit) => Ok(lit.base10_parse::<u32>()?),
        _ => Err(Error::new_spanned(&name_value.lit, "expected integer")),
    }
}

fn name_value(meta: &Meta) -> Result<&MetaNameValue> {
    match meta {
        Meta::NameValue(meta) => Ok(meta),
        _ => Err(Error::new_spanned(meta, "expected key = value format")),
    }
}

fn replace_if_empty<T>(span: Span, dest: &mut Option<T>, val: T) -> Result<()> {
    if dest.replace(val).is_some() {
        Err(Error::new(span, "key specified more than once"))
    } else {
        Ok(())
    }
}
