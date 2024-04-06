use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{self, Parse, ParseStream},
    ItemFn, Token,
};

#[proc_macro_attribute]
pub fn timed(attr: TokenStream, item: TokenStream) -> TokenStream {
    // let Args(a, b) = syn::parse_macro_input!(attr);
    let item_fn = syn::parse_macro_input!(item as ItemFn);

    let vis = &item_fn.vis;
    let fn_name = &item_fn.sig.ident;
    let fn_block = &item_fn.block;

    let output = quote! {
        #vis fn #fn_name() {
            let start_time = std::time::Instant::now();
            #fn_block
            let duration = start_time.elapsed();
            println!("Execution time of {}(): {:.6} seconds", stringify!(#fn_name), duration.as_secs_f64());
        }
    };

    TokenStream::from(output)
}

// struct Args(Option<Token![move]>, proc_macro2::TokenStream);

// impl Parse for Args {
//     fn parse(input: ParseStream<'_>) -> parse::Result<Self> {
//         let move_token = if input.peek(Token![move]) {
//             let token = input.parse()?;
//             input.parse::<Token![,]>()?;
//             Some(token)
//         } else {
//             None
//         };
//         Ok(Self(move_token, input.parse()?))
//     }
// }
