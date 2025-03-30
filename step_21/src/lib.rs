extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

// Function like macros
#[proc_macro]
pub fn make_answer(_input: TokenStream) -> TokenStream {
    "fn answer() -> i32 { 114 * 1000 }".parse().unwrap()
}

#[proc_macro]
pub fn make_answer_2(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {
        fn answer_2() -> i32 {
            514
        }
    })
}

// Derive macros
#[proc_macro_derive(Log)]
pub fn log_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    
    let name = match ast {
        syn::DeriveInput { ident, .. } => ident,
    };

    let log_impl = quote! {
        impl Log for #name {
            fn info(&self, msg: &str) {
                use colored::Colorize;
                let time = ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let info = "[INFO]".green();
                let time = format!("[{}]", time).blue();
                let name = format!("[{}]", stringify!(#name)).yellow();
                let msg = format!("{}", msg).white();
                println!("{}{}{} {}", info, time, name, msg);
            }
            
            fn warn(&self, msg: &str) {
                use colored::Colorize;
                let time = ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let warn = "[WARN]".yellow();
                let time = format!("[{}]", time).blue();
                let name = format!("[{}]", stringify!(#name)).yellow();
                let msg = format!("{}", msg).white();
                println!("{}{}{} {}", warn, time, name, msg);
            }
            
            fn error(&self, msg: &str) {
                use colored::Colorize;
                let time = ::chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let error = "[ERROR]".red();
                let time = format!("[{}]", time).blue();
                let name = format!("[{}]", stringify!(#name)).yellow();
                let msg = format!("{}", msg).white();
                println!("{}{}{} {}", error, time, name, msg);
            }
        }
    };
    
    log_impl.into()
}
