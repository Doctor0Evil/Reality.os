import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface StealthStatus {
  enabled: boolean;
  suppressed_channels: string[];
  timestamp: number;
  transaction_hash: string;
}

export const PrivacyStealthMode: React.FC = () => {
  const { did } = useDID();
  const [stealth, setStealth] = useState<StealthStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`stealth-mode-${Date.now()}`)
  );

  useEffect(() => {
    const loadStatus = async () => {
      if (!did) return;
      try {
        // Query sync_status table for stealth mode flag
        const result = await window.cozodb?.execute(`
          ?[value] := *config{key: 'stealth_mode', group_key: 'privacy', value: value}
          :order value.timestamp desc
          :limit 1
        `);
        if (result?.results?.length > 0) {
          setStealth(JSON.parse(result.results[0][0]));
        }
      } catch (err) {
        console.error('Failed to load stealth status:', err);
      }
    };
    loadStatus();
  }, [did]);

  const toggleStealth = async () => {
    if (!did || !stealth) return;
    setLoading(true);
    try {
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did,
        action: {
          id: 'portal.privacy.stealth_toggle',
          title: `Toggle Stealth Mode ${!stealth.enabled ? 'ON' : 'OFF'}`,
          layer: 'UX',
          alnCapability: 'aln.tx.sync_status_update',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'PrivacyStealthMode.tsx',
          lineStart: 50,
          colStart: 0,
          lineEnd: 80,
          colEnd: 0,
          authorDid: did,
          symbolId: 'toggleStealth',
        },
        payloadSummary: `Stealth mode ${!stealth.enabled ? 'enabled' : 'disabled'}`,
      });

      const newStealth: StealthStatus = {
        enabled: !stealth.enabled,
        suppressed_channels: !stealth.enabled ? ['location', 'biophysical_raw', 'social'] : [],
        timestamp: Date.now(),
        transaction_hash: '',
      };

      // Update config
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['stealth_mode', 'privacy', ${JSON.stringify(newStealth)}]
        ]
        :insert config
      `);

      // Update sync_status to disable non-critical syncing
      await window.cozodb?.execute(`
        ?[owner, id, entry_type, disabled, ts] <- [
          ['${did}', 'stealth_sync', 3, ${newStealth.enabled}, ${Date.now()}]
        ]
        :insert sync_status
      `);

      setStealth(newStealth);
    } catch (err) {
      console.error('Toggle failed:', err);
    } finally {
      setLoading(false);
    }
  };

  if (!stealth) return <div>Loading...</div>;

  return (
    <div className={`privacy-stealth ${stealth.enabled ? 'active' : ''}`}>
      <h3>Privacy / Stealth Mode</h3>
      <button
        onClick={toggleStealth}
        disabled={loading}
        className={`stealth-toggle ${stealth.enabled ? 'on' : 'off'}`}
      >
        {stealth.enabled ? 'STEALTH ACTIVE' : 'ENABLE STEALTH'}
      </button>
      <div className="stealth-info">
        <p className="info-text">
          <strong>Status:</strong> {stealth.enabled ? 'Non-critical broadcasting suppressed' : 'Normal operation'}
        </p>
        {stealth.enabled && (
          <p className="warning-text">
            ⚠ Safety-critical channels remain active.
          </p>
        )}
      </div>
    </div>
  );
};
