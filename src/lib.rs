#![recursion_limit="128"]
extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate itertools;

use proc_macro::TokenStream;
use syn::{
    Type, Ident, Lit,
    parse::{Parse, ParseStream, Result}
};
use blackbox::BlackboxInput;
use logger::Logger;

mod parse;
use parse::*;


#[proc_macro]
pub fn make_optimizer(item: TokenStream) -> TokenStream {
    let Optimizer {struct_name, vars, evaluate} = parse_macro_input!(item as Optimizer);

    let names = vars.0.iter().map(|x| x.name.clone());
    let names2 = names.clone();
    let names3 = names.clone();
    let names4 = names.clone();
    let names5 = names.clone();
    let names6 = names.clone();
    let n_variables = names.len();

    let types = vars.0.iter().map(|x| x.ty.clone());
    let types2 = types.clone();
    let types3 = types.clone();

    let lows = vars.0.iter().map(|x| x.low.clone());
    let highs = vars.0.iter().map(|x| x.high.clone());

    let domains = izip!(types.clone(), lows.clone(), highs.clone()).map(|(ty, low, high)| {
        if let Type::Path (path) = ty  {
            match path.path.segments.first().unwrap().value().ident.to_string().as_str() {
                "usize" | "i64" | "u64" | "i32" | "u32" | "i16" | "u16" | "u8" | "i8" => quote! {
                    blackbox::Variable {
                        domain: blackbox::Domain::Discrete {
                            low: #low,
                            high: #high,
                        }
                    }
                },
                "f64" | "f32" => quote! {
                    blackbox::Variable {
                        domain: blackbox::Domain::Real {
                            low: #low,
                            high: #high,
                        }
                    }
                },
                e => panic!("Unsupported type {}", e),
            }
        } else {
            panic!("integer of float type required");
        }
    });

    let result = TokenStream::from(quote! {
        #[derive(Clone, Debug, Default)]
        pub struct #struct_name {
            #( pub #names: #types ),*
        }
        impl blackbox::BlackboxInput for #struct_name {
            fn evaluate(&self, mut logger: Option<Logger<String>>) -> f64 {
                let Self {#( #names4 ),*} = *self;
                #evaluate
            }
            fn random() -> Self {
                use rand::distributions::{Uniform, Distribution};
                let mut rng = rand::thread_rng();
                #struct_name {
                    #(
                        #names5: Uniform::new(#lows, #highs).sample(&mut rng),
                    )*
                }
            }
            fn to_numbers(&self) -> Vec<f64> {
                vec![
                    #(self.#names6 as f64),*
                ]
            }
            fn get_domains() -> Vec<blackbox::Variable> {
                vec![
                #(
                    #domains
                ),*
                ]
            }
            fn n_variables() -> usize {
                #n_variables
            }
             
        }

    });
    // println!("\n{}\n", result);
    result
}
