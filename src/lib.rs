use {
    proc_macro::TokenStream,
    quote::quote,
    syn::{parse_macro_input, ItemFn},
};

#[proc_macro_attribute]
pub fn only_role(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the required role from the macro's attribute
    let roles: Vec<String> = attr
        .to_string()
        .replace("\"", "") // Remove quotes
        .split(',') // Split roles by comma
        .map(|role| role.trim().to_string())
        .collect();

    // Parse the input handler function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function's signature and block
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input_fn;

    // Generate the expanded code
    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            use axum::{
                http::StatusCode,
                response::IntoResponse,
                Extension,
                Json,
            };
            use crate::enums::error::Error;

            let allowed_roles: Vec<String> = vec![#(#roles),*].iter().map(|s| s.to_string()).collect();

            // Check the user's role
            if !allowed_roles.contains(&claims.role) {
                return Err(Error::AccessDenied);
            }

            // Continue executing the original handler
            async move { #block }.await
        }
    };

    TokenStream::from(expanded)
}
