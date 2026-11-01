//! Low-level C99 FFI bindings for Nibras C99 Safety, Grounding, and Memory Kernel.
//! Links directly to `libnibras_adapters` (compiled from C99 safety engine).

use serde::{Deserialize, Serialize};
use std::os::raw::{c_char, c_double, c_int, c_void};

/// Grounding Shield Verdict
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroundingVerdict {
    Verified = 0,
    HallucinationTripped = 1,
    Unverifiable = 2,
    PiiBlocked = 3,
    Error = 4,
}

impl Default for GroundingVerdict {
    fn default() -> Self {
        GroundingVerdict::Verified
    }
}

/// Extracted factual claim structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GroundingClaim {
    pub claim_text: *const c_char,
    pub entity_name: *const c_char,
    pub numerical_value: c_double,
    pub has_numerical: bool,
}

impl Default for GroundingClaim {
    fn default() -> Self {
        Self {
            claim_text: std::ptr::null(),
            entity_name: std::ptr::null(),
            numerical_value: 0.0,
            has_numerical: false,
        }
    }
}

/// Authoritative reference source snippet
#[repr(C)]
#[derive(Debug, Clone)]
pub struct GroundingSource {
    pub source_id: *const c_char,
    pub source_title: *const c_char,
    pub snippet_text: *const c_char,
    pub source_numerical: c_double,
    pub doc_id: *const c_char,
    pub section: *const c_char,
    pub page: c_int,
    pub passage_sha256: [c_char; 65],
}

impl Default for GroundingSource {
    fn default() -> Self {
        Self {
            source_id: std::ptr::null(),
            source_title: std::ptr::null(),
            snippet_text: std::ptr::null(),
            source_numerical: 0.0,
            doc_id: std::ptr::null(),
            section: std::ptr::null(),
            page: 0,
            passage_sha256: [0; 65],
        }
    }
}

/// Grounding verification result envelope
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GroundingResult {
    pub verdict: GroundingVerdict,
    pub confidence: c_double,
    pub citation: [c_char; 128],
    pub discrepancy_note: [c_char; 256],
    pub doc_id: [c_char; 64],
    pub section: [c_char; 64],
    pub page: c_int,
    pub passage_sha256: [c_char; 65],
}

impl Default for GroundingResult {
    fn default() -> Self {
        Self {
            verdict: GroundingVerdict::Verified,
            confidence: 0.0,
            citation: [0; 128],
            discrepancy_note: [0; 256],
            doc_id: [0; 64],
            section: [0; 64],
            page: 0,
            passage_sha256: [0; 65],
        }
    }
}

/// Trajectory node state
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrajState {
    NoisePruned = 0,
    ActiveEpisodic = 1,
    DistilledRule = 2,
    RecoveryVector = 3,
}

impl Default for TrajState {
    fn default() -> Self {
        TrajState::ActiveEpisodic
    }
}

/// Memory Node C layout
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasMemoryNode {
    pub id: u64,
    pub summary: [c_char; 128],
    pub valence: i32,
    pub initial_intensity: c_double,
    pub current_intensity: c_double,
    pub created_utc: u64,
    pub token_cost: u32,
    pub valence_inverted: bool,
    pub is_pinned: bool,
    pub state: TrajState,
    pub category: [c_char; 32],
    pub raw_arabic: [c_char; 256],
}

impl Default for NibrasMemoryNode {
    fn default() -> Self {
        Self {
            id: 0,
            summary: [0; 128],
            valence: 0,
            initial_intensity: 1.0,
            current_intensity: 1.0,
            created_utc: 0,
            token_cost: 0,
            valence_inverted: false,
            is_pinned: false,
            state: TrajState::ActiveEpisodic,
            category: [0; 32],
            raw_arabic: [0; 256],
        }
    }
}

/// Telemetry snapshot C layout
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct NibrasTelemetrySnapshot {
    pub total_raw_tokens: u32,
    pub active_tokens_after_prune: u32,
    pub tokens_saved_pct: u32,
    pub distilled_rules_count: u32,
    pub cognitive_friction_index: c_double,
}

/// Entity types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NibrasEntityType {
    None = 0,
    Action = 1,
    Appointment = 2,
    Datetime = 3,
    Decision = 4,
}

impl Default for NibrasEntityType {
    fn default() -> Self {
        NibrasEntityType::None
    }
}

/// Extracted entity
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasEntity {
    pub entity_type: NibrasEntityType,
    pub label: [c_char; 32],
    pub text: [c_char; 96],
    pub offset: usize,
}

impl Default for NibrasEntity {
    fn default() -> Self {
        Self {
            entity_type: NibrasEntityType::None,
            label: [0; 32],
            text: [0; 96],
            offset: 0,
        }
    }
}

/// Extracted information envelope
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasExtractedInfo {
    pub entity_count: usize,
    pub entities: [NibrasEntity; 16],
    pub is_action_item: bool,
    pub is_appointment: bool,
    pub is_executive_decision: bool,
    pub action_verb: [c_char; 64],
    pub appointment_title: [c_char; 64],
    pub datetime_str: [c_char; 64],
    pub suggested_salience: c_double,
    pub suggest_pin: bool,
    pub suggested_valence: i32,
}

impl Default for NibrasExtractedInfo {
    fn default() -> Self {
        Self {
            entity_count: 0,
            entities: [NibrasEntity::default(); 16],
            is_action_item: false,
            is_appointment: false,
            is_executive_decision: false,
            action_verb: [0; 64],
            appointment_title: [0; 64],
            datetime_str: [0; 64],
            suggested_salience: 0.5,
            suggest_pin: false,
            suggested_valence: 0,
        }
    }
}

pub const NIBRAS_MAX_MEMORY_NODES: usize = 1024;
pub const NIBRAS_MAX_DOC_TOKENS: usize = 32;
pub const NIBRAS_MAX_TOKEN_LEN: usize = 48;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasDocTerm {
    pub term: [c_char; NIBRAS_MAX_TOKEN_LEN],
    pub freq: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasDocIndex {
    pub terms: [NibrasDocTerm; NIBRAS_MAX_DOC_TOKENS],
    pub term_count: u16,
    pub total_words: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasSearchResult {
    pub node: NibrasMemoryNode,
    pub score: c_double,
    pub match_reason: [c_char; 160],
}

#[repr(C)]
pub struct NibrasMemoryStore {
    pub nodes: [NibrasMemoryNode; NIBRAS_MAX_MEMORY_NODES],
    pub index: [NibrasDocIndex; NIBRAS_MAX_MEMORY_NODES],
    pub count: usize,
    pub filepath: [c_char; 256],
    pub next_id: u64,
}

pub const NIBRAS_RAG_MAX_TITLE_LEN: usize = 128;
pub const NIBRAS_RAG_MAX_PASSAGE_LEN: usize = 1024;
pub const NIBRAS_RAG_MAX_CITATION_LEN: usize = 256;
pub const NIBRAS_RAG_SHA256_HEX_LEN: usize = 65;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasRagChunk {
    pub chunk_id: u64,
    pub note_id: u64,
    pub note_title: [c_char; NIBRAS_RAG_MAX_TITLE_LEN],
    pub start_line: u32,
    pub end_line: u32,
    pub passage_sha256: [c_char; NIBRAS_RAG_SHA256_HEX_LEN],
    pub text: [c_char; NIBRAS_RAG_MAX_PASSAGE_LEN],
    pub token_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NibrasRagResult {
    pub chunk: NibrasRagChunk,
    pub score: c_double,
    pub confidence: c_double,
    pub citation: [c_char; NIBRAS_RAG_MAX_CITATION_LEN],
    pub match_reason: [c_char; 160],
}

#[repr(C)]
pub struct NibrasRag {
    pub db: *mut c_void,
    pub is_open: bool,
    pub db_path: [c_char; 256],
    pub total_chunks_indexed: usize,
    pub total_notes_indexed: usize,
}

extern "C" {
    // Grounding Shield
    pub fn grounding_scrub_search_query(
        raw_user_prompt: *const c_char,
        out_scrubbed_query: *mut c_char,
        out_cap: usize,
    ) -> usize;

    pub fn grounding_redact_prompt_pii_c99(
        src: *const c_char,
        dst: *mut c_char,
        dst_cap: usize,
    ) -> usize;

    pub fn grounding_verify_claim(
        claim: *const GroundingClaim,
        sources: *const GroundingSource,
        source_count: usize,
    ) -> GroundingResult;

    pub fn grounding_verify_claim_ptr(
        claim: *const GroundingClaim,
        sources: *const GroundingSource,
        source_count: usize,
        out_result: *mut GroundingResult,
    ) -> GroundingVerdict;

    // Arabic and FadeMem Adapters
    pub fn arabic_normalize_utf8_hardened(
        src: *const c_char,
        dst: *mut c_char,
        dst_cap: usize,
    ) -> usize;

    pub fn nibras_extract_arabic_entities(
        normalized_text: *const c_char,
        out_info: *mut NibrasExtractedInfo,
    ) -> usize;

    pub fn nibras_fademem_step_hardened(
        nodes: *mut NibrasMemoryNode,
        count: usize,
        now_utc: u64,
        half_life_sec: c_double,
        sim_time_scale: c_double,
        consolidation_window_sec: c_double,
        out_snap: *mut NibrasTelemetrySnapshot,
    );

    // Persistent Memory Store
    pub fn nibras_store_init(store: *mut NibrasMemoryStore, filepath: *const c_char) -> c_int;

    pub fn nibras_store_ingest(
        store: *mut NibrasMemoryStore,
        raw_text: *const c_char,
        token_cost: u32,
        out_extracted_info: *mut NibrasExtractedInfo,
    ) -> u64;

    pub fn nibras_store_add_node(
        store: *mut NibrasMemoryStore,
        node: *const NibrasMemoryNode,
    ) -> u64;

    pub fn nibras_store_find_by_id(
        store: *mut NibrasMemoryStore,
        id: u64,
    ) -> *mut NibrasMemoryNode;

    pub fn nibras_store_set_pin(
        store: *mut NibrasMemoryStore,
        id: u64,
        pin: bool,
    ) -> c_int;

    pub fn nibras_store_invert_valence(
        store: *mut NibrasMemoryStore,
        id: u64,
    ) -> c_int;

    pub fn nibras_store_step_decay(
        store: *mut NibrasMemoryStore,
        now_utc: u64,
        half_life_sec: c_double,
        sim_time_scale: c_double,
        consolidation_window_sec: c_double,
        out_snap: *mut NibrasTelemetrySnapshot,
    );

    pub fn nibras_store_save(
        store: *const NibrasMemoryStore,
        filepath: *const c_char,
    ) -> c_int;

    pub fn nibras_store_load(
        store: *mut NibrasMemoryStore,
        filepath: *const c_char,
    ) -> c_int;

    pub fn nibras_store_build_index(store: *mut NibrasMemoryStore);

    pub fn nibras_store_search_ranked(
        store: *const NibrasMemoryStore,
        query: *const c_char,
        out_results: *mut NibrasSearchResult,
        max_results: usize,
    ) -> usize;

    pub fn nibras_store_search(
        store: *const NibrasMemoryStore,
        query: *const c_char,
        out_matches: *mut NibrasMemoryNode,
        max_matches: usize,
    ) -> usize;

    // Personal RAG
    pub fn nibras_sha256_hex(
        data: *const c_void,
        len: usize,
        hex_out: *mut c_char,
    );

    pub fn nibras_rag_init(rag: *mut NibrasRag, db_path: *const c_char) -> c_int;

    pub fn nibras_rag_close(rag: *mut NibrasRag);

    pub fn nibras_rag_index_note(
        rag: *mut NibrasRag,
        note_id: u64,
        title: *const c_char,
        full_text: *const c_char,
    ) -> usize;

    pub fn nibras_rag_index_memory_store(
        rag: *mut NibrasRag,
        store: *const NibrasMemoryStore,
    ) -> usize;

    pub fn nibras_rag_retrieve(
        rag: *mut NibrasRag,
        query: *const c_char,
        out_results: *mut NibrasRagResult,
        max_results: usize,
    ) -> usize;

    pub fn nibras_rag_verify_claim(
        rag: *mut NibrasRag,
        claim: *const GroundingClaim,
    ) -> GroundingResult;

    pub fn nibras_rag_answer_meeting_query(
        rag: *mut NibrasRag,
        query: *const c_char,
        out_answer: *mut c_char,
        answer_cap: usize,
        out_citation: *mut c_char,
        citation_cap: usize,
    ) -> c_int;
}
