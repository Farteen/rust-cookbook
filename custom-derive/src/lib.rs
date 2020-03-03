extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(HelloWorld, attributes(hello_world_name))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).expect("failed to parse the source into an AST");
    let gen = impl_hello_world(&ast);
    
    gen.parse()
    .expect("failed to parse the AST generated from deriving from HelloWorld");
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let identifier = &ast::iden;
    let hello_world_name = get_name_attribute(ast).unwrap_or_else(|| identifier.as_ref());
    quote! {
        impl HelloWorld for #identifier {
            fn hello_world() {
                println!("The struct or enum {} says: \"Hello world from {}!\"", stringify!(#identifier), #hello_world_name);
            }
        }
    }
}

fn get_name_attribute(ast: &syn::DeriveInput) -> Option<&str> {
    const ATTR_NAME: &str = "hello_world_name";

    if let Some(attr) = ast.attr.iter().find(|a| a.name() == ATTR_NAME) {
        if let syn::Meta::NameValue(_, ref value) = attr.value {
            if let syn::Lit::Str(ref value_as_str) = *value {
                Some(value_as_str.value_as_str)
            } else {
                panic!("Expected a string as the value of {}, found {:?} instead", ATTR_NAME, value);
            }
        } else {
            panic!("Expected an attribute in the form #[{} = \"Some value\"]", ATTR_NAME);
        }
    } else {
        None
    }
}