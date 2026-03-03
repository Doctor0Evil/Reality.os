import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

interface CozoDBContextType {
  db: any;
  isConnected: boolean;
  execute: (query: string) => Promise<any>;
  initialize: () => Promise<void>;
}

const CozoDBContext = createContext<CozoDBContextType | undefined>(undefined);

export const CozoDBProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [db, setDb] = useState<any>(null);
  const [isConnected, setIsConnected] = useState(false);

  const initialize = async () => {
    try {
      // Initialize CozoDB client (using wasm or http transport)
      const CozoClient = (window as any).CozoClient;
      if (CozoClient) {
        const client = new CozoClient('http://localhost:9001'); // Local CozoDB instance
        await client.run('[.info]');
        setDb(client);
        setIsConnected(true);

        // Initialize schema tables from export.json.txt.txt
        await initializeSchema(client);
      } else {
        // Mock CozoDB for development
        console.warn('CozoDB client not available, using mock implementation');
        setDb({
          execute: async (query: string) => {
            console.log('Mock CozoDB query:', query);
            return { results: [] };
          },
        });
        setIsConnected(true);
      }
    } catch (err) {
      console.error('Failed to initialize CozoDB:', err);
    }
  };

  const initializeSchema = async (client: any) => {
    // Create tables based on export.json.txt.txt schema
    const schemaQueries = [
      // community table
      ':create community { owner_id: String, neuron: String => particle: String, name: String, following: Bool, follower: Bool }',
      // config table
      ':create config { key: String, group_key: String => value: Json }',
      // link table
      ':create link { from: String, to: String, neuron: String => timestamp: Int, transaction_hash: String }',
      // particle table
      ':create particle { cid: String => mime: String, text: String, blocks: Int, size: Int, size_local: Int, type: String }',
      // sync_status table
      ':create sync_status { owner_id: String, id: String => entry_type: Int, disabled: Bool, timestamp_update: Int, timestamp_read: Int, unread_count: Int, meta: Json }',
      // transaction table
      ':create transaction { hash: String, index: Int, neuron: String, type: String => block_height: Int, success: Bool, timestamp: Int, value: Json, memo: String }',
    ];

    for (const query of schemaQueries) {
      try {
        await client.run(query);
      } catch (err) {
        // Table may already exist
        console.log('Schema table already exists or failed:', query);
      }
    }
  };

  const execute = async (query: string) => {
    if (!db) {
      throw new Error('CozoDB not initialized');
    }
    try {
      const result = await db.run(query);
      return result;
    } catch (err) {
      console.error('CozoDB query failed:', err);
      throw err;
    }
  };

  useEffect(() => {
    initialize();
  }, []);

  // Expose CozoDB to window for components that need direct access
  useEffect(() => {
    (window as any).cozodb = db;
  }, [db]);

  return (
    <CozoDBContext.Provider value={{ db, isConnected, execute, initialize }}>
      {children}
    </CozoDBContext.Provider>
  );
};

export const useCozoDB = () => {
  const context = useContext(CozoDBContext);
  if (context === undefined) {
    throw new Error('useCozoDB must be used within a CozoDBProvider');
  }
  return context;
};
