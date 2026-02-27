fn main() {
    // load CargoEnvDescriptor from generated JSON (from ALN posture)
    let env = biospectre_core::load_env_descriptor("target/cargo_env_descriptor.json");
    let guard = biospectre_crypto_guard::BlakePolicyGuard::new(&env);
    guard.enforce("Cargo.lock");
}
