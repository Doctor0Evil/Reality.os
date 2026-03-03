import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { NeuroChannel, ChannelType, ChannelStatus } from '../types/aln/neuro_channel';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface BiophysicalChannelRegistryProps {
  citizenDid?: string;
  allowCalibration?: boolean;
  allowSharing?: boolean;
}

export const BiophysicalChannelRegistry: React.FC<BiophysicalChannelRegistryProps> = ({
  citizenDid,
  allowCalibration = true,
  allowSharing = true,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [channels, setChannels] = useState<NeuroChannel[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`channel-registry-${Date.now()}`)
  );

  const loadChannels = async () => {
    try {
      setLoading(true);
      const targetDid = citizenDid || did;
      if (!targetDid) throw new Error('No DID available');

      const channelRegistry = await sovereigntyCore.queryNeuroChannels(targetDid);
      setChannels(channelRegistry);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load channels');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadChannels();
  }, [citizenDid, did, sovereigntyCore]);

  const handleCalibrate = async (channel: NeuroChannel) => {
    try {
      // Record calibration action
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did!,
        action: {
          id: 'portal.channel.calibrate',
          title: 'NeuroChannel Calibration',
          layer: 'BIOPHYSICAL',
          alnCapability: 'aln.tx.neuro_channel_calibration',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'BiophysicalChannelRegistry.tsx',
          lineStart: 65,
          colStart: 0,
          lineEnd: 90,
          colEnd: 0,
          authorDid: did!,
          symbolId: 'handleCalibrate',
        },
        payloadSummary: `Calibrating channel ${channel.channel_id}`,
      });

      // Start calibration wizard
      await sovereigntyCore.startNeuroChannelCalibration(channel.channel_id);
      await loadChannels(); // Refresh after calibration
    } catch (err) {
      console.error('Calibration failed:', err);
    }
  };

  const handleToggleSharing = async (channel: NeuroChannel) => {
    try {
      // Record sharing toggle action
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did!,
        action: {
          id: 'portal.channel.toggle_sharing',
          title: 'Toggle Channel Sharing',
          layer: 'BIOPHYSICAL',
          alnCapability: 'aln.tx.channel_sharing_control',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'BiophysicalChannelRegistry.tsx',
          lineStart: 95,
          colStart: 0,
          lineEnd: 120,
          colEnd: 0,
          authorDid: did!,
          symbolId: 'handleToggleSharing',
        },
        payloadSummary: `Toggled sharing for channel ${channel.channel_id}`,
      });

      // Toggle sharing consent
      await sovereigntyCore.toggleChannelSharing(channel.channel_id, !channel.is_sharing_enabled);
      await loadChannels(); // Refresh after toggle
    } catch (err) {
      console.error('Toggle sharing failed:', err);
    }
  };

  if (loading) {
    return (
      <div className="biophysical-channel-registry loading">
        <div className="spinner" />
        <p>Loading biophysical channels...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="biophysical-channel-registry error">
        <p className="error-message">{error}</p>
        <button onClick={loadChannels}>Retry</button>
      </div>
    );
  }

  return (
    <div className="biophysical-channel-registry">
      <h3>Biophysical Channel Registry</h3>
      <p className="subtitle">
        NeuroChannel registry: EEG, EMG, IMU, heart-rate, implants, environmental sensors
      </p>

      {channels.length === 0 ? (
        <div className="no-channels">
          <p>No biophysical channels registered.</p>
          <button onClick={() => sovereigntyCore.registerNewChannel()}>
            Register New Channel
          </button>
        </div>
      ) : (
        <div className="channel-grid">
          {channels.map((channel) => (
            <div key={channel.channel_id} className="channel-card">
              <div className="channel-header">
                <h4>{channel.channel_name}</h4>
                <span className={`channel-status ${channel.status}`}>
                  {channel.status}
                </span>
              </div>

              <div className="channel-details">
                <div className="detail-row">
                  <span className="detail-label">Channel ID:</span>
                  <span className="detail-value">{channel.channel_id}</span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Type:</span>
                  <span className="detail-value">{channel.channel_type}</span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Device DID:</span>
                  <span className="detail-value">{channel.device_did}</span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Units:</span>
                  <span className="detail-value">{channel.units}</span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Range:</span>
                  <span className="detail-value">
                    {channel.min_value} to {channel.max_value}
                  </span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Sampling Rate:</span>
                  <span className="detail-value">{channel.sampling_rate_hz} Hz</span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Last Calibration:</span>
                  <span className="detail-value">
                    {channel.last_calibration
                      ? new Date(channel.last_calibration).toLocaleString()
                      : 'Never'}
                  </span>
                </div>
                <div className="detail-row">
                  <span className="detail-label">Calibration Valid:</span>
                  <span className={`detail-value ${channel.is_calibration_valid ? 'valid' : 'invalid'}`}>
                    {channel.is_calibration_valid ? '✓ Valid' : '✗ Expired'}
                  </span>
                </div>
              </div>

              <div className="channel-quality">
                <div className="quality-header">
                  <span>Signal Quality:</span>
                  <span className={`quality-score ${channel.signal_quality >= 0.8 ? 'good' : channel.signal_quality >= 0.5 ? 'fair' : 'poor'}`}>
                    {channel.signal_quality >= 0.8 ? 'Good' : channel.signal_quality >= 0.5 ? 'Fair' : 'Poor'}
                  </span>
                </div>
                <div className="quality-bar">
                  <div
                    className="quality-fill"
                    style={{
                      width: `${channel.signal_quality * 100}%`,
                      backgroundColor:
                        channel.signal_quality >= 0.8
                          ? '#27ae60'
                          : channel.signal_quality >= 0.5
                          ? '#f39c12'
                          : '#e74c3c',
                    }}
                  />
                </div>
              </div>

              <div className="channel-actions">
                {allowCalibration && (
                  <button
                    onClick={() => handleCalibrate(channel)}
                    disabled={!channel.is_active}
                    className="btn-calibrate"
                  >
                    Calibrate
                  </button>
                )}
                {allowSharing && (
                  <button
                    onClick={() => handleToggleSharing(channel)}
                    className={`btn-sharing ${channel.is_sharing_enabled ? 'enabled' : 'disabled'}`}
                  >
                    Sharing: {channel.is_sharing_enabled ? 'ON' : 'OFF'}
                  </button>
                )}
                <button className="btn-details">View Details</button>
              </div>

              {channel.is_sharing_enabled && channel.sharing_recipients && (
                <div className="sharing-recipients">
                  <h5>Sharing With:</h5>
                  <ul>
                    {channel.sharing_recipients.map((recipient) => (
                      <li key={recipient}>{recipient}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          ))}
        </div>
      )}

      <div className="channel-info">
        <p className="info-text">
          <strong>Note:</strong> All channel calibrations and sharing consents are
          ROW-anchored and immutable. Changes create new entries, never modify existing ones.
        </p>
      </div>
    </div>
  );
};
