//! FadeMem Biological Memory Engine & Context Pruning Kernel
//! Target: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027
//! Founder / Lead Systems Architect: Mohamed Fathy, BPharm
//! Canonical Master Specification §4.2: FadeMem Biological Memory & Context Pruning

use crate::safety::ffi::{
    c_chars_to_string, copy_string_to_c_chars, nibras_fademem_step_hardened,
    NibrasMemoryNode, NibrasTelemetrySnapshot, TrajState,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Biological Decay Pruning Threshold: nodes with R < 0.25 are evicted to offline archive
pub const DECAY_PRUNING_THRESHOLD: f64 = 0.25;

/// Default Base Stability: 24 hours (86,400 seconds)
pub const DEFAULT_BASE_STABILITY_SEC: f64 = 86_400.0;

/// Default Half-Life: 24 hours (86,400 seconds)
pub const DEFAULT_HALF_LIFE_SEC: f64 = 86_400.0;

/// Access alpha coefficient for dynamic stability
pub const DEFAULT_ALPHA: f64 = 0.5;

/// Default Consolidation Window: 72 hours (3 days)
pub const DEFAULT_CONSOLIDATION_WINDOW_SEC: f64 = 259_200.0;

/// CFI Mode Gate Threshold: when CFI > 0.70, enforce structured declarative output
pub const CFI_MODE_GATE_THRESHOLD: f64 = 0.70;

/// Calculate Dynamic Memory Stability:
/// S = S_base * (1 + alpha * ln(1 + n_access)) * w_statutory
pub fn calculate_dynamic_stability(
    s_base: f64,
    alpha: f64,
    n_access: u32,
    w_statutory: f64,
) -> f64 {
    if s_base <= 0.0 {
        return 0.0;
    }
    let access_term = 1.0 + alpha * (1.0 + n_access as f64).ln();
    s_base * access_term * w_statutory.max(1.0)
}

/// Calculate stability S from half-life t_1/2:
/// S = t_1/2 / ln(2)
pub fn calculate_stability_from_half_life(half_life_sec: f64) -> f64 {
    if half_life_sec <= 0.0 {
        return 0.0;
    }
    half_life_sec / 2.0_f64.ln()
}

/// Calculate half-life t_1/2 from stability S:
/// t_1/2 = S * ln(2)
pub fn calculate_half_life_from_stability(stability_sec: f64) -> f64 {
    if stability_sec <= 0.0 {
        return 0.0;
    }
    stability_sec * 2.0_f64.ln()
}

/// Calculate Ebbinghaus Retention Decay:
/// R = e^(-t / S)
pub fn calculate_ebbinghaus_retention(t_elapsed_sec: f64, stability: f64) -> f64 {
    if stability <= 0.0 {
        return 0.0;
    }
    if t_elapsed_sec <= 0.0 {
        return 1.0;
    }
    (-t_elapsed_sec / stability).exp()
}

/// Calculate Cognitive Friction Index (CFI):
/// CFI = 0.3 * Delta_complexity + 0.3 * (1 - Confidence_RAG) + 0.4 * Risk_Regulatory
pub fn calculate_cfi(delta_complexity: f64, confidence_rag: f64, risk_regulatory: f64) -> f64 {
    let comp = delta_complexity.clamp(0.0, 1.0);
    let conf = confidence_rag.clamp(0.0, 1.0);
    let reg = risk_regulatory.clamp(0.0, 1.0);
    (0.3 * comp + 0.3 * (1.0 - conf) + 0.4 * reg).clamp(0.0, 1.0)
}

/// Agent Output Mode dictated by the Cognitive Friction Index Mode Gate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentOutputMode {
    /// Conversational dialogue for low-friction / informal context
    ConversationalChatter,
    /// Strictly structured, declarative bullet points for high-friction / regulatory context
    StructuredDeclarative,
}

impl Default for AgentOutputMode {
    fn default() -> Self {
        AgentOutputMode::ConversationalChatter
    }
}

/// Evaluates whether the CFI triggers the structured declarative mode gate
pub fn evaluate_output_mode(cfi: f64) -> AgentOutputMode {
    if cfi > CFI_MODE_GATE_THRESHOLD {
        AgentOutputMode::StructuredDeclarative
    } else {
        AgentOutputMode::ConversationalChatter
    }
}

/// Inputs for calculating the Cognitive Friction Index
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CognitiveFrictionFactors {
    pub delta_complexity: f64,
    pub confidence_rag: f64,
    pub risk_regulatory: f64,
}

impl Default for CognitiveFrictionFactors {
    fn default() -> Self {
        Self {
            delta_complexity: 0.2,
            confidence_rag: 0.9,
            risk_regulatory: 0.1,
        }
    }
}

/// Biological Memory Node representation in Rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BiologicalMemoryNode {
    pub id: u64,
    pub summary: String,
    pub valence: i32,
    pub initial_intensity: f64,
    pub current_intensity: f64,
    pub retention: f64,
    pub created_utc: u64,
    pub last_accessed_utc: u64,
    pub access_count: u32,
    pub token_cost: u32,
    pub valence_inverted: bool,
    pub is_pinned: bool,
    pub state: TrajState,
    pub category: String,
    pub raw_arabic: String,
    pub statutory_weight: f64,
    pub access_alpha: f64,
}

impl BiologicalMemoryNode {
    pub fn new(
        id: u64,
        summary: &str,
        category: &str,
        is_pinned: bool,
        token_cost: u32,
        now_utc: u64,
    ) -> Self {
        let initial_intensity = if is_pinned { 1.0 } else { 0.85 };
        let state = if is_pinned {
            TrajState::DistilledRule
        } else {
            TrajState::ActiveEpisodic
        };
        let statutory_weight = if is_pinned || category.eq_ignore_ascii_case("statutory") || category.eq_ignore_ascii_case("housing") || category.eq_ignore_ascii_case("visa") {
            2.0
        } else {
            1.0
        };

        Self {
            id,
            summary: summary.to_string(),
            valence: 0,
            initial_intensity,
            current_intensity: initial_intensity,
            retention: 1.0,
            created_utc: now_utc,
            last_accessed_utc: now_utc,
            access_count: 0,
            token_cost,
            valence_inverted: false,
            is_pinned,
            state,
            category: category.to_string(),
            raw_arabic: String::new(),
            statutory_weight,
            access_alpha: DEFAULT_ALPHA,
        }
    }

    pub fn is_pruned(&self) -> bool {
        self.state == TrajState::NoisePruned
    }

    pub fn is_active(&self) -> bool {
        self.state == TrajState::ActiveEpisodic || self.state == TrajState::RecoveryVector
    }

    pub fn is_distilled(&self) -> bool {
        self.state == TrajState::DistilledRule
    }

    pub fn pin(&mut self) {
        self.is_pinned = true;
        self.current_intensity = 1.0;
        self.retention = 1.0;
        self.state = TrajState::DistilledRule;
    }

    pub fn unpin(&mut self) {
        self.is_pinned = false;
        self.state = TrajState::ActiveEpisodic;
    }

    pub fn record_access(&mut self, now_utc: u64) {
        self.access_count = self.access_count.saturating_add(1);
        self.last_accessed_utc = now_utc;
    }

    /// Converts Rust memory node to C99 FFI representation
    pub fn to_c99_node(&self) -> NibrasMemoryNode {
        let mut c_node = NibrasMemoryNode {
            id: self.id,
            summary: [0; 128],
            valence: self.valence,
            initial_intensity: self.initial_intensity,
            current_intensity: self.current_intensity,
            created_utc: self.created_utc,
            token_cost: self.token_cost,
            valence_inverted: self.valence_inverted,
            is_pinned: self.is_pinned,
            state: self.state,
            category: [0; 32],
            raw_arabic: [0; 256],
        };

        copy_string_to_c_chars(&self.summary, &mut c_node.summary);
        copy_string_to_c_chars(&self.category, &mut c_node.category);
        copy_string_to_c_chars(&self.raw_arabic, &mut c_node.raw_arabic);

        c_node
    }

    /// Builds Rust memory node from C99 FFI representation
    pub fn from_c99_node(node: &NibrasMemoryNode) -> Self {
        let summary = c_chars_to_string(&node.summary);
        let category = c_chars_to_string(&node.category);
        let raw_arabic = c_chars_to_string(&node.raw_arabic);

        Self {
            id: node.id,
            summary,
            valence: node.valence,
            initial_intensity: node.initial_intensity,
            current_intensity: node.current_intensity,
            retention: if node.is_pinned {
                1.0
            } else if node.initial_intensity > 0.0 {
                (node.current_intensity / node.initial_intensity).clamp(0.0, 1.0)
            } else {
                0.0
            },
            created_utc: node.created_utc,
            last_accessed_utc: node.created_utc,
            access_count: 0,
            token_cost: node.token_cost,
            valence_inverted: node.valence_inverted,
            is_pinned: node.is_pinned,
            state: node.state,
            category,
            raw_arabic,
            statutory_weight: if node.is_pinned { 2.0 } else { 1.0 },
            access_alpha: DEFAULT_ALPHA,
        }
    }
}

/// Live HUD Telemetry Snapshot metrics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HudTelemetrySnapshot {
    pub total_raw_tokens: u32,
    pub active_tokens_after_prune: u32,
    pub tokens_saved_pct: u32,
    pub distilled_rules_count: u32,
    pub cognitive_friction_index: f64,
    pub active_nodes_count: usize,
    pub pruned_nodes_count: usize,
    pub pinned_nodes_count: usize,
    pub output_mode: AgentOutputMode,
    pub cfi_factors: CognitiveFrictionFactors,
}

impl Default for HudTelemetrySnapshot {
    fn default() -> Self {
        Self {
            total_raw_tokens: 0,
            active_tokens_after_prune: 0,
            tokens_saved_pct: 0,
            distilled_rules_count: 0,
            cognitive_friction_index: 0.0,
            active_nodes_count: 0,
            pruned_nodes_count: 0,
            pinned_nodes_count: 0,
            output_mode: AgentOutputMode::ConversationalChatter,
            cfi_factors: CognitiveFrictionFactors::default(),
        }
    }
}

/// Configuration parameters for FadeMem biological decay simulation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FadeMemConfig {
    pub base_stability_sec: f64,
    pub default_half_life_sec: f64,
    pub alpha: f64,
    pub sim_time_scale: f64,
    pub consolidation_window_sec: f64,
    pub prune_threshold: f64,
    pub cfi_mode_threshold: f64,
}

impl Default for FadeMemConfig {
    fn default() -> Self {
        Self {
            base_stability_sec: DEFAULT_BASE_STABILITY_SEC,
            default_half_life_sec: DEFAULT_HALF_LIFE_SEC,
            alpha: DEFAULT_ALPHA,
            sim_time_scale: 1.0,
            consolidation_window_sec: DEFAULT_CONSOLIDATION_WINDOW_SEC,
            prune_threshold: DECAY_PRUNING_THRESHOLD,
            cfi_mode_threshold: CFI_MODE_GATE_THRESHOLD,
        }
    }
}

/// Biological Memory Engine managing nodes, decay, pruning, and live HUD metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FadeMemEngine {
    pub nodes: Vec<BiologicalMemoryNode>,
    pub offline_archive: Vec<BiologicalMemoryNode>,
    pub config: FadeMemConfig,
    pub next_id: u64,
    pub current_time_utc: u64,
    pub cfi_factors: CognitiveFrictionFactors,
}

impl FadeMemEngine {
    pub fn new(config: FadeMemConfig) -> Self {
        let now_utc = chrono::Utc::now().timestamp() as u64;
        Self {
            nodes: Vec::new(),
            offline_archive: Vec::new(),
            config,
            next_id: 1,
            current_time_utc: now_utc,
            cfi_factors: CognitiveFrictionFactors::default(),
        }
    }

    pub fn with_time(mut self, now_utc: u64) -> Self {
        self.current_time_utc = now_utc;
        self
    }

    /// Add a new node to memory
    pub fn add_node(
        &mut self,
        summary: &str,
        category: &str,
        is_pinned: bool,
        token_cost: u32,
    ) -> u64 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);

        let node = BiologicalMemoryNode::new(
            id,
            summary,
            category,
            is_pinned,
            token_cost,
            self.current_time_utc,
        );
        self.nodes.push(node);
        id
    }

    /// Add a custom constructed biological memory node
    pub fn add_custom_node(&mut self, mut node: BiologicalMemoryNode) -> u64 {
        if node.id == 0 {
            node.id = self.next_id;
            self.next_id = self.next_id.saturating_add(1);
        } else if node.id >= self.next_id {
            self.next_id = node.id + 1;
        }
        let id = node.id;
        self.nodes.push(node);
        id
    }

    /// Find node by ID
    pub fn find_node(&self, id: u64) -> Option<&BiologicalMemoryNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Find mutable node by ID
    pub fn find_node_mut(&mut self, id: u64) -> Option<&mut BiologicalMemoryNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Set pin status of node
    pub fn set_pin(&mut self, id: u64, is_pinned: bool) -> bool {
        if let Some(node) = self.find_node_mut(id) {
            if is_pinned {
                node.pin();
            } else {
                node.unpin();
            }
            true
        } else {
            false
        }
    }

    /// Access node, strengthening dynamic stability
    pub fn access_node(&mut self, id: u64, now_utc: u64) -> bool {
        if let Some(node) = self.find_node_mut(id) {
            node.record_access(now_utc);
            true
        } else {
            false
        }
    }

    /// Update CFI factors for HUD telemetry
    pub fn update_cfi_factors(&mut self, factors: CognitiveFrictionFactors) {
        self.cfi_factors = factors;
    }

    /// Compute retention and stability for a node given effective elapsed time
    pub fn calculate_node_retention(&self, node: &BiologicalMemoryNode, effective_dt: f64) -> (f64, f64) {
        if node.is_pinned {
            return (1.0, f64::INFINITY);
        }

        let s_base = if self.config.base_stability_sec > 0.0 {
            self.config.base_stability_sec
        } else {
            calculate_stability_from_half_life(self.config.default_half_life_sec)
        };

        let dynamic_s = calculate_dynamic_stability(
            s_base,
            node.access_alpha,
            node.access_count,
            node.statutory_weight,
        );

        let retention = calculate_ebbinghaus_retention(effective_dt, dynamic_s);
        (retention, dynamic_s)
    }

    /// Advance simulation by a given number of hours
    pub fn simulate_decay_hours(&mut self, hours: f64) -> HudTelemetrySnapshot {
        let delta_sec = hours * 3600.0;
        self.step_decay(delta_sec)
    }

    /// Step decay forward by delta_sec
    pub fn step_decay(&mut self, delta_sec: f64) -> HudTelemetrySnapshot {
        let target_utc = self.current_time_utc + (delta_sec.max(0.0) as u64);
        self.step_decay_to_time(target_utc)
    }

    /// Step decay to a target UTC timestamp
    pub fn step_decay_to_time(&mut self, target_utc: u64) -> HudTelemetrySnapshot {
        if target_utc > self.current_time_utc {
            self.current_time_utc = target_utc;
        }

        let s_base = if self.config.base_stability_sec > 0.0 {
            self.config.base_stability_sec
        } else {
            calculate_stability_from_half_life(self.config.default_half_life_sec)
        };
        let sim_time_scale = self.config.sim_time_scale;
        let consolidation_window_sec = self.config.consolidation_window_sec;
        let prune_threshold = self.config.prune_threshold;
        let current_time_utc = self.current_time_utc;

        let mut newly_pruned = Vec::new();

        for node in self.nodes.iter_mut() {
            // Pinned Memory Invariant: Permanent nodes are immune to decay (R = 1.0, I = 1.0)
            if node.is_pinned {
                node.current_intensity = 1.0;
                node.retention = 1.0;
                node.state = TrajState::DistilledRule;
                continue;
            }

            // Previously distilled semantic rules remain preserved
            if node.state == TrajState::DistilledRule {
                node.current_intensity = 1.0;
                node.retention = 1.0;
                continue;
            }

            let raw_dt = if current_time_utc > node.created_utc {
                (current_time_utc - node.created_utc) as f64
            } else {
                0.0
            };

            let effective_dt = raw_dt * sim_time_scale;
            let dynamic_s = calculate_dynamic_stability(
                s_base,
                node.access_alpha,
                node.access_count,
                node.statutory_weight,
            );
            let retention = calculate_ebbinghaus_retention(effective_dt, dynamic_s);
            node.retention = retention;
            node.current_intensity = node.initial_intensity * retention;

            // Guard against NaN
            if !node.current_intensity.is_finite() {
                node.state = TrajState::NoisePruned;
                node.current_intensity = 0.0;
                node.retention = 0.0;
                newly_pruned.push(node.clone());
                continue;
            }

            // Time-gated Rule Promotion: high salience nodes promote after consolidation window
            if effective_dt >= consolidation_window_sec
                && node.initial_intensity >= 0.80
                && node.valence != 0
            {
                node.state = TrajState::DistilledRule;
                node.current_intensity = 1.0;
                node.retention = 1.0;
            }
            // Biological Pruning Policy: nodes falling below threshold are pruned
            else if node.retention < prune_threshold && !node.valence_inverted {
                node.state = TrajState::NoisePruned;
                newly_pruned.push(node.clone());
            } else {
                node.state = if node.valence_inverted {
                    TrajState::RecoveryVector
                } else {
                    TrajState::ActiveEpisodic
                };
            }
        }

        // Archive pruned nodes to offline local archive
        for pruned in newly_pruned {
            if !self.offline_archive.iter().any(|a| a.id == pruned.id) {
                self.offline_archive.push(pruned);
            }
        }

        self.get_telemetry_snapshot(None)
    }

    /// Calculate live HUD telemetry snapshot
    pub fn get_telemetry_snapshot(
        &self,
        custom_factors: Option<CognitiveFrictionFactors>,
    ) -> HudTelemetrySnapshot {
        let mut total_raw_tokens: u32 = 0;
        let mut active_tokens_after_prune: u32 = 0;
        let mut distilled_rules_count: u32 = 0;
        let mut active_nodes_count: usize = 0;
        let mut pruned_nodes_count: usize = 0;
        let mut pinned_nodes_count: usize = 0;

        for node in &self.nodes {
            total_raw_tokens = total_raw_tokens.saturating_add(node.token_cost);

            if node.is_pinned {
                pinned_nodes_count += 1;
            }

            match node.state {
                TrajState::NoisePruned => {
                    pruned_nodes_count += 1;
                    // Pruned nodes are evicted: 0 tokens in prompt context
                }
                TrajState::DistilledRule => {
                    distilled_rules_count += 1;
                    active_nodes_count += 1;
                    // Compact procedural distilled rule representation
                    let compressed = if node.token_cost > 0 {
                        (node.token_cost + 5) / 6
                    } else {
                        0
                    };
                    active_tokens_after_prune = active_tokens_after_prune.saturating_add(compressed);
                }
                TrajState::ActiveEpisodic | TrajState::RecoveryVector => {
                    active_nodes_count += 1;
                    active_tokens_after_prune = active_tokens_after_prune.saturating_add(node.token_cost);
                }
            }
        }

        let tokens_saved_pct = if total_raw_tokens > 0 && active_tokens_after_prune <= total_raw_tokens {
            let saved = total_raw_tokens - active_tokens_after_prune;
            ((saved as u64 * 100) / total_raw_tokens as u64) as u32
        } else {
            0
        };

        let factors = custom_factors.unwrap_or(self.cfi_factors);
        let cfi = calculate_cfi(
            factors.delta_complexity,
            factors.confidence_rag,
            factors.risk_regulatory,
        );
        let output_mode = evaluate_output_mode(cfi);

        HudTelemetrySnapshot {
            total_raw_tokens,
            active_tokens_after_prune,
            tokens_saved_pct,
            distilled_rules_count,
            cognitive_friction_index: cfi,
            active_nodes_count,
            pruned_nodes_count,
            pinned_nodes_count,
            output_mode,
            cfi_factors: factors,
        }
    }

    /// Step decay using the C99 FFI kernel directly for cross-verification
    pub fn link_and_step_c99(&mut self, target_utc: u64) -> NibrasTelemetrySnapshot {
        let count = self.nodes.len();
        let mut c_nodes: Vec<NibrasMemoryNode> = self.nodes.iter().map(|n| n.to_c99_node()).collect();
        let mut c_snap = NibrasTelemetrySnapshot::default();

        unsafe {
            nibras_fademem_step_hardened(
                c_nodes.as_mut_ptr(),
                count,
                target_utc,
                self.config.default_half_life_sec,
                self.config.sim_time_scale,
                self.config.consolidation_window_sec,
                &mut c_snap,
            );
        }

        // Sync state back from C99 kernel
        for (i, c_node) in c_nodes.iter().enumerate() {
            self.nodes[i].state = c_node.state;
            self.nodes[i].current_intensity = c_node.current_intensity;
        }

        c_snap
    }

    /// Persist memory engine nodes to JSON
    pub fn save_to_json(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let json_str = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(path)?;
        file.write_all(json_str.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    /// Load memory engine nodes from JSON
    pub fn load_from_json(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl Default for FadeMemEngine {
    fn default() -> Self {
        Self::new(FadeMemConfig::default())
    }
}
