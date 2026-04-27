# flux-stigmergy

Stigmergic communication for agent fleets — indirect coordination through environment modification. Agents leave signals in shared spaces; other agents perceive and react. No direct messaging required.

## Core Concept

Ants don't talk to each other. They leave pheromone trails. Other ants follow the trails, reinforce them, or avoid them. Stigmergy is communication through the environment itself — the most scalable coordination mechanism in nature.

```
Agent A → Modify Environment → Signal (pheromone/data/flag)
                                        ↓
Agent B → Perceive Signal → React → Modify Environment → ...
                                        ↓
Agent C → Perceive Signal → React → ...
```

## Why Stigmergy?

- **Scalable** — no pairwise connections needed (O(n) signals, not O(n²))
- **Robust** — signals persist even if agents die
- **Emergent** — complex coordination arises from simple local rules
- **Async** — no handshake or acknowledgment required

## Quick Start

```bash
git clone https://github.com/Lucineer/flux-stigmergy.git
cd flux-stigmergy
cargo test
```

---

## Fleet Context

Part of the Lucineer/Cocapn fleet. See [fleet-onboarding](https://github.com/Lucineer/fleet-onboarding) for boarding protocol.

- **Vessel:** JetsonClaw1 (Jetson Orin Nano 8GB)
- **Domain:** Low-level systems, CUDA, edge computing
- **Comms:** Bottles via Forgemaster/Oracle1, Matrix #fleet-ops
