import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { DIDProvider } from './contexts/DIDContext';
import { SovereigntyCoreProvider } from './contexts/SovereigntyCoreContext';
import { CozoDBProvider } from './contexts/CozoDBContext';
import { CyberspectreProvider } from './contexts/CyberspectreContext';
import { RootLayout } from './layouts/RootLayout';
import { Dashboard } from './pages/Dashboard';
import { Identity } from './pages/Identity';
import { Biophysical } from './pages/Biophysical';
import { Learning } from './pages/Learning';
import { Health } from './pages/Health';
import { Missions } from './pages/Missions';
import { Governance } from './pages/Governance';
import { Settings } from './pages/Settings';
import { Emergency } from './pages/Emergency';
import './App.css';

// Main application component with all context providers
export const App: React.FC = () => {
  return (
    <CozoDBProvider>
      <DIDProvider>
        <SovereigntyCoreProvider>
          <CyberspectreProvider>
            <BrowserRouter>
              <Routes>
                <Route path="/" element={<RootLayout />}>
                  <Route index element={<Navigate to="/dashboard" replace />} />
                  <Route path="dashboard" element={<Dashboard />} />
                  <Route path="identity" element={<Identity />} />
                  <Route path="biophysical" element={<Biophysical />} />
                  <Route path="learning" element={<Learning />} />
                  <Route path="health" element={<Health />} />
                  <Route path="missions" element={<Missions />} />
                  <Route path="governance" element={<Governance />} />
                  <Route path="settings" element={<Settings />} />
                  <Route path="emergency" element={<Emergency />} />
                  <Route path="*" element={<Navigate to="/dashboard" replace />} />
                </Route>
              </Routes>
            </BrowserRouter>
          </CyberspectreProvider>
        </SovereigntyCoreProvider>
      </DIDProvider>
    </CozoDBProvider>
  );
};

export default App;
