import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

export type RealityOSMode = 'daily' | 'clinical' | 'research' | 'field';

interface ModeConfig {
  mode: RealityOSMode;
  safety_profile: 'standard' | 'enhanced' | 'maximum';
  data_logging_level: 'minimal' | 'standard' | 'verbose';
  overlay_density: 'low' | 'balanced' | 'high';
  timestamp: number;
  transaction_hash: string;
}

interface RealityOSModeSwitcherProps {
  onModeChange?: (mode: RealityOSMode) => void;
}

export const RealityOSModeSwitcher: React.FC<RealityOSModeSwitcherProps> = ({
  onModeChange,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [currentMode, setCurrentMode] = useState<RealityOSMode>('daily');
  const [loading, setLoading] = useState(false);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`mode-switcher-${Date.now()}`)
  );

  useEffect(() => {
    const loadMode = async () => {
      if (!did) return;
      try {
        // Query CozoDB config table for current mode
        // :config{key: 'reality_os_mode', group_key: 'ux_preferences', value: json}
        const result = await window.cozodb?.execute(`
          ?[mode, safety, logging, overlay, ts, tx] := 
            *config{key: 'reality_os_mode', group_key: 'ux_preferences', value: json}
            json.mode = mode
            json.safety_profile = safety
            json.data_logging_level = logging
            json.overlay_density = overlay
            json.timestamp = ts
            json.transaction_hash = tx
          :order ts desc
          :limit 1
        `);
        if (result?.results?.length > 0) {
          const row = result.results[0];
          setCurrentMode(row[0]);
        }
      } catch (err) {
        console.error('Failed to load mode:', err);
      }
    };
    loadMode();
  }, [did]);

  const handleSwitchMode = async (newMode: RealityOSMode) => {
    if (!did || loading) return;
    setLoading(true);

    try {
      // Record action in Cyberspectre
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did,
        action: {
          id: 'portal.ux.mode_switch',
          title: `Switch Mode to ${newMode}`,
          layer: 'UX',
          alnCapability: 'aln.tx.config_update',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'RealityOSModeSwitcher.tsx',
          lineStart: 60,
          colStart: 0,
          lineEnd: 100,
          colEnd: 0,
          authorDid: did,
          symbolId: 'handleSwitchMode',
        },
        payloadSummary: `Mode switch from ${currentMode} to ${newMode}`,
      });

      // Evaluate Safety Envelope based on new mode
      const safetyProfile = newMode === 'clinical' ? 'maximum' : newMode === 'research' ? 'enhanced' : 'standard';
      
      const envelopeCheck = await sovereigntyCore.evaluateCorridor({
        method: 'UX_MODE_CHANGE',
        biophysicalData: { mode: newMode, safety_profile: safetyProfile },
      });

      if (!envelopeCheck.allowed) {
        throw new Error(`Mode change denied by RoH envelope: ${envelopeCheck.reason}`);
      }

      // Create new config entry (forward-only, append-only)
      const newConfig: ModeConfig = {
        mode: newMode,
        safety_profile: safetyProfile,
        data_logging_level: newMode === 'research' ? 'verbose' : 'standard',
        overlay_density: newMode === 'daily' ? 'balanced' : 'low',
        timestamp: Date.now(),
        transaction_hash: '', // Will be filled by anchor
      };

      // Insert into CozoDB config table
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['reality_os_mode', 'ux_preferences', ${JSON.stringify(newConfig)}]
        ]
        :insert config
      `);

      // Anchor to ALN ROW ledger
      const anchorResult = await sovereigntyCore.anchorToROW({
        sessionId: introspectionEngine.exportRecord().sessionId,
        envelope: newConfig,
      });

      // Update config with transaction hash (forward-only update creates new entry)
      const finalConfig = { ...newConfig, transaction_hash: anchorResult.transactionHash };
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['reality_os_mode', 'ux_preferences', ${JSON.stringify(finalConfig)}]
        ]
        :insert config
      `);

      setCurrentMode(newMode);
      onModeChange?.(newMode);
    } catch (err) {
      console.error('Mode switch failed:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="mode-switcher">
      <h3>Reality.OS Mode</h3>
      <div className="mode-grid">
        {(['daily', 'clinical', 'research', 'field'] as RealityOSMode[]).map((mode) => (
          <button
            key={mode}
            onClick={() => handleSwitchMode(mode)}
            disabled={loading || currentMode === mode}
            className={`mode-btn ${currentMode === mode ? 'active' : ''} ${mode}`}
          >
            <span className="mode-icon">
              {mode === 'daily' && '🏠'}
              {mode === 'clinical' && '🏥'}
              {mode === 'research' && '🔬'}
              {mode === 'field' && '🌲'}
            </span>
            <span className="mode-label">{mode.toUpperCase()}</span>
            {currentMode === mode && <span className="active-indicator">✓</span>}
          </button>
        ))}
      </div>
      <div className="mode-info">
        <p className="info-text">
          <strong>Safety Profile:</strong> {currentMode === 'clinical' ? 'Maximum (RoH Enforced)' : 'Standard'}
        </p>
      </div>
    </div>
  );
};
