import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { EcoImpactScore, HostBudget } from '../types/aln/eco_metrics_host_budget';
import { EcoImpactBadge } from './EcoImpactBadge';

interface EcoImpactDashboardProps {
  citizenDid?: string;
  showHistoricalData?: boolean;
}

interface EcoMetrics {
  ecoImpactScore: EcoImpactScore;
  joulesConsumed: number;
  gco2PerJoule: number;
  autonomyPercentage: number;
  renewableEnergyPercentage: number;
  hardwareToxicityScore: number;
  coolingEfficiency: number;
}

interface MissionContribution {
  missionId: string;
  missionName: string;
  ecoImpactDelta: number;
  joulesContributed: number;
  timestamp: string;
}

export const EcoImpactDashboard: React.FC<EcoImpactDashboardProps> = ({
  citizenDid,
  showHistoricalData = true,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [metrics, setMetrics] = useState<EcoMetrics | null>(null);
  const [budget, setBudget] = useState<HostBudget | null>(null);
  const [missionContributions, setMissionContributions] = useState<MissionContribution[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadEcoData = async () => {
      try {
        setLoading(true);
        const targetDid = citizenDid || did;
        if (!targetDid) throw new Error('No DID available');

        // Load eco-metrics
        const ecoMetrics = await sovereigntyCore.queryEcoMetrics(targetDid);
        setMetrics(ecoMetrics);

        // Load host budget
        const hostBudget = await sovereigntyCore.queryHostBudget(targetDid);
        setBudget(hostBudget);

        // Load mission contributions (ROW-anchored)
        if (showHistoricalData) {
          const contributions = await sovereigntyCore.queryMissionContributions(targetDid);
          setMissionContributions(contributions);
        }

        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load eco-impact data');
      } finally {
        setLoading(false);
      }
    };

    loadEcoData();
  }, [citizenDid, did, showHistoricalData, sovereigntyCore]);

  if (loading) {
    return (
      <div className="eco-impact-dashboard loading">
        <div className="spinner" />
        <p>Loading eco-impact data...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="eco-impact-dashboard error">
        <p className="error-message">{error}</p>
        <button onClick={() => window.location.reload()}>Retry</button>
      </div>
    );
  }

  return (
    <div className="eco-impact-dashboard">
      <h3>Eco-Impact Dashboard</h3>

      {metrics && (
        <>
          <div className="eco-score-card">
            <div className="eco-score-header">
              <h4>Your Eco-Impact Score</h4>
              <EcoImpactBadge score={metrics.ecoImpactScore.value} size="large" />
            </div>
            <div className="eco-score-details">
              <div className="detail-row">
                <span className="detail-label">gCO₂/J:</span>
                <span className="detail-value">{metrics.gco2PerJoule.toFixed(4)}</span>
              </div>
              <div className="detail-row">
                <span className="detail-label">Renewable Energy:</span>
                <span className="detail-value">{metrics.renewableEnergyPercentage.toFixed(1)}%</span>
              </div>
              <div className="detail-row">
                <span className="detail-label">Hardware Toxicity:</span>
                <span className="detail-value">{metrics.hardwareToxicityScore.toFixed(2)}</span>
              </div>
              <div className="detail-row">
                <span className="detail-label">Cooling Efficiency:</span>
                <span className="detail-value">{metrics.coolingEfficiency.toFixed(2)}</span>
              </div>
            </div>
          </div>

          <div className="eco-metrics-grid">
            <div className="metric-card">
              <div className="metric-value">{metrics.joulesConsumed.toLocaleString()}</div>
              <div className="metric-label">Joules Consumed</div>
              <div className="metric-unit">J</div>
            </div>
            <div className="metric-card">
              <div className="metric-value">{metrics.autonomyPercentage.toFixed(1)}%</div>
              <div className="metric-label">Autonomy</div>
              <div className="metric-unit">Self-powered</div>
            </div>
            <div className="metric-card">
              <div className="metric-value">{metrics.renewableEnergyPercentage.toFixed(1)}%</div>
              <div className="metric-label">Renewable</div>
              <div className="metric-unit">Energy Mix</div>
            </div>
          </div>
        </>
      )}

      {budget && (
        <div className="budget-card">
          <h4>Energy Budget & Autonomy</h4>
          <div className="budget-progress">
            <div className="budget-header">
              <span>Daily Energy Budget</span>
              <span>
                {budget.current_energy_usage_mj.toLocaleString()} /{' '}
                {budget.daily_energy_budget_mj.toLocaleString()} mJ
              </span>
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{
                  width: `${budget.energy_usage_percentage()}%`,
                  backgroundColor:
                    budget.energy_usage_percentage() > 90
                      ? '#e74c3c'
                      : budget.energy_usage_percentage() > 70
                      ? '#f39c12'
                      : '#27ae60',
                }}
              />
            </div>
            <div className="budget-percentage">
              {budget.energy_usage_percentage().toFixed(1)}% used
              {budget.is_energy_budget_exceeded() && (
                <span className="budget-warning"> ⚠ Budget Exceeded</span>
              )}
            </div>
          </div>
          <div className="budget-details">
            <div className="budget-detail-row">
              <span>Compute Budget:</span>
              <span>
                {budget.current_compute_usage.toLocaleString()} /{' '}
                {budget.daily_compute_budget.toLocaleString()} gas
              </span>
            </div>
            <div className="budget-detail-row">
              <span>Bandwidth Budget:</span>
              <span>
                {(budget.current_bandwidth_usage_bytes / 1024 / 1024).toFixed(2)} /{' '}
                {(budget.daily_bandwidth_budget_bytes / 1024 / 1024).toFixed(2)} MB
              </span>
            </div>
            <div className="budget-detail-row">
              <span>Reset Time:</span>
              <span>{budget.budget_reset_time.toLocaleString()}</span>
            </div>
          </div>
        </div>
      )}

      {showHistoricalData && missionContributions.length > 0 && (
        <div className="mission-contributions">
          <h4>Restoration Timeline (ROW-Anchored Contributions)</h4>
          <div className="contributions-list">
            {missionContributions.map((contribution) => (
              <div key={contribution.missionId} className="contribution-card">
                <div className="contribution-header">
                  <span className="mission-name">{contribution.missionName}</span>
                  <span className="contribution-date">
                    {new Date(contribution.timestamp).toLocaleDateString()}
                  </span>
                </div>
                <div className="contribution-metrics">
                  <div className="contribution-metric">
                    <span className="metric-label">Eco-Impact Δ:</span>
                    <span className={`metric-value ${contribution.ecoImpactDelta >= 0 ? 'positive' : 'negative'}`}>
                      {contribution.ecoImpactDelta >= 0 ? '+' : ''}
                      {contribution.ecoImpactDelta.toFixed(3)}
                    </span>
                  </div>
                  <div className="contribution-metric">
                    <span className="metric-label">Joules Contributed:</span>
                    <span className="metric-value">{contribution.joulesContributed.toLocaleString()} J</span>
                  </div>
                </div>
                <div className="contribution-proof">
                  <span className="proof-label">ROW-Anchored:</span>
                  <span className="proof-hash">{contribution.missionId}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="eco-info">
        <p className="info-text">
          <strong>Eco-Impact Score:</strong> Calculated from renewable energy percentage,
          hardware toxicity, cooling efficiency, and gCO₂/J. Higher scores indicate
          more sustainable operation. All metrics are ROW-anchored and verifiable.
        </p>
      </div>
    </div>
  );
};
