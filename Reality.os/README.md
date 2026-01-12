# Reality.orchestrator stack

This workspace provides a lab-grade, production-ready Rust toolchain to:

- Clone and update Reality.os, Dreamscape.os, and XR-Grid-Infrastructure in a single command.
- Maintain a manifest (`manifest.reality.toml`) describing CyberOrganic.os-aligned projects.
- Prepare local and Kubernetes-backed targets for later simulation, XR, and dream-lab deployment.

## Quick start

```bash
# from a clean machine with Rust installed
git clone https://github.com/Doctor0Evil/Reality.os.git
cd Reality.os

# build the orchestrator + lab cli
cargo build -p reality-orchestrator -p reality-lab-cli --release

# initialize (writes ~/.config/RealityOrchestrator/manifest.reality.toml)
./target/release/reality-orchestrator init

# clone/update Reality.os, Dreamscape.os, XR-Grid-Infrastructure into ~/xr-lab
./target/release/reality-orchestrator clone-all

# inspect your lab workspace
./target/release/reality-lab list
