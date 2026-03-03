# Reality.OS Architecture

## System Overview

flowchart TB
  subgraph Portal["Reality.OS Portal (100 Surfaces)"]
    A[Identity & Reputation 1-10]
    B[Biophysical & BCI 11-20]
    C[Learning & Dev 21-30]
    D[Health & Safety 31-40]
    E[Missions & Swarms 41-50]
    F[Governance & Rights 51-60]
    G[UX & Modes 61-70]
    H[Core ALN Surfaces 71-100]
  end

  subgraph CozoDB["CozoDB Local Cache"]
    I[community table]
    J[config table]
    K[link table]
    L[particle table]
    M[sync_status table]
    N[transaction table]
  end

  subgraph ALNCore["ALN Core Chain"]
    O[Tx Format & Blocks]
    P[DID Registry & Keys]
    Q[WASM Engine & Gas Model]
    R[IAVL State Trees]
    S[Tendermint BFT Phases]
    T[ROW / RPM Ledgers]
  end

  subgraph SovereigntyCore["SovereigntyCore Host"]
    U[Corridor Evaluation]
    V[RoH Envelope Enforcement]
    W[Energy Budget Management]
    X[RPC Session Management]
  end

  Portal --> CozoDB
  Portal --> SovereigntyCore
  CozoDB --> ALNCore
  SovereigntyCore --> ALNCore
  SovereigntyCore --> Portal

  style Portal fill:#e8f4f8,stroke:#2c3e50
  style CozoDB fill:#f0e8f8,stroke:#8e44ad
  style ALNCore fill:#e8f8f0,stroke:#27ae60
  style SovereigntyCore fill:#f8f0e8,stroke:#d35400
