use syn::{
    Type, Ident, Lit,
    parse::{Parse, ParseStream, Result}
};
use proc_macro2::TokenStream;

pub struct Optimizer {
    pub struct_name: Ident,
    pub vars: Variables,
    pub evaluate: TokenStream,
}
impl Parse for Optimizer {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_name: Ident = input.parse()?;

        let vars = {
            let content;
            braced!(content in input);
            content.parse::<Variables>()?
        };

        Ok(Optimizer {
            struct_name,
            vars,
            evaluate: input.parse()?,
        })
    }
}

pub struct Variables (pub Vec<Variable>);

impl Parse for Variables {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut vars = Vec::new();
        while input.lookahead1().peek(Ident) {
            vars.push(
                input.parse::<Variable>()?
                );
        }
        Ok(Variables (vars))
    }
}


pub struct Variable {
    pub name: Ident,
    pub ty: Type,
    pub low: TokenStream,
    pub high: TokenStream,
}
impl Parse for Variable {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;

        let low_sign: TokenStream = if input.lookahead1().peek(Token![-]) {input.parse::<Token![-]>().unwrap(); quote!{-}} else {quote!{}};
        let low: Lit = input.parse()?;
        input.parse::<Token![..]>()?;

        let high_sign: TokenStream = if input.lookahead1().peek(Token![-]) {input.parse::<Token![-]>().unwrap(); quote!{-}} else {quote!{}};
        let high: Lit = input.parse()?;

        // optional ','
        let _ = input.parse::<Token![,]>();
        Ok(Variable {
            name,
            ty,
            low: quote! {#low_sign #low},
            high: quote! {#high_sign #high},
        })
    }
}

