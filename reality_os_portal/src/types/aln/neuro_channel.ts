import { DidUri } from './did';

/**
 * NeuroChannel types aligned with ALN Rust structs (Batch 2, File 13).
 * These types are cached locally in CozoDB 'particle' and 'config' tables
 * and synced from the ALN NeuroChannel shard.
 */
export type ChannelType = 
  | 'EEG' 
  | 'EMG' 
  | 'ECG' 
  | 'IMU' 
  | 'Temperature' 
  | 'Galvanic' 
  | 'Environmental';

export type ChannelStatus = 
  | 'active' 
  | 'inactive' 
  | 'calibrating' 
  | 'error' 
  | 'revoked';

export interface NeuroChannel {
  /** Unique channel ID (UUID v7) */
  channel_id: string;
  /** Human-readable name (e.g., "Frontal EEG Lobe") */
  channel_name: string;
  /** Citizen DID who owns this channel */
  citizen_did: DidUri;
  /** Device DID (implant, wearable, gateway) */
  device_did: DidUri;
  /** Type of biophysical signal */
  channel_type: ChannelType;
  /** Current operational status */
  status: ChannelStatus;
  /** Units of measurement (e.g., "uV", "Hz", "bpm") */
  units: string;
  /** Minimum safe value */
  min_value: number;
  /** Maximum safe value */
  max_value: number;
  /** Sampling rate in Hz */
  sampling_rate_hz: number;
  /** Timestamp of last calibration (ISO 8601) */
  last_calibration: string | null;
  /** Is calibration currently valid? */
  is_calibration_valid: boolean;
  /** Calibration expiry height (ALN block height) */
  calibration_expiry_height: number | null;
  /** Signal quality score (0.0 - 1.0) */
  signal_quality: number;
  /** Is sharing enabled? */
  is_sharing_enabled: boolean;
  /** List of DIDs sharing this channel (if enabled) */
  sharing_recipients: DidUri[] | null;
  /** ROW anchor height when this channel config was last updated */
  row_anchor_height: number;
  /** Forward-only: true if this is the latest config */
  is_current: boolean;
  /** CozoDB particle CID storing raw config metadata */
  particle_cid: string | null;
}

/**
 * Calibration Event structure (stored in CozoDB 'transaction' table)
 */
export interface NeuroCalibrationEvent {
  transaction_hash: string;
  channel_id: string;
  citizen_did: DidUri;
  timestamp: number; // Unix ms
  block_height: number;
  success: boolean;
  quality_score_before: number;
  quality_score_after: number;
  memo: string; // e.g., "Auto-calibration via SovereigntyCore"
}

/**
 * Channel Sharing Consent (stored in CozoDB 'link' table)
 */
export interface ChannelSharingConsent {
  from_cid: string; // Citizen DID particle
  to_cid: string;   // Recipient DID particle
  channel_id: string;
  neuron: DidUri;   // Citizen DID
  timestamp: number;
  transaction_hash: string;
  scope: string[];  // e.g., ["read_raw", "read_aggregated"]
  expiry_timestamp: number | null;
  is_revoked: boolean;
}
