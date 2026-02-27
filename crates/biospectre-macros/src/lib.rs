use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn bioscaleupgrade(input: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(input as syn::Expr); // simplified

    let env = biospectre_core::load_env_descriptor_at_compile_time();
    let guard = biospectre_crypto_guard::BlakePolicyGuard::new(&env);
    guard.enforce("Cargo.lock");

    let expanded = quote! {
        {
            // normal evolve! logic injected here
            biospectre_upgrade_runtime::perform_upgrade()
        }
    };
    expanded.into()
}

#[proc_macro]
pub fn aln_enforce_corridor(input: TokenStream) -> TokenStream {
    let _args = parse_macro_input!(input as syn::Expr);

    let env = biospectre_core::load_env_descriptor_at_compile_time();
    if !env.is_in_scope() {
        return quote!({}).into();
    }
    let guard = biospectre_crypto_guard::BlakePolicyGuard::new(&env);
    guard.enforce("Cargo.lock");

    quote!({}).into()
}
