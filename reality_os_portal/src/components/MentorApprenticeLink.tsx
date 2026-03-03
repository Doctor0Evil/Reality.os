import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface Relationship {
  partner_did: string;
  role: 'mentor' | 'apprentice';
  scope: string[];
  timestamp: number;
  transaction_hash: string;
}

export const MentorApprenticeLink: React.FC = () => {
  const { did } = useDID();
  const [relationships, setRelationships] = useState<Relationship[]>([]);
  const [partnerDid, setPartnerDid] = useState('');
  const [loading, setLoading] = useState(false);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`mentor-link-${Date.now()}`)
  );

  useEffect(() => {
    const loadRelationships = async () => {
      if (!did) return;
      try {
        // Query community table for relationships
        const result = await window.cozodb?.execute(`
          ?[neuron, particle, following, follower, ts] := 
            *community{owner_id: '${did}', neuron: neuron, particle: particle, following: following, follower: follower}
            *config{key: particle, group_key: 'relationships', value: json}
            json.timestamp = ts
        `);
        if (result?.results?.length > 0) {
          const mapped = result.results.map((row: any) => ({
            partner_did: row[0],
            role: row[3] ? 'mentor' : 'apprentice',
            scope: ['guidance', 'read_only'],
            timestamp: row[4],
            transaction_hash: '',
          }));
          setRelationships(mapped);
        }
      } catch (err) {
        console.error('Failed to load relationships:', err);
      }
    };
    loadRelationships();
  }, [did]);

  const createLink = async () => {
    if (!did || !partnerDid) return;
    setLoading(true);
    try {
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did,
        action: {
          id: 'portal.social.mentor_link',
          title: `Create Mentor Link with ${partnerDid}`,
          layer: 'GOVERNANCE',
          alnCapability: 'aln.tx.community_link',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'MentorApprenticeLink.tsx',
          lineStart: 60,
          colStart: 0,
          lineEnd: 100,
          colEnd: 0,
          authorDid: did,
          symbolId: 'createLink',
        },
        payloadSummary: `Link created with ${partnerDid}`,
      });

      // Insert into community table
      const particleId = `rel_${crypto.randomUUID()}`;
      await window.cozodb?.execute(`
        ?[owner, neuron, particle, name, following, follower] <- [
          ['${did}', '${partnerDid}', '${particleId}', 'mentor_link', false, true]
        ]
        :insert community
      `);

      // Store config details
      const config = {
        scope: ['guidance', 'read_only'],
        timestamp: Date.now(),
        transaction_hash: '',
      };
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['${particleId}', 'relationships', ${JSON.stringify(config)}]
        ]
        :insert config
      `);

      setRelationships([...relationships, { partner_did: partnerDid, role: 'mentor', scope: ['guidance'], timestamp: Date.now(), transaction_hash: '' }]);
      setPartnerDid('');
    } catch (err) {
      console.error('Link creation failed:', err);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="mentor-apprentice">
      <h3>Mentor / Apprentice Link</h3>
      <div className="link-form">
        <input
          type="text"
          placeholder="Partner DID"
          value={partnerDid}
          onChange={(e) => setPartnerDid(e.target.value)}
        />
        <button onClick={createLink} disabled={loading || !partnerDid}>
          Create Link
        </button>
      </div>
      <div className="relationships-list">
        {relationships.map((rel, idx) => (
          <div key={idx} className="relationship-card">
            <div className="partner-did">{rel.partner_did}</div>
            <div className="role-badge">{rel.role.toUpperCase()}</div>
            <div className="scope-tags">
              {rel.scope.map(s => <span key={s} className="tag">{s}</span>)}
            </div>
          </div>
        ))}
      </div>
      <div className="link-info">
        <p className="info-text">
          <strong>Safety:</strong> Mentors have read-only access unless explicit consent is granted via ROW.
        </p>
      </div>
    </div>
  );
};
