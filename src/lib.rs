extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::parse::{self, Parse, ParseStream};
use syn::Token;

#[proc_macro_attribute]
pub fn timed(args: TokenStream, input: TokenStream) -> TokenStream {
    let Args(move_token, format_args) = syn::parse_macro_input!(args);
    let mut input = syn::parse_macro_input!(input as syn::ItemFn);

    let body = &input.block;
    let return_ty = &input.sig.output;
    let err = Ident::new("err", Span::mixed_site());

    let new_body = if input.sig.asyncness.is_some() {
        let return_ty = match return_ty {
            syn::ReturnType::Default => {
                return syn::Error::new_spanned(input, "function should return Result")
                    .to_compile_error()
                    .into();
            }
            syn::ReturnType::Type(_, return_ty) => return_ty,
        };
        let result = Ident::new("result", Span::mixed_site());
        quote! {
            // let #result: #return_ty = async #move_token { #body }.await;
            // #result.map_err(|#err| ::eyre::Context.context(#err, format!(#format_args)).into())
        }
    } else {
        let force_fn_once = Ident::new("force_fn_once", Span::mixed_site());
        quote! {
            // // Moving a non-`Copy` value into the closure tells borrowck to always treat the closure
            // // as a `FnOnce`, preventing some borrowing errors.
            // let #force_fn_once = ::core::iter::empty::<()>();
            // (#move_token || #return_ty {
            //     ::core::mem::drop(#force_fn_once);
            //     #body
            // })().map_err(|#err| ::eyre::Context.context(#err, format!(#format_args)).into())
        }
    };

    input.block.stmts = vec![syn::Stmt::Expr(syn::Expr::Verbatim(new_body), None)];
    input.into_token_stream().into()
}

struct Args(Option<Token![move]>, TokenStream2);

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
        let move_token = if input.peek(Token![move]) {
            let token = input.parse()?;
            input.parse::<Token![,]>()?;
            Some(token)
        } else {
            None
        };
        Ok(Self(move_token, input.parse()?))
    }
}
