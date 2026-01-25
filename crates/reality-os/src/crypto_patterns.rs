use regex::Regex;

#[derive(Debug, Clone)]
pub struct CryptoPatternSet {
    blake_re: Regex,
    sha3_re: Regex,
}

impl CryptoPatternSet {
    pub fn v1() -> Self {
        let blake_re = Regex::new(r"(?i)blake3?|blake2b?s?").expect("valid BLAKE regex");
        let sha3_re  = Regex::new(r"(?i)sha3[-_ ]?(224|256|384|512)?").expect("valid SHA3 regex");
        Self { blake_re, sha3_re }
    }

    pub fn matches_any<S: AsRef<str>>(&self, s: S) -> bool {
        let s = s.as_ref();
        self.blake_re.is_match(s) || self.sha3_re.is_match(s)
    }

    pub fn is_blake<S: AsRef<str>>(&self, s: S) -> bool {
        self.blake_re.is_match(s.as_ref())
    }

    pub fn is_sha3<S: AsRef<str>>(&self, s: S) -> bool {
        self.sha3_re.is_match(s.as_ref())
    }
}
