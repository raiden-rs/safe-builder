use convert_case::*;
use quote::*;
use syn::*;

fn create_field_type(ident: &Ident) -> Ident {
    let name = ident.to_string().to_case(Case::Pascal);
    format_ident!("{}Type", name)
}

fn expand_default_type_variables(
    idents: &Vec<Ident>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    idents.clone().into_iter().map(|_ident| {
        quote! { (), }
    })
}

fn expand_masked_type_variables(
    idents: &Vec<Ident>,
    open_index: usize,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    idents
        .clone()
        .into_iter()
        .enumerate()
        .map(move |(index, ident)| {
            if open_index == index {
                quote! { (), }
            } else {
                let ident = create_field_type(&ident);
                quote! { #ident, }
            }
        })
}

fn expand_all_type_variables(
    idents: &Vec<Ident>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    idents
        .clone()
        .into_iter()
        .enumerate()
        .map(move |(_index, ident)| {
            let ident = create_field_type(&ident);
            quote! { #ident, }
        })
}

fn expand_return_types(
    idents: &Vec<Ident>,
    open_index: usize,
    ty: &Type,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    let ty = ty.clone();
    idents
        .clone()
        .into_iter()
        .enumerate()
        .map(move |(index, ident)| {
            if open_index == index {
                quote! { #ty, }
            } else {
                let ident = create_field_type(&ident);
                quote! { #ident, }
            }
        })
}

fn expand_masked_type_variables_decl(
    idents: &Vec<Ident>,
    open_index: usize,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    idents
        .clone()
        .into_iter()
        .enumerate()
        .map(move |(index, ident)| {
            if open_index == index {
                quote! {}
            } else {
                let ident = create_field_type(&ident);
                quote! { #ident, }
            }
        })
}

fn expand_filled_type_variables(
    idents: &Vec<Ident>,
    type_map: std::collections::HashMap<String, Type>,
) -> impl Iterator<Item = proc_macro2::TokenStream> {
    idents.clone().into_iter().map(move |ident| {
        let ty = type_map.get(&ident.to_string()).unwrap();
        quote! { #ty, }
    })
}

fn create_field_type_map(fields: &FieldsNamed) -> std::collections::HashMap<String, Type> {
    let mut field_type_map: std::collections::HashMap<String, Type> =
        std::collections::HashMap::new();
    for f in fields.named.iter() {
        let name = f.ident.as_ref().unwrap().to_string();
        field_type_map.insert(name, f.ty.clone());
    }
    field_type_map
}

pub(crate) fn expand(input: syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_name = input.ident;

    let builder_name = format_ident!("{}Builder", struct_name);
    let default_builder_type = format_ident!("Default{}Builder", struct_name);

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(n),
            ..
        }) => n,
        _ => panic!("`Builder` only supports `struct`."),
    };

    let default_builder_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        if is_option(&f.ty) {
            quote! { #name: None, }
        } else {
            quote! { #name: (), }
        }
    });

    let builder_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_option(&f.ty) {
            quote! { #name: #ty, }
        } else {
            let name = name.clone().unwrap();
            let ident = create_field_type(&name);
            quote! { #name: #ident, }
        }
    });

    let required_field_idents: Vec<Ident> = fields
        .named
        .iter()
        .filter(|f| !is_option(&f.ty))
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let optional_field_idents: Vec<Ident> = fields
        .named
        .iter()
        .filter(|f| is_option(&f.ty))
        .map(|f| f.ident.clone().unwrap())
        .collect();

        let default_type_variables = expand_default_type_variables(&required_field_idents);

    let field_type_map = create_field_type_map(&fields);
    let required_fns =
        required_field_idents
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, ident)| {
                let vars = expand_masked_type_variables(&required_field_idents, index);
                let decls = expand_masked_type_variables_decl(&required_field_idents, index);
                let rest_fields = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    if name.clone().unwrap().to_string() == ident.to_string() {
                        return quote! {};
                    };
                    quote! { #name: self.#name, }
                });
                let arg_type = field_type_map.get(&ident.to_string()).unwrap();
                let return_types = expand_return_types(&required_field_idents, index, arg_type);
                quote! {
                    impl<#(#decls)*> #builder_name<#(#vars)*> {
                        pub fn #ident(self, #ident: #arg_type) -> #builder_name<#(#return_types)*> {
                            #builder_name{
                                #ident,
                                #(#rest_fields)*
                            }
                        }
                    }
                }
            });

    let optional_fns = optional_field_idents.clone().into_iter().map(|ident| {
        let decls = expand_all_type_variables(&required_field_idents);
        let types = expand_all_type_variables(&required_field_idents);
        let return_types = expand_all_type_variables(&required_field_idents);
        let rest_fields = fields.named.iter().map(|f| {
            let name = &f.ident;
            if name.clone().unwrap().to_string() == ident.to_string() {
                return quote! {};
            };
            quote! { #name: self.#name, }
        });
        let arg_type = field_type_map.get(&ident.to_string()).unwrap();
        let arg_type = inner_type("Option", arg_type).to_owned().unwrap();

        quote! {
            impl<#(#decls)*> #builder_name<#(#types)*> {
                pub fn #ident(self, #ident: #arg_type) -> #builder_name<#(#return_types)*> {
                    #builder_name{
                        #ident: Some(#ident),
                        #(#rest_fields)*
                    }
                }
            }
        }
    });

    let build_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: self.#name, }
    });
    let builder_type_variables = expand_all_type_variables(&required_field_idents);

    let field_type_map = create_field_type_map(&fields);
    let builder_filled_type_variables =
        expand_filled_type_variables(&required_field_idents, field_type_map);

    let method_name = format_ident!(
        "{}",
        if let Some(name) = find_method_name(&input.attrs) {
            name
        } else {
            "builder".to_owned()
        }
    );

    let default_builder_type_alias = {
        let default_type_variables = expand_default_type_variables(&required_field_idents);
        quote! {
            pub type #default_builder_type = #builder_name<#(#default_type_variables)*>;
        }
    };

    quote! {
        pub struct #builder_name<#(#builder_type_variables)*> {
            #(#builder_fields)*
        }

        #default_builder_type_alias


        impl #struct_name {
            pub fn #method_name() -> #builder_name<#(#default_type_variables)*> {
                #builder_name {
                    #(#default_builder_fields)*
                }
            }
        }

        impl #builder_name<#(#builder_filled_type_variables)*> {
            pub fn build(self) -> #struct_name {
                #struct_name {
                    #(#build_fields)*
                }
            }
        }

        #(#required_fns)*
        #(#optional_fns)*
    }
}

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) => {
            if segments.iter().any(|s| s.ident == "Option") {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn inner_type<'a>(wrapper: &str, ty: &'a syn::Type) -> Option<&'a syn::Type> {
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != wrapper {
            return None;
        }

        if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
            if inner_ty.args.len() != 1 {
                return None;
            }

            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }
        }
    }
    None
}

fn find_eq_string_from(attr: &syn::Attribute, name: &str) -> Option<String> {
    let mut tokens = match attr.tokens.clone().into_iter().next() {
        Some(proc_macro2::TokenTree::Group(g)) => g.stream().into_iter(),
        _ => return None,
    };
    match tokens.next() {
        Some(proc_macro2::TokenTree::Ident(ref ident)) if *ident == name => {}
        _ => return None,
    };
    // #[builder(method_name = )]
    match tokens.next() {
        Some(proc_macro2::TokenTree::Punct(ref punct)) if punct.as_char() == '=' => {}
        _ => return None,
    };
    // #[builder(method_name = value)]
    let lit = match tokens.next() {
        Some(proc_macro2::TokenTree::Literal(lit)) => syn::Lit::new(lit),
        _ => return None,
    };
    match &lit {
        syn::Lit::Str(lit_str) => {
            let value = lit_str.value();
            if value.trim().is_empty() {
                panic!()
            };
            Some(value)
        }
        _ => None,
    }
}
fn find_method_name(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if let Some(lit) = find_eq_string_from(&attr, "method_name") {
            return Some(lit);
        }
    }
    None
}
