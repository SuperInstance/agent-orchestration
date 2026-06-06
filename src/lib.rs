//! # agent-orchestration
//!
//! Orchestration for agent fleets, modeled on orchestral composition.
//!
//! An orchestra isn't a group of musicians playing simultaneously. It's a
//! carefully balanced system where each instrument has a role, a section,
//! a dynamic range, and a moment to shine. The conductor doesn't tell them
//! what to play — the score does that. The conductor shapes *when* and *how*.
//!
//! This crate treats agent fleets the same way: agents are instruments with
//! roles and ranges, sections group related capabilities, and the orchestrator
//! shapes dynamics — who's loud, who's soft, who rests, who carries the melody.

use std::collections::HashMap;

/// Musical dynamic markings, repurposed for agent intensity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dynamic {
    Pianissimo = 0,   // pp — barely active, listening only
    Piano = 1,         // p  — low activity, background processing
    MezzoPiano = 2,    // mp — moderate-low, supporting role
    MezzoForte = 3,    // mf — moderate, standard operation
    Forte = 4,         // f  — active, driving the task
    Fortissimo = 5,    // ff — maximum effort, all hands on deck
}

impl Dynamic {
    /// Interpret a dynamic as a resource fraction (0.0 to 1.0).
    pub fn intensity(&self) -> f64 {
        match self {
            Dynamic::Pianissimo => 0.1,
            Dynamic::Piano => 0.25,
            Dynamic::MezzoPiano => 0.4,
            Dynamic::MezzoForte => 0.6,
            Dynamic::Forte => 0.8,
            Dynamic::Fortissimo => 1.0,
        }
    }

    /// Crescendo: step up one dynamic level.
    pub fn crescendo(&self) -> Dynamic {
        match self {
            Dynamic::Pianissimo => Dynamic::Piano,
            Dynamic::Piano => Dynamic::MezzoPiano,
            Dynamic::MezzoPiano => Dynamic::MezzoForte,
            Dynamic::MezzoForte => Dynamic::Forte,
            Dynamic::Forte => Dynamic::Fortissimo,
            Dynamic::Fortissimo => Dynamic::Fortissimo,
        }
    }

    /// Decrescendo: step down one dynamic level.
    pub fn decrescendo(&self) -> Dynamic {
        match self {
            Dynamic::Pianissimo => Dynamic::Pianissimo,
            Dynamic::Piano => Dynamic::Pianissimo,
            Dynamic::MezzoPiano => Dynamic::Piano,
            Dynamic::MezzoForte => Dynamic::MezzoPiano,
            Dynamic::Forte => Dynamic::MezzoForte,
            Dynamic::Fortissimo => Dynamic::Forte,
        }
    }
}

/// The role an agent plays in the fleet — modeled on orchestral instruments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentRole {
    /// Bass — foundation. Slow, reliable, always present. Like a bass line.
    Bass,
    /// Harmony — supports the melody. Background processing, infrastructure.
    Harmony,
    /// Melody — carries the main task. Front-line, visible, high-impact.
    Melody,
    /// Percussion — timing and rhythm. Scheduling, heartbeats, alerts.
    Percussion,
    /// Solo — specialized, emerges for specific moments. Expert agents.
    Solo,
    /// Rest — not currently active, but available.
    Rest,
}

impl AgentRole {
    /// Typical dynamic range for this role.
    pub fn default_dynamic(&self) -> Dynamic {
        match self {
            AgentRole::Bass => Dynamic::MezzoPiano,
            AgentRole::Harmony => Dynamic::MezzoForte,
            AgentRole::Melody => Dynamic::Forte,
            AgentRole::Percussion => Dynamic::MezzoForte,
            AgentRole::Solo => Dynamic::Fortissimo,
            AgentRole::Rest => Dynamic::Pianissimo,
        }
    }

    /// Does this role carry the primary action right now?
    pub fn is_frontline(&self) -> bool {
        matches!(self, AgentRole::Melody | AgentRole::Solo)
    }
}

/// An instrument (agent) in the orchestra.
#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub role: AgentRole,
    pub dynamic: Dynamic,
    pub section: String,
    pub capability: f64, // 0.0 to 1.0 — how capable this agent is
}

impl Instrument {
    pub fn new(name: &str, role: AgentRole, section: &str, capability: f64) -> Self {
        let dynamic = role.default_dynamic();
        Self { name: name.to_string(), role, dynamic, section: section.to_string(), capability }
    }

    /// Effective output: capability × dynamic intensity.
    pub fn effective_output(&self) -> f64 {
        self.capability * self.dynamic.intensity()
    }
}

/// A section of the orchestra (group of agents with related capabilities).
#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub instruments: Vec<String>,
}

/// The full orchestral score — the plan for who does what and when.
#[derive(Debug, Clone)]
pub struct Score {
    /// Which agents exist.
    pub instruments: HashMap<String, Instrument>,
    /// Sections (groups).
    pub sections: Vec<Section>,
    /// Current measure (step in the plan).
    pub measure: usize,
}

impl Score {
    pub fn new() -> Self {
        Self { instruments: HashMap::new(), sections: Vec::new(), measure: 0 }
    }

    /// Add an instrument to the score.
    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.instruments.insert(instrument.name.clone(), instrument);
    }

    /// Add a section.
    pub fn add_section(&mut self, name: &str, instrument_names: Vec<String>) {
        self.sections.push(Section {
            name: name.to_string(),
            instruments: instrument_names.iter().map(|s| s.to_string()).collect(),
        });
    }

    /// Calculate section balance — how evenly distributed is the output.
    /// Returns 1.0 for perfect balance, lower for imbalance.
    pub fn section_balance(&self) -> f64 {
        if self.sections.is_empty() { return 1.0; }
        let section_outputs: Vec<f64> = self.sections.iter().map(|s| {
            s.instruments.iter()
                .filter_map(|name| self.instruments.get(name))
                .map(|i| i.effective_output())
                .sum()
        }).collect();
        let max = section_outputs.iter().cloned().fold(0.0_f64, f64::max);
        let avg = section_outputs.iter().sum::<f64>() / section_outputs.len() as f64;
        if max == 0.0 { 1.0 } else { avg / max }
    }

    /// Get total effective output across all instruments.
    pub fn total_output(&self) -> f64 {
        self.instruments.values().map(|i| i.effective_output()).sum()
    }

    /// Who's carrying the melody right now? (highest-output frontline agent)
    pub fn melody_carrier(&self) -> Option<&Instrument> {
        self.instruments.values()
            .filter(|i| i.role.is_frontline())
            .max_by(|a, b| a.effective_output().partial_cmp(&b.effective_output()).unwrap())
    }

    /// Advance to next measure — apply dynamic changes.
    pub fn next_measure(&mut self) {
        self.measure += 1;
    }

    /// Crescendo for a section.
    pub fn section_crescendo(&mut self, section_name: &str) {
        if let Some(section) = self.sections.iter().find(|s| s.name == section_name) {
            for name in &section.instruments {
                if let Some(instrument) = self.instruments.get_mut(name) {
                    instrument.dynamic = instrument.dynamic.crescendo();
                }
            }
        }
    }

    /// Decrescendo for a section.
    pub fn section_decrescendo(&mut self, section_name: &str) {
        if let Some(section) = self.sections.iter().find(|s| s.name == section_name) {
            for name in &section.instruments {
                if let Some(instrument) = self.instruments.get_mut(name) {
                    instrument.dynamic = instrument.dynamic.decrescendo();
                }
            }
        }
    }

    /// Solo: one agent goes fortissimo, everyone else drops to piano.
    pub fn solo_spotlight(&mut self, soloist: &str) {
        for (name, instrument) in self.instruments.iter_mut() {
            if name == soloist {
                instrument.role = AgentRole::Solo;
                instrument.dynamic = Dynamic::Fortissimo;
            } else if instrument.role.is_frontline() {
                instrument.dynamic = instrument.dynamic.decrescendo();
            }
        }
    }

    /// Tutti: everyone plays at their role's default dynamic.
    pub fn tutti(&mut self) {
        for instrument in self.instruments.values_mut() {
            instrument.dynamic = instrument.role.default_dynamic();
        }
    }
}

impl Default for Score {
    fn default() -> Self { Self::new() }
}

/// Experiment: orchestral balance affects fleet performance.
pub fn run_balance_experiment(
    agents: usize,
    measures: usize,
    balanced: bool,
) -> (f64, f64) {
    let mut score = Score::new();
    let roles = [AgentRole::Bass, AgentRole::Harmony, AgentRole::Melody, AgentRole::Percussion];
    let sections = ["rhythm", "harmony", "melody"];

    for i in 0..agents {
        let role = roles[i % roles.len()].clone();
        let section = sections[i % sections.len()];
        let cap = if balanced { 0.8 } else { 0.3 + (i as f64 / agents as f64) * 0.7 };
        score.add_instrument(Instrument::new(&format!("agent-{}", i), role, section, cap));
    }
    for (si, sec_name) in ["rhythm", "harmony", "melody"].iter().enumerate() {
        let names: Vec<String> = (0..agents)
            .filter(|i| i % 3 == si)
            .map(|i| format!("agent-{}", i))
            .collect();
        score.add_section(sec_name, names);
    }

    let mut total_output = 0.0;
    let mut total_balance = 0.0;
    for _ in 0..measures {
        total_output += score.total_output();
        total_balance += score.section_balance();
        score.next_measure();
        // Cycle dynamics
        if score.measure % 4 == 0 { score.section_crescendo("melody"); }
        if score.measure % 4 == 2 { score.section_decrescendo("melody"); }
    }
    (total_output / measures as f64, total_balance / measures as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_crescendo() {
        assert_eq!(Dynamic::Piano.crescendo(), Dynamic::MezzoPiano);
        assert_eq!(Dynamic::Fortissimo.crescendo(), Dynamic::Fortissimo);
    }

    #[test]
    fn test_dynamic_decrescendo() {
        assert_eq!(Dynamic::Forte.decrescendo(), Dynamic::MezzoForte);
        assert_eq!(Dynamic::Pianissimo.decrescendo(), Dynamic::Pianissimo);
    }

    #[test]
    fn test_dynamic_intensity_range() {
        assert!(Dynamic::Pianissimo.intensity() < Dynamic::Fortissimo.intensity());
        assert!(Dynamic::Forte.intensity() > 0.5);
    }

    #[test]
    fn test_instrument_effective_output() {
        let inst = Instrument::new("test", AgentRole::Melody, "front", 0.8);
        assert!((inst.effective_output() - 0.8 * Dynamic::Forte.intensity()).abs() < 1e-10);
    }

    #[test]
    fn test_melody_carrier() {
        let mut score = Score::new();
        score.add_instrument(Instrument::new("bass", AgentRole::Bass, "low", 0.9));
        score.add_instrument(Instrument::new("lead", AgentRole::Melody, "front", 0.7));
        let carrier = score.melody_carrier().unwrap();
        assert_eq!(carrier.name, "lead");
    }

    #[test]
    fn test_section_crescendo() {
        let mut score = Score::new();
        score.add_instrument(Instrument::new("a1", AgentRole::Melody, "melody", 0.8));
        score.add_instrument(Instrument::new("a2", AgentRole::Bass, "rhythm", 0.8));
        score.add_section("melody", vec!["a1".to_string()]);
        let before = score.instruments["a1"].dynamic;
        score.section_crescendo("melody");
        assert_eq!(score.instruments["a1"].dynamic, before.crescendo());
    }

    #[test]
    fn test_solo_spotlight() {
        let mut score = Score::new();
        score.add_instrument(Instrument::new("soloist", AgentRole::Melody, "front", 0.9));
        score.add_instrument(Instrument::new("backup", AgentRole::Harmony, "mid", 0.7));
        score.solo_spotlight("soloist");
        assert_eq!(score.instruments["soloist"].dynamic, Dynamic::Fortissimo);
        assert_eq!(score.instruments["soloist"].role, AgentRole::Solo);
    }

    #[test]
    fn test_tutti_resets() {
        let mut score = Score::new();
        score.add_instrument(Instrument::new("a", AgentRole::Melody, "front", 0.8));
        score.solo_spotlight("a");
        score.tutti();
        assert_eq!(score.instruments["a"].dynamic, score.instruments["a"].role.default_dynamic());
    }

    #[test]
    fn test_balance_experiment_runs() {
        let (output, balance) = run_balance_experiment(6, 20, true);
        assert!(output > 0.0);
        assert!(balance > 0.0);
    }
}
