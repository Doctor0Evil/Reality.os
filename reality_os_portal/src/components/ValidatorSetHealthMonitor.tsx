import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { OrganicCpuValidator, ValidatorType } from '../types/aln/organic_cpu_validator';
import { EcoImpactBadge } from './EcoImpactBadge';
import { RoHBadge } from './RoHBadge';

interface ValidatorSetHealthMonitorProps {
  showOnlyHealthValidators?: boolean;
  refreshIntervalSeconds?: number;
}

interface ValidatorStats {
  total: number;
  active: number;
  healthy: number;
  healthValidated: number;
  avgUptime: number;
  avgEcoScore: number;
}

export const ValidatorSetHealthMonitor: React.FC<ValidatorSetHealthMonitorProps> = ({
  showOnlyHealthValidators = false,
  refreshIntervalSeconds = 30,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [validators, setValidators] = useState<OrganicCpuValidator[]>([]);
  const [stats, setStats] = useState<ValidatorStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadValidators = async () => {
    try {
      setLoading(true);
      const validatorSet = await sovereigntyCore.queryValidatorSet();
      
      let filteredValidators = validatorSet.validators;
      if (showOnlyHealthValidators) {
        filteredValidators = filteredValidators.filter(v =>
          v.meets_health_validation_requirements()
        );
      }
      
      setValidators(filteredValidators);
      
      // Calculate stats
      const activeValidators = validatorSet.validators.filter(v => v.is_active);
      const healthyValidators = activeValidators.filter(v => v.is_healthy());
      const healthValidated = activeValidators.filter(v =>
        v.meets_health_validation_requirements()
      );
      
      const avgUptime = activeValidators.length > 0
        ? activeValidators.reduce((sum, v) => sum + v.uptime_percentage, 0) / activeValidators.length
        : 0;
      
      const avgEcoScore = activeValidators.length > 0
        ? activeValidators.reduce((sum, v) => sum + v.eco_impact_score.value, 0) / activeValidators.length
        : 0;
      
      setStats({
        total: validatorSet.total_validators,
        active: validatorSet.active_validators,
        healthy: healthyValidators.length,
        healthValidated: healthValidated.length,
        avgUptime,
        avgEcoScore,
      });
      
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load validator set');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadValidators();
    const interval = setInterval(loadValidators, refreshIntervalSeconds * 1000);
    return () => clearInterval(interval);
  }, [showOnlyHealthValidators, refreshIntervalSeconds, sovereigntyCore]);

  if (loading) {
    return (
      <div className="validator-health-monitor loading">
        <div className="spinner" />
        <p>Loading validator set health...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="validator-health-monitor error">
        <p className="error-message">{error}</p>
        <button onClick={loadValidators}>Retry</button>
      </div>
    );
  }

  return (
    <div className="validator-health-monitor">
      <h3>Organic CPU Validator Set Health</h3>
      
      {stats && (
        <div className="validator-stats-grid">
          <div className="stat-card">
            <div className="stat-value">{stats.total}</div>
            <div className="stat-label">Total Validators</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.active}</div>
            <div className="stat-label">Active</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.healthy}</div>
            <div className="stat-label">Healthy</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.healthValidated}</div>
            <div className="stat-label">Health-Validated</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.avgUptime.toFixed(1)}%</div>
            <div className="stat-label">Avg Uptime</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.avgEcoScore.toFixed(2)}</div>
            <div className="stat-label">Avg Eco Score</div>
          </div>
        </div>
      )}

      <div className="validator-list">
        <h4>Active Validators</h4>
        {validators.length === 0 ? (
          <p className="no-validators">
            No validators match the current criteria.
          </p>
        ) : (
          <div className="validator-table">
            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Type</th>
                  <th>Jurisdiction</th>
                  <th>Eco Score</th>
                  <th>RoH</th>
                  <th>Uptime</th>
                  <th>Energy</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {validators.map((validator) => (
                  <tr key={validator.validator_did}>
                    <td>
                      <div className="validator-name">{validator.name}</div>
                      <div className="validator-did">{validator.validator_did}</div>
                    </td>
                    <td>{validator.validator_type}</td>
                    <td>{validator.jurisdiction}</td>
                    <td>
                      <EcoImpactBadge score={validator.eco_impact_score.value} />
                    </td>
                    <td>
                      <RoHBadge value={validator.risk_of_harm} />
                    </td>
                    <td>{validator.uptime_percentage.toFixed(1)}%</td>
                    <td>
                      <div className="energy-usage">
                        <div className="energy-bar">
                          <div
                            className="energy-fill"
                            style={{
                              width: `${validator.energy_usage_percentage()}%`,
                              backgroundColor:
                                validator.energy_usage_percentage() > 90
                                  ? '#e74c3c'
                                  : validator.energy_usage_percentage() > 70
                                  ? '#f39c12'
                                  : '#27ae60',
                            }}
                          />
                        </div>
                        <span className="energy-text">
                          {validator.energy_usage_percentage().toFixed(0)}%
                        </span>
                      </div>
                    </td>
                    <td>
                      <div className="status-indicators">
                        {validator.is_validating && (
                          <span className="status-badge validating">Validating</span>
                        )}
                        {validator.is_healthy() ? (
                          <span className="status-badge healthy">✓ Healthy</span>
                        ) : (
                          <span className="status-badge unhealthy">✗ Unhealthy</span>
                        )}
                        {validator.meets_health_validation_requirements() && (
                          <span className="status-badge health-validated">
                            Health-Validated
                          </span>
                        )}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      <div className="validator-info">
        <p className="info-text">
          <strong>Note:</strong> Organic CPU validators are selected based on eco-impact,
          uptime, K/E/R metrics, and RoH compliance. No monetary stakes involved.
        </p>
      </div>
    </div>
  );
};
