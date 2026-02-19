// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>,
// Kevin Knapp (@kbknapp) <kbknapp@gmail.com>, and
// Ana Hobden (@hoverbear) <operator@hoverbear.org>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This work was derived from Structopt (https://github.com/TeXitoi/structopt)
// commit#ea76fa1b1b273e65e3b0b1046643715b49bec51f which is licensed under the
// MIT/Apache 2.0 license.
mod args;
mod into_app;
mod parser;
mod subcommand;
mod value_enum;

pub(crate) use self::parser::derive_parser;
pub(crate) use args::derive_args;
pub(crate) use subcommand::derive_subcommand;
pub(crate) use value_enum::derive_value_enum;

const ALLOC_CRATE: lazy_tokens::LazyTokens<syn::TypePath> = lazy_tokens::LazyTokens::new(|| {
    #[cfg(feature = "std")]
    return syn::parse_quote!(::std);
    #[cfg(not(feature = "std"))]
    return syn::parse_quote!(::alloc);
});

mod lazy_tokens {
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use std::ops::Deref;
    use std::sync::LazyLock;
    pub(super) struct LazyTokens<T>(LazyLock<T>);
    impl<T> LazyTokens<T> {
        pub(super) const fn new(init: fn() -> T) -> Self {
            Self(LazyLock::new(init))
        }
    }
    impl<T: ToTokens> ToTokens for LazyTokens<T> {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.deref().to_tokens(tokens)
        }
    }
}
