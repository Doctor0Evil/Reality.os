import React, { useState } from 'react';
import { Outlet, Link, useLocation } from 'react-router-dom';
import { useDID } from '../contexts/DIDContext';
import { EmergencyNeuroStop } from '../components/EmergencyNeuroStop';
import { RealityOSModeSwitcher } from '../components/RealityOSModeSwitcher';

interface NavItem {
  path: string;
  label: string;
  icon: string;
}

const NAV_ITEMS: NavItem[] = [
  { path: '/dashboard', label: 'Dashboard', icon: '📊' },
  { path: '/identity', label: 'Identity', icon: '🆔' },
  { path: '/biophysical', label: 'Biophysical', icon: '🧠' },
  { path: '/learning', label: 'Learning', icon: '📚' },
  { path: '/health', label: 'Health', icon: '❤️' },
  { path: '/missions', label: 'Missions', icon: '🎯' },
  { path: '/governance', label: 'Governance', icon: '⚖️' },
  { path: '/settings', label: 'Settings', icon: '⚙️' },
];

export const RootLayout: React.FC = () => {
  const location = useLocation();
  const { did, isConnected } = useDID();
  const [sidebarOpen, setSidebarOpen] = useState(true);

  return (
    <div className="root-layout">
      {/* Emergency Stop - Always Visible */}
      <div className="emergency-bar">
        <EmergencyNeuroStop />
        <RealityOSModeSwitcher />
      </div>

      {/* Sidebar Navigation */}
      <aside className={`sidebar ${sidebarOpen ? 'open' : 'closed'}`}>
        <div className="sidebar-header">
          <h1>Reality.OS</h1>
          <button onClick={() => setSidebarOpen(!sidebarOpen)} className="toggle-btn">
            {sidebarOpen ? '◀' : '▶'}
          </button>
        </div>

        {isConnected && did && (
          <div className="citizen-info">
            <p className="citizen-did">{did.substring(0, 20)}...</p>
            <span className="connection-status connected">● Connected</span>
          </div>
        )}

        {!isConnected && (
          <div className="citizen-info">
            <span className="connection-status disconnected">● Disconnected</span>
          </div>
        )}

        <nav className="nav-menu">
          {NAV_ITEMS.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              className={`nav-item ${location.pathname === item.path ? 'active' : ''}`}
            >
              <span className="nav-icon">{item.icon}</span>
              {sidebarOpen && <span className="nav-label">{item.label}</span>}
            </Link>
          ))}
        </nav>

        <div className="sidebar-footer">
          {sidebarOpen && (
            <p className="version-info">
              Reality.OS v2.0.0 | ALN Mainnet
            </p>
          )}
        </div>
      </aside>

      {/* Main Content Area */}
      <main className="main-content">
        <div className="content-wrapper">
          <Outlet />
        </div>
      </main>
    </div>
  );
};
