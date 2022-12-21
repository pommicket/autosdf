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
#[proc_macro_derive(GenRandom, attributes(prob, scale, bias, params, only_if))]
pub fn gen_random_derive(input: TokenStream) -> TokenStream {
	// Construct a representation of Rust code as a syntax tree
	// that we can manipulate
	let ast = syn::parse(input).unwrap();
	// Build the trait implementation
	impl_gen_random(&ast)
}

fn get_attribute(attrs: &[syn::Attribute], name: &str) -> Option<TokenStream2> {
	let attr = attrs.iter().find(|a| {
		let path = &a.path;
		if let Some(ident) = path.get_ident() {
			ident == name
		} else {
			false
		}
	})?;

	let tokens: TokenStream2 = attr.tokens.clone().into();
	let tokens: Vec<TokenTree2> = tokens.into_iter().collect();
	if tokens.len() != 1 {
		panic!("Expected {name}(<value>)");
	}
	use TokenTree2::Group;
	use proc_macro2::Delimiter;
	let value = match &tokens[0] {
		Group(g) if g.delimiter() == Delimiter::Parenthesis => {
			g.stream()
		},
		_ => {
			panic!("Expected {name}(<value>)");
		},
	};
	
	Some(value)
}

fn parse_attribute_value<T: FromStr>(attrs: &[syn::Attribute], name: &str) -> Option<T> {
	let stream = get_attribute(attrs, name)?;
	let Ok(value) = stream.to_string().parse() else {
		panic!("Bad {} for {name} attribute", type_name::<T>())
	};
	Some(value)
}

fn generate_fields(fields: &syn::Fields, params_type: &TokenStream2) -> impl quote::ToTokens {
	let mut field_values = quote! {};
	for field in fields.iter() {
		if let Some(name) = &field.ident {
			field_values.extend(quote! {#name: });
		}
		let ty = &field.ty;
		field_values.extend(quote! { <#ty as GenRandom<#params_type>>::gen_random_params(rng, <#params_type as GenRandomParams>::inc_depth(params)) });
		
		if let Some(scale) = get_attribute(&field.attrs, "scale") {
			field_values.extend(quote! { * ( #scale ) });
		}
		if let Some(bias) = get_attribute(&field.attrs, "bias") {
			field_values.extend(quote! { + ( #bias ) });
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
	let params_type = get_attribute(&ast.attrs, "params").unwrap_or(quote! { () });
	
	match &ast.data {
		syn::Data::Enum(enumeration) => {
			let variants = &enumeration.variants;
			function_body = quote! {
				let mut prob_sum = 0.0;
			};
			
			for variant in variants.iter() {
				 match parse_attribute_value::<f64>(&variant.attrs, "prob") {
				 	Some(prob) => if prob >= 0.0 {
				 		let only_if = get_attribute(&variant.attrs, "only_if")
				 			.unwrap_or(quote! { true });
				 		
				 		function_body.extend(quote! {
				 			if #only_if { prob_sum += #prob; }
				 		});
				 	} else {
				 		panic!("Variant {} has negative probability", variant.ident)
				 	},
				 	None => panic!("Variant {} has no probability", variant.ident)
				 }
			}
			
			let compensation = (variants.len() + 1) as f64 * f64::EPSILON;
			function_body.extend(quote! {
				let mut variant: f64 = rng.gen_range(0.0..prob_sum - #compensation);
			});
			
			// parse enum fields
			for variant in variants.iter() {
				// Note: None case was checked above when computing prob_sum
				let probability: f64 = parse_attribute_value(&variant.attrs, "prob").unwrap();
				let only_if = get_attribute(&variant.attrs, "only_if")
		 			.unwrap_or(quote! { true });
				
				let name = &variant.ident;
				let field_values = generate_fields(&variant.fields, &params_type);
				
				function_body.extend(quote! {
					if #only_if {
						variant -= #probability;
						if variant < 0.0 { return Self::#name #field_values; }
					}
				});
			}
			
			function_body.extend(quote! {
				panic!("RNG returned value outside of range (this should never happen).")
			});
			
		},
		syn::Data::Struct(structure) => {
			let field_values = generate_fields(&structure.fields, &params_type);
			function_body = quote! {
				Self #field_values
			};
		},
		syn::Data::Union(_) => panic!("derive(GenRandom) cannot be applied to unions."),
	};
	
	let gen = quote! {
		impl GenRandom<#params_type> for #name {
			fn gen_random_params(rng: &mut impl rand::Rng, params: #params_type) -> Self {
				#function_body
			}
		}
	};
	
	//println!("{gen}");
	gen.into()
}

