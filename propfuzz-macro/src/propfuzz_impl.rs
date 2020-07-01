// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::config::{
    ConfigBuilder, ParamConfig, ParamConfigBuilder, PropfuzzConfig, PropfuzzConfigBuilder,
};
use crate::errors::*;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    Attribute, AttributeArgs, Block, FnArg, Index, ItemFn, Lit, Meta, NestedMeta, Pat, PatType,
    Signature, Type,
};

pub(crate) fn propfuzz_impl(attr: AttributeArgs, item: ItemFn) -> Result<TokenStream, TokenStream> {
    let propfuzz_fn = match PropfuzzFn::new(&attr, &item) {
        Ok(propfuzz_fn) => propfuzz_fn,
        Err(err) => return Err(err.to_compile_error()),
    };

    Ok(propfuzz_fn.into_token_stream())
}

/// Processor for a propfuzz function.
#[derive(Debug)]
struct PropfuzzFn<'a> {
    name: &'a Ident,
    description: String,
    other_attrs: Vec<&'a Attribute>,
    config: PropfuzzConfig,
    struct_name: Ident,
    body: PropfuzzFnBody<'a>,
}

impl<'a> PropfuzzFn<'a> {
    const STRUCT_PREFIX: &'static str = "__PROPFUZZ__";

    /// Creates a new instance of `PropfuzzFn`.
    fn new(attr: &'a [NestedMeta], item: &'a ItemFn) -> Result<Self> {
        let mut errors = ErrorList::new();
        let mut config_builder = PropfuzzConfigBuilder::default();

        // Apply the arguments from the first #[propfuzz] invocation.
        config_builder.apply_args(attr, &mut errors);

        let name = &item.sig.ident;

        // Read the description from the doc comment.
        let description = {
            let description = item
                .attrs
                .iter()
                .filter_map(|attr| {
                    if attr.path.is_ident("doc") {
                        Some(extract_doc_comment(attr))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>>>()?;
            if description.is_empty() {
                errors.combine(Error::new_spanned(
                    &item.sig,
                    "#[propfuzz] requires a description as a doc comment",
                ));
            }
            description.join("\n")
        };

        // Read arguments from remaining #[propfuzz] attributes.
        let (propfuzz_attrs, other_attrs) = item
            .attrs
            .iter()
            .partition::<Vec<_>, _>(|attr| attr.path.is_ident("propfuzz"));

        config_builder.apply_attrs(propfuzz_attrs, &mut errors);

        let body = match PropfuzzFnBody::new(&item.sig, &item.block) {
            Ok(body) => body,
            Err(error) => return Err(errors.combine_finish(error)),
        };

        let struct_name = format_ident!("{}{}", Self::STRUCT_PREFIX, name);

        // If any errors were collected, return them.
        errors.finish()?;

        Ok(Self {
            name: &item.sig.ident,
            description,
            other_attrs,
            config: config_builder.finish(),
            struct_name,
            body,
        })
    }
}

impl<'a> ToTokens for PropfuzzFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            description,
            other_attrs,
            config,
            struct_name,
            body,
            ..
        } = self;

        let proptest_config = &config.proptest;
        let types = body.types();
        let name_pats = body.name_pats();

        // Use indexes as tuple accessors in fmt_value.
        // Note that we can't destructure values because name_pats may contain modifiers like mut.
        // TODO: modifiers like mut can be filtered out -- consider doing so for a nicer display.
        let indexes = (0..body.num_params()).map(Index::from);

        tokens.extend(quote! {
            #[test]
            #(#other_attrs )*
            fn #name() {
                ::propfuzz::runtime::execute_as_proptest(#struct_name);
            }

            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            struct #struct_name;

            impl ::propfuzz::traits::StructuredTarget for #struct_name {
                type Value = (#(#types,)*);

                fn name(&self) -> &'static str {
                    concat!(module_path!(), "::", stringify!(#name))
                }

                fn description(&self) -> &'static str {
                    #description
                }

                fn proptest_config(&self) -> ::propfuzz::proptest::test_runner::Config {
                    #proptest_config
                }

                fn execute(&self, __propfuzz_test_runner: &mut ::propfuzz::proptest::test_runner::TestRunner)
                    -> ::std::result::Result<(), ::propfuzz::proptest::test_runner::TestError<Self::Value>> {
                    #body
                }

                fn fmt_value(&self, value: &Self::Value, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    #(writeln!(f, "{} = {:?}", stringify!(#name_pats), value.#indexes)?;)*
                    Ok(())
                }
            }
        });
    }
}

fn extract_doc_comment(attr: &Attribute) -> Result<String> {
    match attr.parse_meta()? {
        Meta::NameValue(name_value) => match name_value.lit {
            Lit::Str(lit) => Ok(lit.value().trim().to_string()),
            _ => Err(Error::new_spanned(attr, "expected #[doc = r\"string\"]")),
        },
        _ => Err(Error::new_spanned(attr, "expected #[doc = r\"string\"]")),
    }
}

/// The body of a proptest function.
#[derive(Debug)]
struct PropfuzzFnBody<'a> {
    params: Vec<PropfuzzParam<'a>>,
    block: &'a Block,
}

impl<'a> PropfuzzFnBody<'a> {
    fn new(sig: &'a Signature, block: &'a Block) -> Result<Self> {
        if sig.inputs.is_empty() {
            return Err(Error::new_spanned(
                sig,
                "#[propfuzz] requires at least one argument",
            ));
        }

        let mut errors = ErrorList::new();

        let params = sig
            .inputs
            .iter()
            .filter_map(|param| match param {
                FnArg::Receiver(receiver) => {
                    errors.combine(Error::new_spanned(
                        receiver,
                        "#[propfuzz] is only supported on top-level functions",
                    ));
                    None
                }
                FnArg::Typed(param) => errors.combine_opt(|| PropfuzzParam::new(param)),
            })
            .collect::<Vec<_>>();

        // If there are any errors, return them.
        errors.finish()?;

        Ok(Self { params, block })
    }

    fn num_params(&self) -> usize {
        self.params.len()
    }

    fn types<'b>(&'b self) -> impl Iterator<Item = impl ToTokens + 'b> + 'b {
        self.params.iter().map(|param| param.ty)
    }

    fn strategies<'b>(&'b self) -> impl Iterator<Item = impl ToTokens + 'b> + 'b {
        self.params.iter().map(|param| param.config.strategy())
    }

    fn name_pats<'b>(&'b self) -> impl Iterator<Item = impl ToTokens + 'b> + 'b {
        self.params.iter().map(|param| param.name_pat)
    }
}

impl<'a> ToTokens for PropfuzzFnBody<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let strategies = self.strategies();
        let name_pats = self.name_pats();
        let block = self.block;

        tokens.extend(quote! {
            __propfuzz_test_runner.run(&(#(#strategies,)*), |(#(#name_pats,)*)| {
                // This is similar to proptest -- it ensures that the block itself doesn't
                // return a value, other than through an explicit `return` statement (as with
                // the prop_assert_ methods).
                let _: () = #block;
                Ok(())
            })
        });
    }
}

/// A propfuzz input parameter representing a random instance of a specific type, and a strategy to
/// generate that random instance.
#[derive(Debug)]
struct PropfuzzParam<'a> {
    name_pat: &'a Pat,
    ty: &'a Type,
    config: ParamConfig,
}

impl<'a> PropfuzzParam<'a> {
    fn new(param: &'a PatType) -> Result<Self> {
        let ty = &*param.ty;

        let mut errors = ErrorList::new();

        let mut config_builder = ParamConfigBuilder::new(ty);
        let (propfuzz_attrs, other_attrs) = param
            .attrs
            .iter()
            .partition::<Vec<_>, _>(|attr| attr.path.is_ident("propfuzz"));

        config_builder.apply_attrs(propfuzz_attrs, &mut errors);

        // Non-propfuzz attributes on arguments aren't recognized (there's nowhere to put them!)
        for attr in other_attrs {
            errors.combine(Error::new_spanned(
                attr,
                "non-#[propfuzz] attributes are not supported",
            ));
        }

        errors.finish()?;
        let config = config_builder.finish();

        Ok(Self {
            name_pat: &param.pat,
            ty,
            config,
        })
    }
}
