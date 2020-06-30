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
    proptest: ProptestConfig,
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
                let path = meta.path();

                if path.is_ident("cases") {
                    errors.combine_fn(|| {
                        replace_empty(meta.span(), &mut self.proptest.cases, read_u32(meta)?)
                    });
                } else if path.is_ident("max_local_rejects") {
                    errors.combine_fn(|| {
                        replace_empty(
                            meta.span(),
                            &mut self.proptest.max_local_rejects,
                            read_u32(meta)?,
                        )
                    });
                } else if path.is_ident("max_global_rejects") {
                    errors.combine_fn(|| {
                        replace_empty(
                            meta.span(),
                            &mut self.proptest.max_global_rejects,
                            read_u32(meta)?,
                        )
                    });
                } else if path.is_ident("max_flat_map_regens") {
                    errors.combine_fn(|| {
                        replace_empty(
                            meta.span(),
                            &mut self.proptest.max_flat_map_regens,
                            read_u32(meta)?,
                        )
                    });
                } else if path.is_ident("fork") {
                    errors.combine_fn(|| {
                        replace_empty(meta.span(), &mut self.proptest.fork, read_bool(meta)?)
                    });
                } else if path.is_ident("timeout") {
                    errors.combine_fn(|| {
                        replace_empty(meta.span(), &mut self.proptest.timeout, read_u32(meta)?)
                    });
                } else if path.is_ident("max_shrink_time") {
                    errors.combine_fn(|| {
                        replace_empty(
                            meta.span(),
                            &mut self.proptest.max_shrink_time,
                            read_u32(meta)?,
                        )
                    });
                } else if path.is_ident("max_shrink_iters") {
                    errors.combine_fn(|| {
                        replace_empty(
                            meta.span(),
                            &mut self.proptest.max_shrink_iters,
                            read_u32(meta)?,
                        )
                    });
                } else if path.is_ident("verbose") {
                    errors.combine_fn(|| {
                        replace_empty(meta.span(), &mut self.proptest.verbose, read_u32(meta)?)
                    });
                } else {
                    errors.combine(Error::new_spanned(path, "argument not recognized"));
                }
            }
            NestedMeta::Lit(meta) => {
                errors.combine(Error::new_spanned(meta, "expected key = value format"));
            }
        }
    }

    /// Completes building args and returns a `PropfuzzConfig`.
    pub(crate) fn finish(self) -> PropfuzzConfig {
        PropfuzzConfig {
            proptest: self.proptest,
        }
    }
}

/// Overall config for a single propfuzz function, fully built.
#[derive(Debug)]
pub(crate) struct PropfuzzConfig {
    pub(crate) proptest: ProptestConfig,
}

/// Proptest config for a single propfuzz function.
///
/// This contains most of the settings in proptest's config.
#[derive(Debug, Default)]
pub(crate) struct ProptestConfig {
    cases: Option<u32>,
    max_local_rejects: Option<u32>,
    max_global_rejects: Option<u32>,
    max_flat_map_regens: Option<u32>,
    fork: Option<bool>,
    timeout: Option<u32>,
    max_shrink_time: Option<u32>,
    max_shrink_iters: Option<u32>,
    verbose: Option<u32>,
}

macro_rules! extend_config {
    ($tokens:ident, $var:ident) => {
        if let Some($var) = $var {
            $tokens.extend(quote! {
                config.$var = #$var;
            })
        }
    };
}

/// Generates a ProptestConfig for this function.
impl ToTokens for ProptestConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            cases,
            max_local_rejects,
            max_global_rejects,
            max_flat_map_regens,
            fork,
            timeout,
            max_shrink_time,
            max_shrink_iters,
            verbose,
        } = self;

        tokens.extend(quote! {
            let mut config = ::propfuzz::proptest::test_runner::Config::default();
            config.source_file = Some(file!());
        });

        extend_config!(tokens, cases);
        extend_config!(tokens, max_local_rejects);
        extend_config!(tokens, max_global_rejects);
        extend_config!(tokens, max_flat_map_regens);
        extend_config!(tokens, fork);
        extend_config!(tokens, timeout);
        extend_config!(tokens, max_shrink_time);
        extend_config!(tokens, max_shrink_iters);
        extend_config!(tokens, verbose);

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

fn replace_empty<T>(span: Span, dest: &mut Option<T>, val: T) -> Result<()> {
    if dest.replace(val).is_some() {
        Err(Error::new(span, "key specified more than once"))
    } else {
        Ok(())
    }
}
