use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_component(&ast)
}

fn impl_component(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            fn as_any(&mut self) -> &mut dyn std::any::Any { self }
        }
    };
    gen.into()
}
