use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Field, Fields, Type, FieldsNamed};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    dbg!(&input);

    let name = input.ident;
    let builder_name = name.to_string();
    let builder_name = syn::Ident::new(format!("{builder_name}Builder").as_str(), name.span());

    let Data::Struct(DataStruct { fields, .. }) = input.data else {
        todo!()
    };

    let named_fields = if let Fields::Named(named_fields) = fields {
        named_fields
            .named
            .into_iter()
            .map(|x| {
                let name = x.ident;
                let ty = x.ty;
                (name.unwrap(), ty)
            })
            .collect::<Vec<_>>()
    } else {
        unimplemented!()
    };

    let builder_fields_ast = named_fields.iter().map(|(ident, ty)| {
        quote! {
            #ident: std::option::Option<#ty>
        }
    }).collect::<Vec<_>>();

    let builder_functions_ast = named_fields.iter().map(|(ident, ty)| {
        quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    }).collect::<Vec<_>>();

    let builder_defaults_ast = named_fields.iter().map(|(ident, ty)| {
        quote! {
            #ident: None
        }
    }).collect::<Vec<_>>();

    let builder_to_struct_field_ast = named_fields.iter().map(|(ident, ty)| {
        let err = format!("{} is not set", ident.to_string());
        quote! {
            #ident: self.#ident.clone().ok_or(#err)?
        }
    }).collect::<Vec<_>>();

    let generated_code = quote! {
        pub struct #builder_name {
            #(#builder_fields_ast,)*
        }        

        impl #builder_name {
            #(#builder_functions_ast)*

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(
                    #name {
                        #(#builder_to_struct_field_ast,)*
                    }
                )
            }
        }
        
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#builder_defaults_ast,)*
                }
            }
        }
    };

    generated_code.into()
    // TokenStream::new()
}
