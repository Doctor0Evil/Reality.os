import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';

interface FocusWindow {
  start_hour: number;
  end_hour: number;
  days: number[]; // 0-6 (Sun-Sat)
  enabled: boolean;
  id: string;
}

export const FocusCorridorSetup: React.FC = () => {
  const { did } = useDID();
  const [windows, setWindows] = useState<FocusWindow[]>([]);

  useEffect(() => {
    const loadWindows = async () => {
      if (!did) return;
      try {
        const result = await window.cozodb?.execute(`
          ?[value] := *config{key: 'focus_corridors', group_key: 'productivity', value: value}
          :order value.timestamp desc
          :limit 1
        `);
        if (result?.results?.length > 0) {
          setWindows(JSON.parse(result.results[0][0]).windows || []);
        }
      } catch (err) {
        console.error('Failed to load focus windows:', err);
      }
    };
    loadWindows();
  }, [did]);

  const addWindow = async () => {
    if (!did) return;
    const newWindow: FocusWindow = {
      start_hour: 9,
      end_hour: 17,
      days: [1, 2, 3, 4, 5],
      enabled: true,
      id: crypto.randomUUID(),
    };
    const updated = [...windows, newWindow];
    await saveWindows(updated);
  };

  const saveWindows = async (updated: FocusWindow[]) => {
    if (!did) return;
    const config = { windows: updated, timestamp: Date.now() };
    await window.cozodb?.execute(`
      ?[key, group, value] <- [
        ['focus_corridors', 'productivity', ${JSON.stringify(config)}]
      ]
      :insert config
    `);
    setWindows(updated);
  };

  return (
    <div className="focus-corridor">
      <h3>Focus Corridor Setup</h3>
      <button onClick={addWindow} className="btn-add">Add Focus Window</button>
      <div className="windows-list">
        {windows.map((win) => (
          <div key={win.id} className="window-card">
            <div className="window-time">
              {win.start_hour}:00 - {win.end_hour}:00
            </div>
            <div className="window-days">
              {win.days.map(d => ['S','M','T','W','T','F','S'][d]).join(' ')}
            </div>
            <label>
              <input
                type="checkbox"
                checked={win.enabled}
                onChange={(e) => {
                  win.enabled = e.target.checked;
                  saveWindows([...windows]);
                }}
              />
              Enabled
            </label>
          </div>
        ))}
      </div>
      <div className="focus-info">
        <p className="info-text">
          <strong>Effect:</strong> Alerts and overlays are heavily filtered during active windows.
        </p>
      </div>
    </div>
  );
};
