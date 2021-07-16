use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{Data, DataStruct, Fields, GenericArgument, parse_macro_input, Path, PathArguments, PathSegment, Type, TypePath, DeriveInput};

fn option_inner_type(path: &Path) -> Option<&Type> {
    if path.leading_colon.is_some() {
        return None;
    }

    if path.segments.len() != 1 {
        return None;
    }

    if path.segments[0].ident == "Vec" {
        println!("Oh a vector!");
        return None;
    }

    if path.segments[0].ident != "Option" {
        return None;
    }

    let ab = match &path.segments[0].arguments {
        PathArguments::AngleBracketed(ab) => ab,
        _ => return None,
    };

    if ab.args.len() != 1 {
        return None;
    }

    match &ab.args[0] {
        GenericArgument::Type(t) => Some(t),
        _ => None,
    }
}

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let builder_name = format_ident!("{}Builder", ident);

    let fields = match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("at the disco"),
    };

    for f in fields.into_iter() {
        let field_name = f.ident;
        let ty = f.ty;
        // let ty = match f.ty {
        //     Type::Path(TypePath { path, .. }) if path.segments.len() == 1 && path.segments[0].ident == "String" => "oh hey it's a string",
        //     Type::Path(ty @ TypePath { .. }) => match option_inner_type(&ty.path) {
        //         Some(Type::Path(TypePath { path, .. })) if path.is_ident("String") => "an option string ffs",
        //         Some(_) => "ummm",
        //         None => "WELP",
        //     },
        //     _ => "fuck if I know"
        // };

        println!("Field {:?}, {:?}", field_name, ty.into_token_stream());
    }

    let tokens = quote! {
        pub struct #builder_name {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        #[automatically_derived]
        impl #builder_name {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&self) -> Result<#ident, Box<dyn std::error::Error>> {
                Ok(#ident {
                    executable: self.executable.clone().ok_or("executable")?,
                    args: self.args.clone().ok_or("args")?,
                    env: self.env.clone().ok_or("env")?,
                    current_dir: None,
                })
            }
        }

        impl #ident {
            pub fn builder() -> #builder_name {
                #builder_name {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    tokens.into()
}
