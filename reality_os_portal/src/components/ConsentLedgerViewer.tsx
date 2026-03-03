import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { ChannelSharingConsent } from '../types/aln/neuro_channel';

interface ConsentLedgerViewerProps {
  citizenDid?: string;
}

export const ConsentLedgerViewer: React.FC<ConsentLedgerViewerProps> = ({
  citizenDid,
}) => {
  const { did } = useDID();
  const [consents, setConsents] = useState<ChannelSharingConsent[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadConsents = async () => {
      try {
        setLoading(true);
        const targetDid = citizenDid || did;
        if (!targetDid) return;

        // Query CozoDB 'link' table for consent relationships
        // Aligns with export.json.txt.txt schema: link(from, to, neuron, timestamp, transaction_hash)
        const result = await window.cozodb?.execute(`
          ?[from_cid, to_cid, channel_id, neuron, timestamp, transaction_hash, scope, expiry, revoked] := 
            *link{from: from_cid, to: to_cid, neuron: neuron, timestamp: timestamp, transaction_hash: tx_hash}
            *config{key: tx_hash, group_key: 'consent', value: json}
            json.scope = scope
            json.expiry_timestamp = expiry
            json.is_revoked = revoked
            json.channel_id = channel_id
            neuron == ${JSON.stringify(targetDid)}
          :order timestamp desc
          :limit 100
        `);

        const mappedConsents: ChannelSharingConsent[] = (result?.results || []).map((row: any) => ({
          from_cid: row[0],
          to_cid: row[1],
          channel_id: row[2],
          neuron: row[3],
          timestamp: row[4],
          transaction_hash: row[5],
          scope: row[6],
          expiry_timestamp: row[7],
          is_revoked: row[8],
        }));

        setConsents(mappedConsents);
      } catch (err) {
        console.error('Failed to load consent ledger:', err);
      } finally {
        setLoading(false);
      }
    };

    loadConsents();
  }, [citizenDid, did]);

  if (loading) {
    return <div className="consent-ledger loading">Loading consent ledger...</div>;
  }

  return (
    <div className="consent-ledger">
      <h3>Consent Ledger Viewer</h3>
      <p className="subtitle">
        Append-only log of all consent grants and revocations (ROW-anchored)
      </p>

      {consents.length === 0 ? (
        <div className="no-consents">
          <p>No consent records found.</p>
        </div>
      ) : (
        <div className="consent-table">
          <table>
            <thead>
              <tr>
                <th>Timestamp</th>
                <th>Channel</th>
                <th>Recipient</th>
                <th>Scope</th>
                <th>TX Hash</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              {consents.map((consent) => (
                <tr key={consent.transaction_hash} className={consent.is_revoked ? 'revoked' : ''}>
                  <td>{new Date(consent.timestamp).toLocaleString()}</td>
                  <td className="mono">{consent.channel_id.substring(0, 8)}...</td>
                  <td className="mono">{consent.to_cid.substring(0, 12)}...</td>
                  <td>
                    <div className="scope-tags">
                      {consent.scope.map((s) => (
                        <span key={s} className="scope-tag">{s}</span>
                      ))}
                    </div>
                  </td>
                  <td className="mono">
                    <a
                      href={`https://bostrom.cybernode.ai/tx/${consent.transaction_hash}`}
                      target="_blank"
                      rel="noopener noreferrer"
                    >
                      {consent.transaction_hash.substring(0, 10)}...
                    </a>
                  </td>
                  <td>
                    <span className={`status-badge ${consent.is_revoked ? 'revoked' : 'active'}`}>
                      {consent.is_revoked ? 'Revoked' : 'Active'}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <div className="ledger-info">
        <p className="info-text">
          <strong>Immutability:</strong> Consent records cannot be edited. 
          Revocations create new entries linking to the original grant.
        </p>
      </div>
    </div>
  );
};
