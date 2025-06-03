use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, GenericArgument, Ident, PathArguments, parse_macro_input, spanned::Spanned,
};

#[proc_macro_derive(K8sResource)]
pub fn k8s_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let setters = if let syn::Data::Struct(data) = &input.data {
        data.fields.iter().map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            // Method name will be like set_name(..)
            let setter_name = Ident::new(
                &format!("set_{}", field_name.clone().unwrap()).replace("r#", ""),
                field_name.span(),
            );

            if let syn::Type::Path(type_path) = field_type {
                // Is this an option?
                let option_path = type_path.path.segments.iter().find(|s| s.ident == "Option");
                if option_path.is_some() {
                    // We should be able to get generics from it if so
                    if let PathArguments::AngleBracketed(generic_t) =
                        &option_path.unwrap().arguments
                    {
                        if let GenericArgument::Type(syn::Type::Path(type_path)) =
                            &generic_t.args[0]
                        {
                            quote! {
                                pub fn #setter_name(&mut self, value: #type_path) {
                                    self.#field_name = Some(value);
                                }
                            }
                        } else {
                            panic!("No generics for Option<T>");
                        }
                    } else {
                        panic!("No generics for Option<T>");
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
            #(#setters)*

            pub fn new() -> Self {
                Self {
                    ..std::default::Default::default()
                }
            }
        }
    };

    TokenStream::from(expanded)
}
