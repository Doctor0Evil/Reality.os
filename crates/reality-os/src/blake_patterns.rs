use regex::Regex;

/// Static, versioned Blake-family pattern set.
pub struct BlakePatternSet {
    pub version: &'static str,
    pub crate_regex: Regex,
    pub symbol_regex: Regex,
}

impl BlakePatternSet {
    pub fn v1() -> Self {
        // Matches blake, blake2, blake3 in any casing.
        let crate_regex = Regex::new(r"(?i)blake3?|blake2[bs]?").unwrap();
        // Symbols like blake3::hash, blake3::keyed_hash, blake3::derive_key.
        let symbol_regex =
            Regex::new(r"(?i)^(?:blake[23][bs]?)::(hash|keyed_hash|derive_key)$").unwrap();

        Self {
            version: "blake-patterns-v1",
            crate_regex,
            symbol_regex,
        }
    }

    pub fn matches_crate(&self, name: &str) -> bool {
        self.crate_regex.is_match(name)
    }

    pub fn matches_symbol(&self, symbol: &str) -> bool {
        self.symbol_regex.is_match(symbol)
    }
}
