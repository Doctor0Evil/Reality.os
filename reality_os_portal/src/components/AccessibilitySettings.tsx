import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';

interface AccessibilityConfig {
  high_contrast: boolean;
  dyslexia_font: boolean;
  reduced_motion: boolean;
  screen_reader_optimized: boolean;
  timestamp: number;
}

export const AccessibilitySettings: React.FC = () => {
  const { did } = useDID();
  const [config, setConfig] = useState<AccessibilityConfig | null>(null);

  useEffect(() => {
    const loadConfig = async () => {
      if (!did) return;
      try {
        const result = await window.cozodb?.execute(`
          ?[value] := *config{key: 'accessibility', group_key: 'ux_preferences', value: value}
          :order value.timestamp desc
          :limit 1
        `);
        if (result?.results?.length > 0) {
          setConfig(JSON.parse(result.results[0][0]));
        }
      } catch (err) {
        console.error('Failed to load a11y config:', err);
      }
    };
    loadConfig();
  }, [did]);

  const updateConfig = async (key: keyof AccessibilityConfig, value: boolean) => {
    if (!did || !config) return;
    const newConfig = { ...config, [key]: value, timestamp: Date.now() };
    await window.cozodb?.execute(`
      ?[key, group, value] <- [
        ['accessibility', 'ux_preferences', ${JSON.stringify(newConfig)}]
      ]
      :insert config
    `);
    setConfig(newConfig);
    
    // Apply global CSS classes
    document.body.classList.toggle('high-contrast', newConfig.high_contrast);
    document.body.classList.toggle('dyslexia-font', newConfig.dyslexia_font);
    document.body.classList.toggle('reduced-motion', newConfig.reduced_motion);
  };

  if (!config) return <div>Loading...</div>;

  return (
    <div className="accessibility-settings">
      <h3>Accessibility (Neuro-Aware)</h3>
      <div className="a11y-grid">
        <label className="a11y-toggle">
          <input
            type="checkbox"
            checked={config.high_contrast}
            onChange={(e) => updateConfig('high_contrast', e.target.checked)}
          />
          High Contrast
        </label>
        <label className="a11y-toggle">
          <input
            type="checkbox"
            checked={config.dyslexia_font}
            onChange={(e) => updateConfig('dyslexia_font', e.target.checked)}
          />
          Dyslexia Font
        </label>
        <label className="a11y-toggle">
          <input
            type="checkbox"
            checked={config.reduced_motion}
            onChange={(e) => updateConfig('reduced_motion', e.target.checked)}
          />
          Reduced Motion
        </label>
        <label className="a11y-toggle">
          <input
            type="checkbox"
            checked={config.screen_reader_optimized}
            onChange={(e) => updateConfig('screen_reader_optimized', e.target.checked)}
          />
          Screen Reader
        </label>
      </div>
    </div>
  );
};
