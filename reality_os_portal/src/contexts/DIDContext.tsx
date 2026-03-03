import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { CozoDBProvider } from './CozoDBContext';

interface DIDContextType {
  did: string | null;
  isConnected: boolean;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
  isLoading: boolean;
  error: string | null;
}

const DIDContext = createContext<DIDContextType | undefined>(undefined);

export const DIDProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [did, setDid] = useState<string | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    // Check for existing DID in CozoDB config table
    const checkExistingDID = async () => {
      try {
        const result = await window.cozodb?.execute(`
          ?[value] := *config{key: 'citizen_did', group_key: 'identity', value: value}
          :order value.timestamp desc
          :limit 1
        `);
        
        if (result?.results?.length > 0) {
          const storedDid = JSON.parse(result.results[0][0]).did;
          setDid(storedDid);
          setIsConnected(true);
        }
      } catch (err) {
        console.error('Failed to check existing DID:', err);
      } finally {
        setIsLoading(false);
      }
    };

    checkExistingDID();
  }, []);

  const connect = async () => {
    setIsLoading(true);
    setError(null);
    try {
      // Generate or retrieve DID from SovereigntyCore
      const newDid = `did:aln:bostrom:${crypto.randomUUID()}`;
      
      // Store in CozoDB config table
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['citizen_did', 'identity', ${JSON.stringify({ did: newDid, timestamp: Date.now() })}]
        ]
        :insert config
      `);

      setDid(newDid);
      setIsConnected(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to connect');
    } finally {
      setIsLoading(false);
    }
  };

  const disconnect = async () => {
    try {
      // Clear DID from storage (forward-only: create new entry marking as disconnected)
      await window.cozodb?.execute(`
        ?[key, group, value] <- [
          ['citizen_did', 'identity', ${JSON.stringify({ did: null, timestamp: Date.now(), disconnected: true })}]
        ]
        :insert config
      `);

      setDid(null);
      setIsConnected(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to disconnect');
    }
  };

  return (
    <DIDContext.Provider value={{ did, isConnected, connect, disconnect, isLoading, error }}>
      {children}
    </DIDContext.Provider>
  );
};

export const useDID = () => {
  const context = useContext(DIDContext);
  if (context === undefined) {
    throw new Error('useDID must be used within a DIDProvider');
  }
  return context;
};
