use quote::quote;
use syn::{bracketed, Ident, Lit, parse_macro_input, Token};
use syn::parse::{self, Parse, ParseStream};

struct BitSlice {
    name: Ident,
    start: Lit,
    end: Lit,
}

impl Parse for BitSlice {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let name: Ident = input.parse()?;
        let inner;
        bracketed!(inner in input);
        let start: Lit = inner.parse()?;
        inner.parse::<Token![:]>()?;
        let end: Lit = inner.parse()?;
        Ok(BitSlice {
            name,
            start,
            end,
        })
    }
}


/// Allows for taking subsets of Bits objects as a new Bits, with conventional bitwise syntax (e.g. 15:8 instead of 8..16)
#[proc_macro]
pub fn bitslice(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let BitSlice {
        name,
        start,
        end,
    } = parse_macro_input!(input as BitSlice);

    let start: usize = match start {
        Lit::Int(i) => {
            i.base10_parse().unwrap()
        }
        _ => {
            panic!("Expected integers in brackets - syntax is `bits[7:0]`");
        }
    };

    let end: usize = match end {
        Lit::Int(i) => {
            i.base10_parse().unwrap()
        }
        _ => {
            panic!("Expected integers in brackets - syntax is `bits[7:0]`");
        }
    };

    let low = start.min(end);
    let high = start.max(end);
    let width = high - low + 1;

    proc_macro::TokenStream::from(quote! {
        bitmath::Bits::<#width>::from_reverse_index(&#name.0,#high,#low).unwrap()
    })
}
