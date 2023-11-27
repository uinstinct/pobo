use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, FieldsNamed, Type};

mod helpers {
    use proc_macro2::{Ident, TokenStream as TokenStream2};
    use quote::{format_ident, quote, ToTokens};
    use syn::{GenericArgument, PathArguments, Type, TypePath};

    pub fn get_struct_inner_field_type(struct_field_type_path: &TypePath) -> Type {
        let struct_field_inner_path = struct_field_type_path
            .path
            .segments
            .iter()
            .next()
            .unwrap()
            .arguments
            .clone();

        let struct_field_generic_argument = match struct_field_inner_path.clone() {
            PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap().clone(),
            _ => panic!("struct field does not have generic argument"),
        };

        match struct_field_generic_argument {
            GenericArgument::Type(ty) => ty,
            _ => {
                panic!("cannot get the type of generic argument of struct field")
            }
        }
    }

    pub fn is_clonable(field_type: &Type) -> bool {
        !format!("{}", field_type.to_token_stream()).contains("JoinHandle")
    }

    pub fn mutex_getter(
        impl_stream: &mut TokenStream2,
        field: &Option<Ident>,
        mutex_field_type: &Type,
    ) {
        let field_name = field.clone().unwrap();
        let getter_function_name = format_ident!("get_{}", field_name);

        impl_stream.extend::<TokenStream2>(quote! {
            pub async fn #getter_function_name(&self) -> #mutex_field_type {
                let value = self.#field_name.lock().await;
                (*value).clone()
            }
        });
    }

    pub fn mutex_setter(
        impl_stream: &mut TokenStream2,
        field: &Option<Ident>,
        mutex_field_type: &Type,
    ) {
        let field_name = field.clone().unwrap();
        let setter_function_name = format_ident!("set_{}", field_name);
        impl_stream.extend::<TokenStream2>(quote! {
            pub async fn #setter_function_name(&self, value: #mutex_field_type) {
                let mut mutexed_field = self.#field_name.lock().await;
                *mutexed_field = value;
            }
        })
    }
}

#[proc_macro_derive(MutexGetSet)]
pub fn mutex_get_set(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut impl_stream = TokenStream2::default();

    if let syn::Data::Struct(s) = data {
        if let syn::Fields::Named(FieldsNamed { named, .. }) = s.fields {
            let fields = named.iter().map(|f| &f.ident);
            let field_types = named.iter().map(|f| &f.ty);

            for (field, ftype) in fields.into_iter().zip(field_types.into_iter()) {
                match ftype {
                    Type::Path(struct_field_type_path) => {
                        if struct_field_type_path.clone().path.segments[0]
                            .ident
                            .to_string()
                            == "Mutex"
                        // IMPROVE: instead of directly checking for "Mutex", it should check for type equality i.e. tokio::sync::Mutex should also be matched
                        {
                            let mutex_field_type =
                                helpers::get_struct_inner_field_type(struct_field_type_path);

                            if helpers::is_clonable(&mutex_field_type) {
                                helpers::mutex_getter(&mut impl_stream, field, &mutex_field_type);
                            }

                            helpers::mutex_setter(&mut impl_stream, field, &mutex_field_type);
                        }
                    }
                    _ => {}
                };
            }
        }
    };

    let output = quote! {
        impl #ident {
            #impl_stream
        }
    };

    output.into()
}
