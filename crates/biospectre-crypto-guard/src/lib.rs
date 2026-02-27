use std::{collections::HashSet, path::Path};

#[derive(Debug)]
pub struct BlakePolicyViolation {
    pub crate_name: String,
    pub symbol: String,
    pub reason: String,
}

#[derive(Debug)]
pub struct BlakePolicyGuard<'a> {
    pub env: &'a crate::CargoEnvDescriptor,
    forbidden_symbols: HashSet<String>,
}

impl<'a> BlakePolicyGuard<'a> {
    pub fn new(env: &'a crate::CargoEnvDescriptor) -> Self {
        let mut forbidden = HashSet::new();
        forbidden.insert("blake3".into());
        forbidden.insert("Blake3".into());
        forbidden.insert("BLAKE3".into());
        forbidden.insert("argon2".into());
        forbidden.insert("argon2id".into());
        forbidden.insert("argon2i".into());
        forbidden.insert("argon2d".into());
        Self { env, forbidden_symbols: forbidden }
    }

    pub fn scan_lockfile<P: AsRef<Path>>(
        &self,
        cargo_lock_path: P,
    ) -> Vec<BlakePolicyViolation> {
        let mut violations = Vec::new();
        let text = std::fs::read_to_string(cargo_lock_path).unwrap_or_default();

        for line in text.lines() {
            if line.starts_with("name =") {
                let crate_name = line
                    .split('"')
                    .nth(1)
                    .unwrap_or_default()
                    .to_lowercase();
                for sym in &self.forbidden_symbols {
                    if crate_name.contains(&sym.to_lowercase()) {
                        violations.push(BlakePolicyViolation {
                            crate_name: crate_name.clone(),
                            symbol: sym.clone(),
                            reason: "forbidden cryptographic primitive in biophysical corridor"
                                .into(),
                        });
                    }
                }
            }
        }
        violations
    }

    pub fn enforce(&self, cargo_lock_path: &str) {
        if !self.env.is_in_scope() {
            return;
        }
        let violations = self.scan_lockfile(cargo_lock_path);
        if !violations.is_empty() {
            let msgs: Vec<String> = violations
                .into_iter()
                .map(|v| format!("crate `{}` uses `{}`: {}", v.crate_name, v.symbol, v.reason))
                .collect();
            panic!(
                "BlakePolicyGuard: posture violation for corridor `{}`:\n{}",
                self.env.posture.corridor_id,
                msgs.join("\n"),
            );
        }
    }
}
