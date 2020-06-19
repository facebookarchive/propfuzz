// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::errors::*;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    Attribute, AttributeArgs, Block, FnArg, ItemFn, Lit, Meta, MetaNameValue, Pat, PatType, Type,
};

pub(crate) fn propfuzz_impl(attr: AttributeArgs, item: ItemFn) -> Result<TokenStream, TokenStream> {
    let propfuzz_fn = match PropfuzzFn::new(&attr, &item) {
        Ok(propfuzz_fn) => propfuzz_fn,
        Err(err) => return Err(err.into_token_stream()),
    };

    Ok(propfuzz_fn.into_token_stream())
}

/// Processor for a propfuzz function.
#[derive(Debug)]
struct PropfuzzFn<'a> {
    name: &'a Ident,
    description: String,
    attrs: &'a Vec<Attribute>,
    config: PropfuzzFnConfig,
    struct_name: Ident,
    params: Vec<PropfuzzParam<'a>>,
    body: &'a Block,
}

impl<'a> PropfuzzFn<'a> {
    const STRUCT_PREFIX: &'static str = "__PROPFUZZ__";

    /// Creates a new instance of `PropfuzzFn`.
    fn new(attr: &'a AttributeArgs, item: &'a ItemFn) -> Result<Self> {
        #[derive(Debug, FromMeta)]
        struct MacroArgs {
            #[darling(default)]
            description: Option<String>,
            #[darling(default)]
            cases: Option<u32>,
            #[darling(default)]
            fork: bool,
        }

        let macro_args = MacroArgs::from_list(attr)?;

        let name = &item.sig.ident;
        let attrs = &item.attrs;

        // Read the description from the attribute, otherwise from the doc comment.
        let description = if let Some(description) = macro_args.description {
            description
        } else {
            let description = attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path.is_ident("doc") {
                        Some(extract_doc_string(attr))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>>>()?;
            description.join("\n")
        };

        let params = item
            .sig
            .inputs
            .iter()
            .map(|param| match param {
                FnArg::Receiver(receiver) => {
                    return Err(Error::new_spanned(
                        receiver,
                        "#[propfuzz] is only supported on top-level functions",
                    ));
                }
                FnArg::Typed(param) => PropfuzzParam::new(param),
            })
            .collect::<Result<Vec<_>>>()?;

        let struct_name = format_ident!("{}{}", Self::STRUCT_PREFIX, name);
        Ok(Self {
            name: &item.sig.ident,
            description,
            attrs,
            config: PropfuzzFnConfig {
                cases: macro_args.cases,
                fork: macro_args.fork,
            },
            struct_name,
            params,
            body: &item.block,
        })
    }
}

impl<'a> ToTokens for PropfuzzFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            attrs,
            name,
            description,
            struct_name,
            config,
            ..
        } = self;

        tokens.extend(quote! {
            #[test]
            #(#attrs )*
            fn #name() {
                ::propfuzz::runtime::execute_as_proptest(&#struct_name);
            }

            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            struct #struct_name;

            impl ::propfuzz::Propfuzz for #struct_name {
                fn name(&self) -> &'static str {
                    concat!(module_path!(), "::", stringify!(#name))
                }

                fn description(&self) -> &'static str {
                    #description
                }

                fn proptest_config(&self) -> ::propfuzz::proptest::test_runner::Config {
                    #config
                }

                fn execute(&self, __propfuzz_test_runner: &mut ::propfuzz::proptest::test_runner::TestRunner) {

                }
            }
        })
    }
}

/// Proptest config for a single function.
#[derive(Debug)]
struct PropfuzzFnConfig {
    cases: Option<u32>,
    fork: bool,
}

/// Generates a ProptestConfig for this function.
impl ToTokens for PropfuzzFnConfig {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fork, .. } = self;

        tokens.extend(quote! {
            let mut config = ::propfuzz::proptest::test_runner::Config::default();
            config.fork = #fork;
        });

        if let Some(cases) = self.cases {
            tokens.extend(quote! {
                config.cases = #cases;
            });
        }
        tokens.extend(quote! { config })
    }
}

fn extract_doc_string(attr: &Attribute) -> Result<String> {
    match attr.parse_meta()? {
        Meta::NameValue(name_value) => match name_value.lit {
            Lit::Str(lit) => Ok(lit.value().trim().to_string()),
            _ => Err(Error::new_spanned(attr, "expected #[doc = r\"string\"]")),
        },
        _ => Err(Error::new_spanned(attr, "expected #[doc = r\"string\"]")),
    }
}

/// A propfuzz input parameter representing a random instance of a specific type, and a strategy to
/// generate that random instance.
#[derive(Debug)]
struct PropfuzzParam<'a> {
    name_pat: &'a Pat,
    ty: &'a Type,
    // None means the default (any) strategy.
    strategy: Option<TokenStream>,
    other_attrs: Vec<&'a Attribute>,
}

impl<'a> PropfuzzParam<'a> {
    fn new(param: &'a PatType) -> Result<Self> {
        let (strategy_attrs, other_attrs) = param
            .attrs
            .iter()
            .partition::<Vec<_>, _>(|attr| attr.path.is_ident("strategy"));
        let mut strategy_iter = strategy_attrs.iter();
        let strategy = match strategy_iter.next() {
            Some(strategy) => {
                if let Some(other_strategy) = strategy_iter.next() {
                    return Err(Error::new_spanned(
                        other_strategy,
                        "an argument cannot have more than one strategy",
                    ));
                }
                Some(quote! { #strategy })
            }
            None => None,
        };

        Ok(Self {
            name_pat: &param.pat,
            ty: &param.ty,
            strategy,
            other_attrs,
        })
    }
}
