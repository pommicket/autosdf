extern crate quote;
extern crate proc_macro2;
extern crate syn;

use proc_macro::{TokenStream, TokenTree::{self, Punct, Literal}};
use quote::quote;

#[proc_macro_derive(GenRandom, attributes(prob))]
pub fn gen_random_derive(input: TokenStream) -> TokenStream {
	// Construct a representation of Rust code as a syntax tree
	// that we can manipulate
	let ast = syn::parse(input).unwrap();
	// Build the trait implementation
	impl_gen_random(&ast)
}


fn impl_gen_random(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	match &ast.data {
		syn::Data::Enum(enumeration) => {
			let variants = &enumeration.variants;
			let epsilon: f64 = 1e-9;
			let one_minus_epsilon = 1.0 - epsilon;
			let mut function_body = quote! {
				let mut variant: f64 = rng.gen_range(0.0..#one_minus_epsilon);
			};
			
			let mut test_variant = one_minus_epsilon;
			
			// parse enum fields
			for variant in variants.iter() {
				let probability: f64 = {
					let attr = variant.attrs.iter().find(|a| {
						let path = &a.path;
						if let Some(ident) = path.get_ident() {
							ident == "prob"
						} else {
							false
						}
					});
					let Some(attr) = attr else {
						panic!("Variant {} has no probability", variant.ident)
					};
					let tokens: TokenStream = attr.tokens.clone().into();
					let tokens: Vec<TokenTree> = tokens.into_iter().collect();
					if tokens.len() != 2 {
						panic!("Expected prob = <floating-point number>");
					}
					match &tokens[0] {
						Punct(equals) if equals.as_char() == '=' => {}
						_ => panic!("Expected = after prob attribute"),
					};
					
					let Literal(literal) = &tokens[1] else {
						panic!("Bad number for prob attribute.");
					};
					literal.to_string().parse().expect("Bad number for prob attribute")
				};
				
				let name = &variant.ident;
				
				
				let mut variant_arguments = quote! {};
				for field in variant.fields.iter() {
					if let Some(name) = &field.ident {
						variant_arguments.extend(quote! {#name: });
					}
					let ty = &field.ty;
					variant_arguments.extend(quote! { <#ty as GenRandom>::gen_random(rng), });
				}
				
				// surround the arguments with either () or {} brackets
				let constructor_group = match variant.fields {
					syn::Fields::Named(_) => {
						Some(proc_macro2::Group::new(
							proc_macro2::Delimiter::Brace,
							variant_arguments
						))
					},
					syn::Fields::Unnamed(_) => {
						Some(proc_macro2::Group::new(
							proc_macro2::Delimiter::Parenthesis,
							variant_arguments
						))
					},
					syn::Fields::Unit => None,
				};
				
				function_body.extend(quote! {
					variant -= #probability;
					if variant <= 0.0 { return Self::#name #constructor_group; }
				});
				test_variant -= probability;
				
				
			}
			
			if test_variant >= 0.0 || test_variant < -2.0 * epsilon {
				panic!("Probabilities for enum do not add up to 1 (final test_variant = {test_variant}).");
			}
			
			function_body.extend(quote! {
				panic!("RNG returned value outside of range.")
			});
			
			let gen = quote! {
				impl GenRandom for #name {
					fn gen_random(rng: &mut impl rand::Rng) -> Self {
						#function_body
					}
				}
			};
			//println!("{gen}");
			gen.into()
		},
		_ => panic!("derive(GenRandom) can currently only be applied to enums."),
	}
}

