import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { HostBudget } from '../types/aln/eco_metrics_host_budget';

interface SafetyEnvelope {
  max_intensity_uv: number;
  max_frequency_hz: number;
  max_duty_cycle_percent: number;
  max_cognitive_load_score: number;
  current_intensity_uv: number;
  current_frequency_hz: number;
  current_duty_cycle_percent: number;
  current_cognitive_load_score: number;
}

interface BCISafetyEnvelopeInspectorProps {
  channel_id?: string;
}

export const BCISafetyEnvelopeInspector: React.FC<BCISafetyEnvelopeInspectorProps> = ({
  channel_id,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [envelope, setEnvelope] = useState<SafetyEnvelope | null>(null);
  const [budget, setBudget] = useState<HostBudget | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadSafetyData = async () => {
      try {
        setLoading(true);
        const [envData, budgetData] = await Promise.all([
          sovereigntyCore.querySafetyEnvelope(channel_id),
          sovereigntyCore.queryHostBudget(did!),
        ]);
        setEnvelope(envData);
        setBudget(budgetData);
      } catch (err) {
        console.error('Failed to load safety envelope:', err);
      } finally {
        setLoading(false);
      }
    };

    if (did) {
      loadSafetyData();
      const interval = setInterval(loadSafetyData, 5000); // Refresh every 5s
      return () => clearInterval(interval);
    }
  }, [did, channel_id, sovereigntyCore]);

  if (loading) {
    return <div className="safety-inspector loading">Loading safety envelope...</div>;
  }

  if (!envelope) {
    return <div className="safety-inspector error">No safety envelope available</div>;
  }

  const calculatePercentage = (current: number, max: number) => {
    return Math.min((current / max) * 100, 100);
  };

  const getStatusColor = (percentage: number) => {
    if (percentage >= 90) return '#e74c3c'; // Red
    if (percentage >= 70) return '#f39c12'; // Orange
    return '#27ae60'; // Green
  };

  return (
    <div className="safety-inspector">
      <h3>BCI Safety Envelope Inspector</h3>
      <p className="subtitle">
        Real-time monitoring of neuro-rights corridors and RoH ceilings
      </p>

      <div className="envelope-grid">
        <div className="envelope-card">
          <h4>Intensity (μV)</h4>
          <div className="gauge">
            <div
              className="gauge-fill"
              style={{
                width: `${calculatePercentage(envelope.current_intensity_uv, envelope.max_intensity_uv)}%`,
                backgroundColor: getStatusColor(calculatePercentage(envelope.current_intensity_uv, envelope.max_intensity_uv)),
              }}
            />
          </div>
          <div className="gauge-values">
            <span>{envelope.current_intensity_uv.toFixed(2)}</span>
            <span>/{envelope.max_intensity_uv.toFixed(2)}</span>
          </div>
        </div>

        <div className="envelope-card">
          <h4>Frequency (Hz)</h4>
          <div className="gauge">
            <div
              className="gauge-fill"
              style={{
                width: `${calculatePercentage(envelope.current_frequency_hz, envelope.max_frequency_hz)}%`,
                backgroundColor: getStatusColor(calculatePercentage(envelope.current_frequency_hz, envelope.max_frequency_hz)),
              }}
            />
          </div>
          <div className="gauge-values">
            <span>{envelope.current_frequency_hz.toFixed(1)}</span>
            <span>/{envelope.max_frequency_hz.toFixed(1)}</span>
          </div>
        </div>

        <div className="envelope-card">
          <h4>Duty Cycle (%)</h4>
          <div className="gauge">
            <div
              className="gauge-fill"
              style={{
                width: `${calculatePercentage(envelope.current_duty_cycle_percent, envelope.max_duty_cycle_percent)}%`,
                backgroundColor: getStatusColor(calculatePercentage(envelope.current_duty_cycle_percent, envelope.max_duty_cycle_percent)),
              }}
            />
          </div>
          <div className="gauge-values">
            <span>{envelope.current_duty_cycle_percent.toFixed(1)}%</span>
            <span>/{envelope.max_duty_cycle_percent.toFixed(1)}%</span>
          </div>
        </div>

        <div className="envelope-card">
          <h4>Cognitive Load</h4>
          <div className="gauge">
            <div
              className="gauge-fill"
              style={{
                width: `${calculatePercentage(envelope.current_cognitive_load_score, envelope.max_cognitive_load_score)}%`,
                backgroundColor: getStatusColor(calculatePercentage(envelope.current_cognitive_load_score, envelope.max_cognitive_load_score)),
              }}
            />
          </div>
          <div className="gauge-values">
            <span>{envelope.current_cognitive_load_score.toFixed(2)}</span>
            <span>/{envelope.max_cognitive_load_score.toFixed(2)}</span>
          </div>
        </div>
      </div>

      {budget && (
        <div className="energy-context">
          <h4>Energy Budget Context</h4>
          <p>
            Current Energy Usage: {budget.current_energy_usage_mj.toLocaleString()} mJ / {budget.daily_energy_budget_mj.toLocaleString()} mJ
          </p>
          <p className={budget.is_energy_budget_exceeded() ? 'warning' : 'safe'}>
            Status: {budget.is_energy_budget_exceeded() ? '⚠ Budget Exceeded' : '✓ Within Budget'}
          </p>
        </div>
      )}

      <div className="envelope-info">
        <p className="info-text">
          <strong>Note:</strong> All parameters are enforced by RoH envelopes. 
          Exceeding ceilings triggers automatic throttling or shutdown.
        </p>
      </div>
    </div>
  );
};
