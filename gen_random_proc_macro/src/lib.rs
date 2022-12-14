extern crate quote;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree as TokenTree2;
use std::str::FromStr;
use std::any::type_name;
use quote::quote;

/// See `gen_random::GenRandom`.
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
		field_values.extend(quote! { <#ty as GenRandom>::gen_random_max_depth(rng, _depth - 1) });
		
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

// very very precise summation algorithm
// see https://en.wikipedia.org/wiki/Kahan_summation_algorithm
fn kahan_sum(it: impl IntoIterator<Item = f64>) -> f64 {
	let mut it = it.into_iter();
	let mut sum = 0.0;
	let mut c = 0.0;
	while let Some(x) = it.next() {
		let y = x - c;
		let t = sum + y;
		c = (t - sum) - y;
		sum = t;
	}
	sum
}

fn impl_gen_random(ast: &syn::DeriveInput) -> TokenStream {
	let name = &ast.ident;
	let mut function_body;
	
	match &ast.data {
		syn::Data::Enum(enumeration) => {
			let variants = &enumeration.variants;
			
			let prob_sum = kahan_sum(variants.iter().map(|variant| {
				 match parse_attribute_value(&variant.attrs, "prob") {
				 	Some(prob) => if prob >= 0.0 {
				 		prob
				 	} else {
				 		panic!("Variant {} has negative probability", variant.ident)
				 	},
				 	None => panic!("Variant {} has no probability", variant.ident)
				 }
			}));
			
			if prob_sum <= f64::EPSILON {
				panic!("Sum of probabilties is (basically) zero.");
			}
			
			// ideally we would just do
			//     let mut variant: f64 = rng.gen_range(0.0..prob_sum);
			//     variant -= variant1_probability;
			//     if variant < 0.0 { bla bla bla }
			//     variant -= variant2_probability;
			//     if variant < 0.0 { bla bla bla }
			//     etc.
			// but because of floating point imprecision, it's possible
			// that all if conditions are false.
			// however we know that for each subtraction at most one ULP is lost.
			// so we'll be fine as long as we put the end of the range at
			//     prob_sum * (1.0 - (variant_count + 2) * ULP)
			// the + 2 is for the imprecision lost in kahan_sum and one more just to be sure.
			
			let variant_max = prob_sum * (1.0 - f64::EPSILON * (variants.len() + 2) as f64);
			function_body = quote! {
				let mut variant: f64 = rng.gen_range(0.0..=#variant_max);
			};
			
			// this test value ensures that the gen_random function never panicks.
			let mut test_variant = variant_max;
			
			// parse enum fields
			for variant in variants.iter() {
				// Note: None case was checked above when computing prob_sum
				let probability: f64 = parse_attribute_value(&variant.attrs, "prob").unwrap();
				
				let name = &variant.ident;
				let field_values = generate_fields(&variant.fields);
				
				function_body.extend(quote! {
					variant -= #probability;
					// note: if _depth <= 0, we will always return the first variant.
					if _depth <= 0 || variant < 0.0 { return Self::#name #field_values; }
				});
				test_variant -= probability;
				
				
			}
			
			if test_variant >= 0.0 {
				panic!("i did floating-point math wrong. this should never happen. (test_variant = {test_variant})");
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
			fn gen_random_max_depth(rng: &mut impl rand::Rng, _depth: isize) -> Self {
				#function_body
			}
		}
	};
	
	//println!("{gen}");
	gen.into()
}

