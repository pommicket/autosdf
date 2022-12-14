extern crate quote;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree as TokenTree2;
use std::str::FromStr;
use std::any::type_name;
use quote::quote;

#[proc_macro_derive(GenRandom, attributes(prob, scale, bias))]
pub fn gen_random_derive(input: TokenStream) -> TokenStream {
	// Construct a representation of Rust code as a syntax tree
	// that we can manipulate
	let ast = syn::parse(input).unwrap();
	// Build the trait implementation
	impl_gen_random(&ast)
}

fn get_attribute_literal(attrs: &[syn::Attribute], name: &str) -> Option<proc_macro2::Literal> {
	let attr = attrs.iter().find(|a| {
		let path = &a.path;
		if let Some(ident) = path.get_ident() {
			ident == name
		} else {
			false
		}
	})?;

	let tokens: TokenStream2 = attr.tokens.clone().into();
	let mut tokens: Vec<TokenTree2> = tokens.into_iter().collect();
	if tokens.len() != 2 {
		panic!("Expected {name} = <value>");
	}
	use TokenTree2::{Punct, Literal};
	match &tokens[0] {
		Punct(equals) if equals.as_char() == '=' => {}
		_ => panic!("Expected = after {name} attribute"),
	};
	
	let Literal(literal) = tokens.remove(1) else {
		panic!("Bad value for {name} attribute.");
	};
	Some(literal)
}

fn parse_attribute_value<T: FromStr>(attrs: &[syn::Attribute], name: &str) -> Option<T> {
	let literal = get_attribute_literal(attrs, name)?;
	let Ok(value) = literal.to_string().parse() else {
		panic!("Bad {} for {name} attribute", type_name::<T>())
	};
	Some(value)
}

fn generate_fields(fields: &syn::Fields) -> impl quote::ToTokens {
	let mut field_values = quote! {};
	for field in fields.iter() {
		if let Some(name) = &field.ident {
			field_values.extend(quote! {#name: });
		}
		let ty = &field.ty;
		field_values.extend(quote! { <#ty as GenRandom>::gen_random(rng) });
		
		if let Some(scale) = get_attribute_literal(&field.attrs, "scale") {
			field_values.extend(quote! { * #scale });
		}
		if let Some(bias) = get_attribute_literal(&field.attrs, "bias") {
			field_values.extend(quote! { + #bias });
		}
		
		field_values.extend(quote! { , });
	}
	
	// surround the field values with either () or {} brackets
	match fields {
		syn::Fields::Named(_) => {
			Some(proc_macro2::Group::new(
				proc_macro2::Delimiter::Brace,
				field_values
			))
		},
		syn::Fields::Unnamed(_) => {
			Some(proc_macro2::Group::new(
				proc_macro2::Delimiter::Parenthesis,
				field_values
			))
		},
		syn::Fields::Unit => None,
	}
}

fn impl_gen_random(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let mut function_body;
	
	match &ast.data {
		syn::Data::Enum(enumeration) => {
			let variants = &enumeration.variants;
			let epsilon: f64 = 1e-9;
			let one_minus_epsilon = 1.0 - epsilon;
			function_body = quote! {
				let mut variant: f64 = rng.gen_range(0.0..#one_minus_epsilon);
			};
			
			let mut test_variant = one_minus_epsilon;
			
			// parse enum fields
			for variant in variants.iter() {
				let probability: Option<f64> = parse_attribute_value(&variant.attrs, "prob");
				let Some(probability) = probability else {
					panic!("Variant {} has no probability", variant.ident)
				};
				
				let name = &variant.ident;
				let field_values = generate_fields(&variant.fields);
				
				function_body.extend(quote! {
					variant -= #probability;
					if variant <= 0.0 { return Self::#name #field_values; }
				});
				test_variant -= probability;
				
				
			}
			
			if test_variant >= 0.0 || test_variant < -2.0 * epsilon {
				panic!("Probabilities for enum do not add up to 1 (final test_variant = {test_variant}).");
			}
			
			function_body.extend(quote! {
				panic!("RNG returned value outside of range.")
			});
			
		},
		syn::Data::Struct(structure) => {
			let field_values = generate_fields(&structure.fields);
			function_body = quote! {
				Self #field_values
			};
		},
		syn::Data::Union(_) => panic!("derive(GenRandom) cannot be applied to unions."),
	};
	
	let gen = quote! {
		impl GenRandom for #name {
			fn gen_random(rng: &mut impl rand::Rng) -> Self {
				#function_body
			}
		}
	};
	
	//println!("{gen}");
	gen.into()
}

