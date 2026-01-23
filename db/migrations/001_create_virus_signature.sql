CREATE TABLE IF NOT EXISTS virus_signature (
    sig_id TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    threat_class TEXT NOT NULL,
    forbidden_fields TEXT NOT NULL,   -- JSON array of strings. [file:5]
    forbidden_modules TEXT NOT NULL,  -- JSON array of strings. [file:5]
    delta_risk REAL NOT NULL,
    delta_duty_cycle REAL NOT NULL,
    delta_fatigue REAL NOT NULL,
    delta_eco REAL NOT NULL,
    jurisdiction_particle TEXT NOT NULL
);
