use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input, spanned::Spanned};

#[proc_macro_derive(K8sResource)]
pub fn k8s_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let getters = if let syn::Data::Struct(data) = &input.data {
        data.fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            let setter_name = Ident::new(
                &format!("set_{}", field_name.clone().unwrap()).replace("r#", ""),
                field_name.span(),
            );

            if let syn::Type::Path(type_path) = &field.ty {
                if type_path.path.segments[0].ident == "Option" {
                    quote! {
                        pub fn #setter_name(&mut self, value: #field_type) {
                            self.#field_name = Some(value);
                        }
                    }
                } else {
                    quote! {
                        pub fn #setter_name(&mut self, value: #field_type) {
                            self.#field_name = value;
                        }
                    }
                }
            } else {
                quote! {}
            }
        })
    } else {
        return TokenStream::new();
    };

    let expanded = quote! {
        impl #name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}
