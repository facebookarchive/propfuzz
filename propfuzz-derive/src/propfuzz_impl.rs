// Copyright (c) The propfuzz Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::errors::*;
use darling::FromMeta;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    Attribute, AttributeArgs, Block, Expr, FnArg, Index, ItemFn, Lit, Meta, NestedMeta, Pat,
    PatType, Signature, Type,
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
    body: PropfuzzFnBody<'a>,
}

impl<'a> PropfuzzFn<'a> {
    const STRUCT_PREFIX: &'static str = "__PROPFUZZ__";

    /// Creates a new instance of `PropfuzzFn`.
    fn new(attr: &'a [NestedMeta], item: &'a ItemFn) -> Result<Self> {
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
                        Some(extract_doc_comment(attr))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<_>>>()?;
            description.join("\n")
        };

        let body = PropfuzzFnBody::new(&item.sig, &item.block)?;

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
            body,
        })
    }
}

impl<'a> ToTokens for PropfuzzFn<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            attrs,
            name,
            description,
            config,
            struct_name,
            body,
            ..
        } = self;

        let types = body.types();
        let name_pats = body.name_pats();

        // Use indexes as tuple accessors in fmt_value.
        // Note that we can't destructure values because name_pats may contain modifiers like mut.
        // TODO: modifiers like mut can be filtered out -- consider doing so for a nicer display.
        let indexes = (0..body.num_params()).map(Index::from);

        tokens.extend(quote! {
            #[test]
            #(#attrs )*
            fn #name() {
                ::propfuzz::runtime::execute_as_proptest(#struct_name);
            }

            #[derive(Copy, Clone, Debug)]
            #[allow(non_camel_case_types)]
            struct #struct_name;

            impl ::propfuzz::Propfuzz for #struct_name {
                type Value = (#(#types,)*);

                fn name(&self) -> &'static str {
                    concat!(module_path!(), "::", stringify!(#name))
                }

                fn description(&self) -> &'static str {
                    #description
                }

                fn proptest_config(&self) -> ::propfuzz::test::test_runner::Config {
                    #config
                }

                fn execute(&self, __propfuzz_test_runner: &mut ::propfuzz::test::test_runner::TestRunner)
                    -> ::std::result::Result<(), ::propfuzz::test::test_runner::TestError<Self::Value>> {
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

/// Proptest config for a single function.
#[derive(Debug)]
struct PropfuzzFnConfig {
    cases: Option<u32>,
    fork: bool,
}

/// Generates a ProptestConfig for this function.
impl ToTokens for PropfuzzFnConfig {
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

        let params = sig
            .inputs
            .iter()
            .map(|param| match param {
                FnArg::Receiver(receiver) => Err(Error::new_spanned(
                    receiver,
                    "#[propfuzz] is only supported on top-level functions",
                )),
                FnArg::Typed(param) => PropfuzzParam::new(param),
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { params, block })
    }

    fn num_params(&self) -> usize {
        self.params.len()
    }

    fn types<'b>(&'b self) -> impl Iterator<Item = impl ToTokens + 'b> + 'b {
        self.params.iter().map(|param| param.ty)
    }

    fn strategies<'b>(&'b self) -> impl Iterator<Item = impl ToTokens + 'b> + 'b {
        self.params.iter().map(|param| &param.strategy)
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
    strategy: TokenStream,
    other_attrs: Vec<&'a Attribute>,
}

impl<'a> PropfuzzParam<'a> {
    fn new(param: &'a PatType) -> Result<Self> {
        let ty = &*param.ty;

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
                let strategy = strategy.parse_args::<Expr>()?;
                quote! { #strategy }
            }
            None => quote! { ::propfuzz::test::arbitrary::any::<#ty>() },
        };

        Ok(Self {
            name_pat: &param.pat,
            ty,
            strategy,
            other_attrs,
        })
    }
}
