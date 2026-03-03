import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';

export type LoadLevel = 'low' | 'balanced' | 'high';

interface LoadConfig {
  level: LoadLevel;
  throttle_overlays: boolean;
  throttle_notifications: boolean;
  timestamp: number;
}

export const CognitiveLoadDial: React.FC = () => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [loadLevel, setLoadLevel] = useState<LoadLevel>('balanced');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    const loadConfig = async () => {
      if (!did) return;
      try {
        const result = await window.cozodb?.execute(`
          ?[value] := *config{key: 'cognitive_load', group_key: 'ux_preferences', value: value}
          :order value.timestamp desc
          :limit 1
        `);
        if (result?.results?.length > 0) {
          setLoadLevel(JSON.parse(result.results[0][0]).level);
        }
      } catch (err) {
        console.error('Failed to load load config:', err);
      }
    };
    loadConfig();
  }, [did]);

  const setLoad = async (level: LoadLevel) => {
    if (!did || loading) return;
    setLoading(true);
    try {
      // SovereigntyCore throttles data flow based on this setting
      await sovereigntyCore.requestCognitiveLoadChange({
        citizenDid: did,
        level,
        throttle_overlays: level === 'low',
        throttle_notifications: level === 'low' || level === 'balanced',
      });

      const config: LoadConfig = {
        level,
        throttle_overlays: level === 'low',
        throttle_notifications: level !== 'high',
        timestamp: Date.now(),
      };

      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['cognitive_load', 'ux_preferences', ${JSON.stringify(config)}]
        ]
        :insert config
      `);

      setLoadLevel(level);
    } catch (err) {
      console.error('Load change failed:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="cognitive-load-dial">
      <h3>Cognitive Load Dial</h3>
      <div className="dial-container">
        {(['low', 'balanced', 'high'] as LoadLevel[]).map((level) => (
          <button
            key={level}
            onClick={() => setLoad(level)}
            disabled={loading || loadLevel === level}
            className={`dial-btn ${loadLevel === level ? 'active' : ''} ${level}`}
          >
            {level.toUpperCase()}
            {loadLevel === level && <span className="active-dot">●</span>}
          </button>
        ))}
      </div>
      <div className="load-info">
        <p className="info-text">
          <strong>Throttling:</strong> {loadLevel === 'low' ? 'Overlays & Notifications Disabled' : 'Normal Flow'}
        </p>
      </div>
    </div>
  );
};
