import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface OverlaySettings {
  haptics_enabled: boolean;
  visual_cues_enabled: boolean;
  audio_prompts_enabled: boolean;
  intensity_level: number; // 0-100
  timestamp: number;
  transaction_hash: string;
}

export const SensoryOverlayManager: React.FC = () => {
  const { did } = useDID();
  const [settings, setSettings] = useState<OverlaySettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`sensory-overlay-${Date.now()}`)
  );

  useEffect(() => {
    const loadSettings = async () => {
      if (!did) return;
      try {
        const result = await window.cozodb?.execute(`
          ?[value] := *config{key: 'sensory_overlay', group_key: 'ux_preferences', value: value}
          :order value.timestamp desc
          :limit 1
        `);
        if (result?.results?.length > 0) {
          setSettings(JSON.parse(result.results[0][0]));
        }
      } catch (err) {
        console.error('Failed to load overlay settings:', err);
      } finally {
        setLoading(false);
      }
    };
    loadSettings();
  }, [did]);

  const updateSetting = async (key: keyof OverlaySettings, value: any) => {
    if (!did || !settings) return;
    try {
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did,
        action: {
          id: 'portal.ux.overlay_update',
          title: `Update Overlay ${key}`,
          layer: 'UX',
          alnCapability: 'aln.tx.config_update',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'SensoryOverlayManager.tsx',
          lineStart: 50,
          colStart: 0,
          lineEnd: 80,
          colEnd: 0,
          authorDid: did,
          symbolId: 'updateSetting',
        },
        payloadSummary: `Updated ${key} to ${value}`,
      });

      const newSettings = { ...settings, [key]: value, timestamp: Date.now(), transaction_hash: '' };
      
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['sensory_overlay', 'ux_preferences', ${JSON.stringify(newSettings)}]
        ]
        :insert config
      `);

      setSettings(newSettings);
    } catch (err) {
      console.error('Update failed:', err);
    }
  };

  if (loading) return <div className="sensory-overlay loading">Loading...</div>;
  if (!settings) return <div className="sensory-overlay error">No settings found</div>;

  return (
    <div className="sensory-overlay">
      <h3>Sensory Overlay Manager</h3>
      <div className="overlay-controls">
        <div className="control-row">
          <label>Haptics</label>
          <input
            type="checkbox"
            checked={settings.haptics_enabled}
            onChange={(e) => updateSetting('haptics_enabled', e.target.checked)}
          />
        </div>
        <div className="control-row">
          <label>Visual Cues</label>
          <input
            type="checkbox"
            checked={settings.visual_cues_enabled}
            onChange={(e) => updateSetting('visual_cues_enabled', e.target.checked)}
          />
        </div>
        <div className="control-row">
          <label>Audio Prompts</label>
          <input
            type="checkbox"
            checked={settings.audio_prompts_enabled}
            onChange={(e) => updateSetting('audio_prompts_enabled', e.target.checked)}
          />
        </div>
        <div className="control-row">
          <label>Intensity ({settings.intensity_level}%)</label>
          <input
            type="range"
            min="0"
            max="100"
            value={settings.intensity_level}
            onChange={(e) => updateSetting('intensity_level', parseInt(e.target.value))}
          />
        </div>
      </div>
      <div className="overlay-info">
        <p className="info-text">
          <strong>Neurorights Check:</strong> Intensity levels exceeding 80% require RoH envelope validation.
        </p>
      </div>
    </div>
  );
};
