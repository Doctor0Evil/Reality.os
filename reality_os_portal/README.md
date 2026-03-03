# Reality.OS - Augmented Citizen Portal

A sovereign, offline-capable, biophysical dApp for augmented citizens built on ALN (Organichain) with CozoDB local caching.

## Features

- **100 Non-Monetary Functions**: Identity, biophysical management, learning, health, missions, governance, and UX layers
- **DID-Bound Authentication**: Self-sovereign identity with forward-only key rotation
- **CozoDB Local Caching**: Offline-first architecture with sync to ALN mainnet
- **Cyberspectre Introspection**: All actions recorded as provenance nodes for audit trails
- **Neurorights Enforcement**: Safety corridors and RoH envelopes enforced at the protocol level
- **Emergency NeuroStop**: Always-available safety override for all bound devices

## Prerequisites

- Node.js 18+
- CozoDB (local instance for caching)
- SovereigntyCore (local instance for host-sovereign validation)
- ALN Mainnet access (https://rpc.bostrom.cybernode.ai)

## Installation

```bash
# Clone repository
git clone https://github.com/Doctor0Evil/Reality.os.git
cd reality_os_portal

# Install dependencies
npm install

# Copy environment variables
cp .env.example .env

# Start CozoDB (local instance)
cozodb run --host 127.0.0.1 --port 9001

# Start development server
npm run dev
