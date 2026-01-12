Reality.os is the host-level environment oracle for your Phoenix / Cyberswarm stack: it exposes typed, evidence-backed descriptors of what this biological host, toolchain, and lab profile are allowed to do before any router, upgrade store, or dev-tunnel acts.[8][11][12]

## Overview

Reality.os is a Rust crate that publishes **authoritative environment predicates** for a single augmented host (you) and its lab zone.[12][8]

- Describes allowed Rust toolchains, targets, and supply-chain policies.[12]
- Encodes bioscale budgets, evidence bundles, and neurorights envelopes that upgrades must respect.[8][12]
- Acts as the first gate for Cyberswarm routing, bioscale upgrades, and cyber_tunnel dev commands.[11][12]

## Core concepts

- **CargoEnvDescriptor**: structured description of the host’s Rust + bioscale + OTA environment (toolchain versions, target triples, allowed orgs/repos, host energy/protein budget, evidence bundle, corridor ceilings).[8][12]
- **HostBudget & EvidenceBundle**: imported from the bioscale-upgrade-store ABI to keep Reality.os numerically consistent with upgrade evaluation and Lyapunov/duty-cycle math.[12][8]
- **Neurorights envelopes**: constraints derived from DEFAULTBIOPHYSEVIDENCE and ReversalConditions so that every environment profile is anchored in real biophysics and rollback obligations.[8][12]

## Why it exists

- Prevent unsafe compilation targets (e.g. disallowed `repr` or target triples) for OTA firmware that touches CyberNano / BCI hardware.[11][12][8]
- Block dev-tunnel or CI actions against unapproved repos/branches even if higher layers misconfigure policies.[11][12]
- Provide a single, typed source of truth that Phoenix Neurostack and BioscaleUpgradeStore must consult before scheduling any evolution point.[11][12][8]

## Key API surface

Reality.os intentionally keeps its public API small and strongly typed.[12]

- `describe_cargo_env() -> CargoEnvDescriptor`  
  - Returns the current host’s environment descriptor (usually loaded from lab profile, TOML, or chain-based config).[12]

- `CargoEnvDescriptor::is_target_allowed(&self, target: &str) -> bool`  
  - Checks whether a given Rust target triple is permitted for this host.[12]

- `CargoEnvDescriptor::is_ota_repo_allowed(&self, org: &str, repo: &str, branch: &str) -> bool`  
  - Encodes OTA governance: which GitHub orgs, repos, and branches are valid for bioscale upgrades and dev-tunnel commands.[12]

These methods are designed to be called by:[11][12]

- Phoenix Neurostack router (`route_with_bioscale`) before selecting ingress nodes.  
- BioscaleUpgradeStore helpers (`evaluate_with_env`) before approving upgrades.  

## Integration pattern

Reality.os is not a router or upgrade store; it is the **environment contract** they must obey.[8][11][12]

- Phoenix’s `env_precheck` uses `describe_cargo_env()` plus `is_target_allowed` / `is_ota_repo_allowed` to hard-fail any evolution attempt outside the configured envelope.[11][12]
- The `evolve!` macro is wired so that every evolution call site must pass an env expression and expands into a call chain that invokes this precheck before `evaluate_upgrade`, `reserve_resources`, `trigger_ota`, and `route_with_bioscale`.[11][12]

This makes the Reality.os descriptor a neurorights-style compile-time obligation: no upgrade can be expressed in the DSL without explicitly acknowledging and passing through the host environment gate.[8][11][12]

[1](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/f1efc61c-7f62-4558-99e3-bb70865ef421/filename-cyberswarm-biosecure-CgXVZlhYQGu8vEQDY7UQng.md)
[2](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/b133ac41-231f-4bbc-9508-8c2c3acaca2d/filename-crates-bio-virtual-sc-yWNw8k5UQJi1pfkCiw62lA.md)
[3](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/54f457a1-def8-4097-af73-b64a651cb9eb/moving-beyond-the-traditional-OnEg29iuRE6XlTJ94_CelQ.md)
[4](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/415b7fd9-10a6-410e-9468-139f9ca10cc7/rust-learn-cybernetics-an-ai-l-J0lozmywQIuul3YvTkCF5w.md)
[5](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/ccadcb46-6b87-40de-8bf0-5487c9d56896/cybernet-as-described-is-a-non-n09vRTFHRNevGzZBhz_zXA.md)
[6](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/06de6d7a-c954-4083-85c0-d3164ffbf006/bioaug-is-a-neurorights-aligne-RgzmQ9bjTeClaL4oVSI4ng.md)
[7](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/38bb48a2-e8f6-4a3b-bd9c-168f8126d134/cyber-tunnel-ai-chat-dev-tunne-Oaa9iXbTQ4qvswfwxUKVJQ.md)
[8](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/968b8314-8a12-4228-b9c5-3c8064ef8983/below-is-math-structs-and-form-fA1IOTewRW2h.lalB3jjOg.md)
[9](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/fc548523-4588-4709-bfd6-f979c1d6e79e/uncovering-neural-rope-rollbac-tZuN_anrQKiVnC.kkQ.jlw.md)
[10](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/73fa1238-29d3-4dd6-8191-aa44645bfc0b/cybernetic-functions-telepathi-hIv4phm6RCCRc7cOi6BmIw.md)
[11](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/64726453-0b98-47e6-869b-32f542349016/filename-phoenix-lab-cyberswar-nzlGV7X8Sj.qJ_u9lRdAVA.md)
[12](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_cea84602-2afd-4788-b7f5-98537c17c5c6/de10705c-903d-4920-be88-354a071af41a/this-space-is-all-about-rust-p-fvC2kZk5RFeFS2f6WvKXow.md)
