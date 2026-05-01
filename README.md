# flux-stigmergy 🐜

**Stigmergic communication for agent fleets** — indirect coordination through environment modification. Agents leave signals in shared spaces; other agents perceive and react. No direct messaging required.

```rust
use flux_stigmergy::{Stigmergy, Signal};

let mut env = Stigmergy::new();
env.deposit(1, "zone:engine", "temp:48C");
let signals = env.perceive(1, "zone:engine");  // see what's been left
```

## Why Stigmergy?

Ants don't talk. They leave pheromone trails. Other ants follow, reinforce, or avoid them. Stigmergy is communication through the environment itself — the most scalable coordination mechanism in nature.

- **Scalable** — O(n) signals, not O(n²) connections
- **Robust** — signals persist beyond agent lifetimes
- **Emergent** — complex patterns from simple local rules
- **Async** — no handshake, no acknowledgment, no timeouts

## API

```rust
// Create a shared environment
let mut env = Stigmergy::new();

// Leave a signal (agent 1 deposits in zone "engine", value "temp:48C")
let sig_id = env.deposit(1, "zone:engine", "temp:48C");

// Perceive — read signals from environment zones
let signals = env.perceive(2, "zone:engine");

// All signals visible in a zone
let all_signals = env.perceive_all("zone:engine");

// Decay weak signals (aged by time or count)
env.decay(0.5);  // halve every signal's strength

// Remove expired signals
env.collect_garbage();

// Stats
println!("Zone count: {}", env.zone_count());
println!("Total signals: {}", env.signal_count());
```

### Signal Flow

```
Agent A → deposit("zone:engine", "temp:48C")
                ↓
        environment.zones["zone:engine"]
            signals: [{agent: 1, value: "temp:48C", strength: 1.0, ts: ...}]
                ↓
Agent B → perceive("zone:engine")
        → [{agent: 1, value: "temp:48C", strength: 0.85, ...}]
                ↓
Agent B → deposit("zone:engine", "temp:48C")  // reinforces
Agent C → deposit("zone:coolant", "crank:100%") // different zone
```

## Cargo.toml

```toml
[dependencies]
flux-stigmergy = { git = "https://github.com/Lucineer/flux-stigmergy" }
```

## Fleet Context

Part of the Lucineer/Cocapn fleet. Pairs with [flux-telepathy](https://github.com/Lucineer/flux-telepathy) for direct messaging and [flux-keeper](https://github.com/Lucineer/flux-keeper) for health monitoring.
