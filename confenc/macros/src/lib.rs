use confenc_crydec::{encrypt, Encrypted};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::{parse_macro_input, LitStr, Token};

struct MacroInput {
    file_name: String,
    key: String,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file_name = input.parse::<LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let key = input.parse::<LitStr>()?.value();
        Ok(Self { file_name, key })
    }
}

/// Macro for compile-time encryption of config variables
/// and their later decryption at runtime.
#[proc_macro]
pub fn confenc(input: TokenStream) -> TokenStream {
    let MacroInput { file_name, key } = parse_macro_input!(input as MacroInput);

    let value: String = config::Config::builder()
        .add_source(config::File::with_name(&file_name))
        .build()
        .unwrap_or_else(|err| panic!("Failed to build config: {err}"))
        .get_string(&key)
        .unwrap_or_else(|err| panic!("Failed to get string value: {err}"));

    let Encrypted {
        data: encrypted,
        nonce,
        key,
    } = encrypt(value.as_bytes()).unwrap();
    generate_decode_scope(key, encrypted, nonce)
}

fn generate_decode_scope(key: [u8; 32], encrypted: Vec<u8>, nonce: [u8; 12]) -> TokenStream {
    quote! {
        {
            use confenc::decrypt;

            let key = [#(#key),*];
            let nonce = [#(#nonce),*];
            let encrypted = vec![#(#encrypted),*];
            #[cfg(test)]
            {
                dbg!(&key);
                dbg!(&nonce);
                dbg!(&encrypted);
            }
            decrypt(&key, &nonce, &encrypted)
                .unwrap_or_else(|e| panic!("Failed to decrypt: {}", e))
        }
    }
    .into()
}
