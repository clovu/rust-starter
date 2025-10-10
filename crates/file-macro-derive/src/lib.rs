use proc_macro::TokenStream;

#[proc_macro_derive(FileOps)]
pub fn file_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen_tk = quote::quote! {
        impl FileOps for #name {
            fn list_file_path(path: &str) -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
                let files = std::fs::read_dir(path)
                    .map_err(|_| "error file reading dir...")?
                    .filter_map(|re| re.ok())
                    .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect();
                Ok(files)
            }
        }
    };
    gen_tk.into()
}
