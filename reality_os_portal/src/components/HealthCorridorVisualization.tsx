import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';

interface HealthMetric {
  name: string;
  current: number;
  min_safe: number;
  max_safe: number;
  unit: string;
  status: 'safe' | 'warning' | 'critical';
}

interface HealthCorridorVisualizationProps {
  refreshIntervalMs?: number;
}

export const HealthCorridorVisualization: React.FC<HealthCorridorVisualizationProps> = ({
  refreshIntervalMs = 2000,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [metrics, setMetrics] = useState<HealthMetric[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadMetrics = async () => {
      try {
        setLoading(true);
        const data = await sovereigntyCore.queryHealthCorridors(did!);
        setMetrics(data);
      } catch (err) {
        console.error('Failed to load health corridors:', err);
      } finally {
        setLoading(false);
      }
    };

    if (did) {
      loadMetrics();
      const interval = setInterval(loadMetrics, refreshIntervalMs);
      return () => clearInterval(interval);
    }
  }, [did, sovereigntyCore, refreshIntervalMs]);

  const getBarColor = (status: string) => {
    switch (status) {
      case 'critical': return '#e74c3c';
      case 'warning': return '#f39c12';
      default: return '#27ae60';
    }
  };

  if (loading) {
    return <div className="health-corridor loading">Loading health corridors...</div>;
  }

  return (
    <div className="health-corridor">
      <h3>My Health Corridors</h3>
      <p className="subtitle">
        Safe operating ranges enforced by neurorights-based ceilings
      </p>

      <div className="corridor-list">
        {metrics.map((metric) => {
          const range = metric.max_safe - metric.min_safe;
          const position = ((metric.current - metric.min_safe) / range) * 100;
          const clampedPosition = Math.max(0, Math.min(100, position));

          return (
            <div key={metric.name} className="corridor-item">
              <div className="corridor-header">
                <span className="metric-name">{metric.name}</span>
                <span className={`metric-status ${metric.status}`}>
                  {metric.status.toUpperCase()}
                </span>
              </div>
              
              <div className="corridor-bar-container">
                <div className="corridor-safe-zone">
                  <div
                    className="corridor-current-marker"
                    style={{
                      left: `${clampedPosition}%`,
                      backgroundColor: getBarColor(metric.status),
                    }}
                  />
                </div>
                <div className="corridor-labels">
                  <span>{metric.min_safe} {metric.unit}</span>
                  <span>{metric.max_safe} {metric.unit}</span>
                </div>
              </div>

              <div className="corridor-current-value">
                Current: <strong>{metric.current.toFixed(2)} {metric.unit}</strong>
              </div>
            </div>
          );
        })}
      </div>

      <div className="corridor-info">
        <p className="info-text">
          <strong>Neurorights Enforcement:</strong> If metrics exceed safe corridors, 
          SovereigntyCore will throttle or halt associated devices automatically.
        </p>
      </div>
    </div>
  );
};
