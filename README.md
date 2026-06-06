# agent-orchestration

**Fleet coordination as orchestral composition.**

An orchestra isn't a group of musicians playing simultaneously. It's a carefully balanced system where each instrument has a role, a section, a dynamic range, and a moment to shine. The conductor doesn't tell the first violin what to play — the score does that. The conductor shapes *when* and *how*. The difference between "everyone play louder" and a well-executed crescendo is the difference between noise and music.

`agent-orchestration` treats agent fleets the same way. Agents are instruments with roles and ranges. Sections group related capabilities. Dynamics (pp to ff) control resource allocation. And the conductor — your orchestration logic — shapes the overall sound without micromanaging individual players.

## Why This Exists

Most fleet orchestration is reactive: scale up when load is high, scale down when it's low. That's a thermostat, not a conductor. It works, but it's crude.

Orchestral dynamics offer something better: **graduated, context-aware intensity control.** Pianissimo isn't "off" — it's listening, watching, ready. Fortissimo isn't "everything maxed out" — it's a focused burst where the melody section carries the load while the rhythm section holds the foundation.

The key insight: **the conductor doesn't tell each musician what to play. They set the overall dynamic.** The musicians handle the details within that frame. That's the right abstraction for fleet management too.

## Core Idea

Six dynamic levels, from barely active to all hands on deck:

| Dynamic | Mark | Intensity | Agent Behavior |
|---------|------|-----------|---------------|
| Pianissimo | pp | 0.1 | Listening only, minimal processing |
| Piano | p | 0.25 | Background tasks, low priority |
| Mezzo-piano | mp | 0.4 | Supporting role, steady state |
| Mezzo-forte | mf | 0.6 | Standard operation, normal load |
| Forte | f | 0.8 | Driving the task, high activity |
| Fortissimo | ff | 1.0 | Maximum effort, all hands on deck |

Five orchestral roles for agents:

| Role | Analogy | Default Dynamic |
|------|---------|----------------|
| Bass | Foundation — slow, reliable, always present | mp |
| Harmony | Support — background processing, infrastructure | mf |
| Melody | Lead — front-line, visible, high-impact | f |
| Percussion | Timing — scheduling, heartbeats, alerts | mf |
| Solo | Specialist — emerges for specific moments | ff |
| Rest | Available but not active | pp |

## Architecture

```
Dynamic (pp → p → mp → mf → f → ff)
  ├─ intensity() → f64 (0.1 to 1.0)
  ├─ crescendo() → next level up
  └─ decrescendo() → next level down

AgentRole (Bass / Harmony / Melody / Percussion / Solo / Rest)
  └─ default_dynamic() → Dynamic

Instrument (agent with role + dynamic + capability)
  └─ effective_output() = capability × dynamic.intensity()

Score (the orchestral plan)
  ├─ instruments: HashMap<String, Instrument>
  ├─ sections: Vec<Section>
  ├─ section_crescendo(section) / section_decrescendo(section)
  ├─ solo_spotlight(agent) → one agent ff, others decrescendo
  ├─ tutti() → everyone at role default
  ├─ melody_carrier() → highest-output frontline agent
  └─ section_balance() → how evenly distributed is output
```

## Usage

### Building an Orchestra

```rust
use agent_orchestration::*;

let mut score = Score::new();

// Add instruments with roles and capabilities
score.add_instrument(Instrument::new("db-watcher", AgentRole::Bass, "rhythm", 0.9));
score.add_instrument(Instrument::new("cache-warmer", AgentRole::Harmony, "harmony", 0.7));
score.add_instrument(Instrument::new("query-handler", AgentRole::Melody, "melody", 0.85));
score.add_instrument(Instrument::new("scheduler", AgentRole::Percussion, "rhythm", 0.8));

// Group into sections
score.add_section("rhythm", vec!["db-watcher".into(), "scheduler".into()]);
score.add_section("harmony", vec!["cache-warmer".into()]);
score.add_section("melody", vec!["query-handler".into()]);
```

### Dynamic Control

```rust
// Ramp up the melody section for peak load
score.section_crescendo("melody");
// melody agents: mf → f → ff over multiple calls

// Ramp down when things calm down
score.section_decrescendo("melody");
// melody agents: ff → f → mf → p → pp
```

Notice the asymmetry: crescendo and decrescendo are smooth, one-step transitions. Real conductors don't jump from pp to ff — they build. Your fleet shouldn't either.

### Spotlight Solo

```rust
// Emergency: one expert agent takes the lead
score.solo_spotlight("incident-handler");
// incident-handler → Solo role, ff dynamic
// All other frontline agents decrescendo
```

The solo spotlight does something clever: it doesn't silence everyone. The bass and harmony sections keep playing at their normal levels. Only other frontline agents (Melody, Solo) decrescendo. The foundation stays solid while the expert takes the lead.

### Finding the Melody Carrier

```rust
// Who's carrying the load right now?
if let Some(carrier) = score.melody_carrier() {
    println!("Lead agent: {} at {} effective output", 
        carrier.name, carrier.effective_output());
}
```

`melody_carrier()` returns the highest-output frontline agent. This is useful for monitoring — if your melody carrier has low effective output, something's wrong. Either they're under-capable or their dynamic is too low.

### Measuring Balance

```rust
// How evenly distributed is the fleet?
let balance = score.section_balance(); // 0.0 to 1.0
// 1.0 = perfect balance, lower = imbalance

// Total fleet output
let output = score.total_output();
```

Section balance is `average_output / max_output` across sections. A balanced orchestra uses all its sections effectively. An imbalanced one is overworking some while others idle.

### Running the Experiment

```rust
// Compare balanced vs unbalanced fleets
let (balanced_output, balanced_balance) = run_balance_experiment(6, 20, true);
let (random_output, random_balance) = run_balance_experiment(6, 20, false);
```

## API Reference

| Type | Purpose |
|------|---------|
| `Dynamic` | 6-level intensity (pp through ff) |
| `AgentRole` | Orchestral role classification |
| `Instrument` | Agent with role, dynamic, and capability |
| `Section` | Named group of instruments |
| `Score` | Full orchestral plan with dynamic control |

### Score Methods

| Method | Effect |
|--------|--------|
| `add_instrument(inst)` | Register an agent |
| `add_section(name, agents)` | Group agents |
| `section_crescendo(section)` | Step up dynamics for a section |
| `section_decrescendo(section)` | Step down dynamics for a section |
| `solo_spotlight(agent)` | One agent to ff, others fade |
| `tutti()` | Reset everyone to role defaults |
| `melody_carrier()` | Highest-output frontline agent |
| `section_balance()` | Balance score (0.0-1.0) |
| `total_output()` | Sum of effective output |

## The Deeper Idea

The orchestral model solves a problem that reactive scaling doesn't: **it separates *what* from *how much*.** An agent's role determines what it does. Its dynamic determines how intensely it does it. These are orthogonal concerns that most fleet managers conflate.

Consider: when load spikes, a thermostat scales everything up. A conductor makes a musical decision. Maybe the melody section crescendos while the harmony section holds steady. Maybe a solo agent takes the lead while the ensemble supports. The conductor has *vocabulary* — crescendo, decrescendo, solo, tutti — that the thermostat lacks.

The `solo_spotlight` method is the clearest example. In a crisis, you don't scale up the entire fleet. You put one expert on the problem and have everyone else support. That's not scaling — that's musical judgment.

## Related Crates

- **`agent-groove`** — Timing and feel for scheduling (the *pocket*)
- **`agent-phrasing`** — Energy contour detection (the *shape*)
- **`agent-intonation`** — Accuracy measurement (how *in tune*)
- **`agent-counterpoint`** — Species counterpoint for coordination (how voices *move*)
- **`agent-ensemble`** — The experiment proving musical coordination wins

## License

MIT
