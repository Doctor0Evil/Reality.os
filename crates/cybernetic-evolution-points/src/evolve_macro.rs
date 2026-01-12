use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, Ident, LitStr, Token};
use syn::parse::{Parse, ParseStream};

use crate::EvolutionPoint;

/// Parsed arguments for evolve! macro.
struct EvolveArgs {
    env_expr: Expr,
    host_ident: Ident,
    store_ident: Ident,
    router_ident: Ident,
    point_ident: Ident,
    target_lit: LitStr,
    ota_org: LitStr,
    ota_repo: LitStr,
    ota_branch: LitStr,
}

impl Parse for EvolveArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // env = <expr>,
        input.parse::<Ident>()?; // env
        input.parse::<Token![=]>()?;
        let env_expr: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        // host = <ident>: HostBudget,
        let _host_kw: Ident = input.parse()?; // host
        input.parse::<Token![=]>()?;
        let host_ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let _: syn::Type = input.parse()?; // discard explicit type
        input.parse::<Token![,]>()?;

        // store = <ident>,
        let _store_kw: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let store_ident: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        // router = <ident>,
        let _router_kw: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let router_ident: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        // point = <Ident>,
        let _point_kw: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let point_ident: Ident = input.parse()?;
        input.parse::<Token![,]>()?;

        // target = "triple",
        let _target_kw: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let target_lit: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        // ota = { org = "...", repo = "...", branch = "..." }
        let _ota_kw: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let content;
        syn::braced!(content in input);
        // org = "..."
        let _org_kw: Ident = content.parse()?;
        content.parse::<Token![=]>()?;
        let ota_org: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;
        // repo = "..."
        let _repo_kw: Ident = content.parse()?;
        content.parse::<Token![=]>()?;
        let ota_repo: LitStr = content.parse()?;
        content.parse::<Token![,]>()?;
        // branch = "..."
        let _branch_kw: Ident = content.parse()?;
        content.parse::<Token![=]>()?;
        let ota_branch: LitStr = content.parse()?;

        Ok(EvolveArgs {
            env_expr,
            host_ident,
            store_ident,
            router_ident,
            point_ident,
            target_lit,
            ota_org,
            ota_repo,
            ota_branch,
        })
    }
}

#[proc_macro]
pub fn evolve(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as EvolveArgs);

    let env_expr    = args.env_expr;
    let host_ident  = args.host_ident;
    let store_ident = args.store_ident;
    let router_ident = args.router_ident;
    let point_ident = args.point_ident;
    let target_lit  = args.target_lit;
    let ota_org     = args.ota_org;
    let ota_repo    = args.ota_repo;
    let ota_branch  = args.ota_branch;

    // Enforce env gate in the generated code.
    let expanded = quote! {
        {
            use reality_os::cargo_env::CargoEnvDescriptor;
            use reality_os::cargo_env::describe_cargo_env;
            use bioscale_upgrade_store::{BioscaleUpgradeStore, HostBudget, UpgradeDecision};
            use phoenix_lab_cyberswarm::env_gate::env_precheck;

            // Evaluate environment descriptor *from the provided expression*.
            let __cargo_env: CargoEnvDescriptor = #env_expr;

            // Enforce env gate before any evolution logic.
            env_precheck(
                &#store_ident,
                &#router_ident,
                &#point_ident::descriptor(),
                #target_lit,
                #ota_org,
                #ota_repo,
                #ota_branch,
            ).expect("evolve!: env_precheck failed");

            // Now proceed with normal evolution chain.
            let __upgrade_desc = #point_ident::descriptor();
            let __decision = #store_ident.evaluate_upgrade(
                #host_ident.clone(),
                __upgrade_desc.clone(),
                std::time::SystemTime::now(),
            );

            match __decision {
                UpgradeDecision::Approved { scheduled_at, expected_completion } => {
                    let mut __host_mut = #host_ident.clone();
                    #store_ident
                        .reserve_resources(&mut __host_mut, __upgrade_desc.clone())
                        .expect("evolve!: reserve_resources failed");

                    #store_ident
                        .trigger_ota(__upgrade_desc.clone())
                        .expect("evolve!: trigger_ota failed");

                    #router_ident
                        .route_with_bioscale(&__host_mut, &__upgrade_desc, &__cargo_env);

                    (scheduled_at, expected_completion, __host_mut)
                }
                UpgradeDecision::Denied { reason } => {
                    panic!("evolve!: upgrade denied by bioscale store: {}", reason);
                }
            }
        }
    };

    TokenStream::from(expanded)
}
