# agent-orchestration

*The conductor doesn't play. The conductor listens, and shapes the moment.*

---

An orchestra isn't musicians playing at the same time. It's a system where each instrument has a role (bass, harmony, melody, percussion), a dynamic range (pianissimo to fortissimo), and a moment. The conductor shapes *when* and *how* — who carries the melody, who supports, who rests.

This crate treats agent fleets the same way. Agents are instruments. Sections group related capabilities. The score tracks who's doing what and at what intensity. Dynamics (pp/p/mp/mf/f/ff) control resource allocation. Solo spotlight gives one agent maximum resources while others pull back. Tutti restores the full ensemble.

The mappings aren't metaphors:
- **Bass** = foundational services (always running, low intensity)
- **Harmony** = supporting agents (background processing, infrastructure)
- **Melody** = frontline agents (carrying the main task, high visibility)
- **Percussion** = timing agents (heartbeats, scheduling, alerts)
- **Solo** = specialist agents (emerge for specific moments, maximum effort)
- **Dynamic markings** = resource allocation (pp = 10%, ff = 100%)

The section balance metric tells you whether your fleet is balanced or one section is overwhelming the others. The melody carrier tells you who's driving the task right now.

This is how you'd design a fleet if you thought of it as an orchestra instead of a cluster.

9 tests: dynamics (crescendo/decrescendo/intensity), instrument output, melody carrier, section crescendo, solo spotlight, tutti reset, balance experiment.

Part of [SuperInstance](https://github.com/SuperInstance/SuperInstance). Connects to [agent-sync](https://github.com/SuperInstance/agent-sync) (timing), [agent-groove](https://github.com/SuperInstance/agent-groove) (rhythm), [musician-soul](https://github.com/SuperInstance/musician-soul) (identity).

License: MIT
