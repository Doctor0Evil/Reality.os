import React, { createContext, useContext, useState, ReactNode } from 'react';
import { CyberspectreIntrospectionEngine, ProvenanceNode } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface CyberspectreContextType {
  engine: CyberspectreIntrospectionEngine | null;
  recordNode: (node: ProvenanceNode) => void;
  exportRecord: () => any;
  getSessionId: () => string;
}

const CyberspectreContext = createContext<CyberspectreContextType | undefined>(undefined);

export const CyberspectreProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [engine, setEngine] = useState<CyberspectreIntrospectionEngine | null>(null);
  const [sessionId, setSessionId] = useState<string>(`session-${Date.now()}`);

  React.useEffect(() => {
    const newEngine = new CyberspectreIntrospectionEngine(sessionId);
    setEngine(newEngine);

    // Store session in localStorage for persistence
    localStorage.setItem(`cyberspectre-session-${sessionId}`, JSON.stringify({
      sessionId,
      createdAt: Date.now(),
    }));
  }, [sessionId]);

  const recordNode = (node: ProvenanceNode) => {
    if (engine) {
      engine.recordNode(node);
      
      // Persist to localStorage
      const record = engine.exportRecord();
      localStorage.setItem(`cyberspectre-session-${sessionId}`, JSON.stringify(record));
    }
  };

  const exportRecord = () => {
    return engine?.exportRecord() || { sessionId, nodes: [] };
  };

  const getSessionId = () => sessionId;

  return (
    <CyberspectreContext.Provider value={{ engine, recordNode, exportRecord, getSessionId }}>
      {children}
    </CyberspectreContext.Provider>
  );
};

export const useCyberspectre = () => {
  const context = useContext(CyberspectreContext);
  if (context === undefined) {
    throw new Error('useCyberspectre must be used within a CyberspectreProvider');
  }
  return context;
};
