//! Safe, idiomatic Rust API wrapping the inviolable C99 Nibras Safety & Grounding Kernel.

pub mod ffi;

pub use ffi::{GroundingVerdict, NibrasEntityType, TrajState};
use ffi::*;
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::os::raw::c_char;

/// Safe representation of an extracted entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExtractedEntity {
    pub entity_type: NibrasEntityType,
    pub label: String,
    pub text: String,
    pub offset: usize,
}

/// Safe representation of extracted Arabic NLP information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedInfo {
    pub entities: Vec<ExtractedEntity>,
    pub is_action_item: bool,
    pub is_appointment: bool,
    pub is_executive_decision: bool,
    pub action_verb: String,
    pub appointment_title: String,
    pub datetime_str: String,
    pub suggested_salience: f64,
    pub suggest_pin: bool,
    pub suggested_valence: i32,
}

/// Authoritative reference source snippet for grounding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSnippet {
    pub source_id: String,
    pub source_title: String,
    pub snippet_text: String,
    pub source_numerical: f64,
    pub doc_id: String,
    pub section: String,
    pub page: i32,
    pub passage_sha256: String,
}

/// Safe Grounding Verification Result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerifiedClaimResult {
    pub verdict: GroundingVerdict,
    pub confidence: f64,
    pub citation: String,
    pub discrepancy_note: String,
    pub doc_id: String,
    pub section: String,
    pub page: i32,
    pub passage_sha256: String,
}

impl VerifiedClaimResult {
    pub fn is_verified(&self) -> bool {
        self.verdict == GroundingVerdict::Verified
    }

    pub fn is_hallucination(&self) -> bool {
        self.verdict == GroundingVerdict::HallucinationTripped
    }
}

/// Safe wrapper for memory node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: u64,
    pub summary: String,
    pub valence: i32,
    pub initial_intensity: f64,
    pub current_intensity: f64,
    pub created_utc: u64,
    pub token_cost: u32,
    pub valence_inverted: bool,
    pub is_pinned: bool,
    pub state: TrajState,
    pub category: String,
    pub raw_arabic: String,
}

/// Safe wrapper for telemetry snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub total_raw_tokens: u32,
    pub active_tokens_after_prune: u32,
    pub tokens_saved_pct: u32,
    pub distilled_rules_count: u32,
    pub cognitive_friction_index: f64,
}

/// Helper function to convert C char buffer to Rust String
pub(crate) fn c_chars_to_string(buf: &[c_char]) -> String {
    let mut bytes = Vec::new();
    for &b in buf {
        if b == 0 {
            break;
        }
        bytes.push(b as u8);
    }
    String::from_utf8_lossy(&bytes).trim().to_string()
}

/// Helper to copy Rust string into fixed C char buffer
pub(crate) fn copy_string_to_c_chars(s: &str, buf: &mut [c_char]) {
    let bytes = s.as_bytes();
    let max = buf.len().saturating_sub(1);
    let len = bytes.len().min(max);
    for i in 0..len {
        buf[i] = bytes[i] as c_char;
    }
    buf[len] = 0;
}

/// Redact PII from text using hardened C99 redactor
pub fn redact_pii(text: &str) -> String {
    let c_text = CString::new(text).unwrap_or_default();
    let cap = text.len() * 2 + 128;
    let mut out_buf = vec![0 as c_char; cap];

    unsafe {
        grounding_redact_prompt_pii_c99(c_text.as_ptr(), out_buf.as_mut_ptr(), out_buf.len());
    }

    c_chars_to_string(&out_buf)
}

/// Scrub query of sensitive terms for zero-leakage search
pub fn scrub_query(text: &str) -> String {
    let c_text = CString::new(text).unwrap_or_default();
    let cap = text.len() * 2 + 128;
    let mut out_buf = vec![0 as c_char; cap];

    unsafe {
        grounding_scrub_search_query(c_text.as_ptr(), out_buf.as_mut_ptr(), out_buf.len());
    }

    c_chars_to_string(&out_buf)
}

/// Normalize Arabic text (tashkeel stripping, tatweel removal, alef/hamza unification)
pub fn normalize_arabic(text: &str) -> String {
    let c_text = CString::new(text).unwrap_or_default();
    let cap = text.len() * 2 + 128;
    let mut out_buf = vec![0 as c_char; cap];

    unsafe {
        arabic_normalize_utf8_hardened(c_text.as_ptr(), out_buf.as_mut_ptr(), out_buf.len());
    }

    c_chars_to_string(&out_buf)
}

/// Extract Arabic entities, appointments, and action items
pub fn extract_arabic_info(normalized_text: &str) -> ExtractedInfo {
    let c_text = CString::new(normalized_text).unwrap_or_default();
    let mut raw_info = NibrasExtractedInfo::default();

    unsafe {
        nibras_extract_arabic_entities(c_text.as_ptr(), &mut raw_info);
    }

    let mut entities = Vec::new();
    for i in 0..raw_info.entity_count.min(16) {
        let e = &raw_info.entities[i];
        entities.push(ExtractedEntity {
            entity_type: e.entity_type,
            label: c_chars_to_string(&e.label),
            text: c_chars_to_string(&e.text),
            offset: e.offset,
        });
    }

    ExtractedInfo {
        entities,
        is_action_item: raw_info.is_action_item,
        is_appointment: raw_info.is_appointment,
        is_executive_decision: raw_info.is_executive_decision,
        action_verb: c_chars_to_string(&raw_info.action_verb),
        appointment_title: c_chars_to_string(&raw_info.appointment_title),
        datetime_str: c_chars_to_string(&raw_info.datetime_str),
        suggested_salience: raw_info.suggested_salience,
        suggest_pin: raw_info.suggest_pin,
        suggested_valence: raw_info.suggested_valence,
    }
}

/// Cross-check factual claim against authoritative snippets via C99 Grounding Shield
pub fn verify_grounding_claim(
    claim_text: &str,
    entity_name: &str,
    numerical_value: Option<f64>,
    sources: &[SourceSnippet],
) -> VerifiedClaimResult {
    let c_claim_text = CString::new(claim_text).unwrap_or_default();
    let c_entity_name = CString::new(entity_name).unwrap_or_default();

    let raw_claim = GroundingClaim {
        claim_text: c_claim_text.as_ptr(),
        entity_name: c_entity_name.as_ptr(),
        numerical_value: numerical_value.unwrap_or(0.0),
        has_numerical: numerical_value.is_some(),
    };

    let mut c_strings: Vec<(CString, CString, CString, CString, CString)> = Vec::new();
    let mut raw_sources: Vec<GroundingSource> = Vec::with_capacity(sources.len());

    for s in sources {
        let sid = CString::new(s.source_id.as_str()).unwrap_or_default();
        let stitle = CString::new(s.source_title.as_str()).unwrap_or_default();
        let stext = CString::new(s.snippet_text.as_str()).unwrap_or_default();
        let sdoc = CString::new(s.doc_id.as_str()).unwrap_or_default();
        let ssec = CString::new(s.section.as_str()).unwrap_or_default();

        let mut hash_arr = [0 as c_char; 65];
        copy_string_to_c_chars(&s.passage_sha256, &mut hash_arr);

        raw_sources.push(GroundingSource {
            source_id: sid.as_ptr(),
            source_title: stitle.as_ptr(),
            snippet_text: stext.as_ptr(),
            source_numerical: s.source_numerical,
            doc_id: sdoc.as_ptr(),
            section: ssec.as_ptr(),
            page: s.page,
            passage_sha256: hash_arr,
        });

        c_strings.push((sid, stitle, stext, sdoc, ssec));
    }

    let mut out_res = GroundingResult::default();
    unsafe {
        grounding_verify_claim_ptr(
            &raw_claim,
            if raw_sources.is_empty() {
                std::ptr::null()
            } else {
                raw_sources.as_ptr()
            },
            raw_sources.len(),
            &mut out_res,
        );
    }

    VerifiedClaimResult {
        verdict: out_res.verdict,
        confidence: out_res.confidence,
        citation: c_chars_to_string(&out_res.citation),
        discrepancy_note: c_chars_to_string(&out_res.discrepancy_note),
        doc_id: c_chars_to_string(&out_res.doc_id),
        section: c_chars_to_string(&out_res.section),
        page: out_res.page,
        passage_sha256: c_chars_to_string(&out_res.passage_sha256),
    }
}

/// High-level Personal RAG engine wrapping SQLite FTS5 C kernel
pub struct PersonalRag {
    raw: NibrasRag,
}

impl PersonalRag {
    pub fn new(db_path: &str) -> Result<Self, String> {
        let mut raw = NibrasRag {
            db: std::ptr::null_mut(),
            is_open: false,
            db_path: [0; 256],
            total_chunks_indexed: 0,
            total_notes_indexed: 0,
        };

        let c_path = CString::new(db_path).map_err(|e| e.to_string())?;
        let rc = unsafe { nibras_rag_init(&mut raw, c_path.as_ptr()) };
        if rc != 0 {
            return Err(format!("Failed to initialize Personal RAG (code: {})", rc));
        }

        Ok(Self { raw })
    }

    pub fn index_note(&mut self, note_id: u64, title: &str, full_text: &str) -> usize {
        let c_title = CString::new(title).unwrap_or_default();
        let c_text = CString::new(full_text).unwrap_or_default();

        unsafe {
            nibras_rag_index_note(&mut self.raw, note_id, c_title.as_ptr(), c_text.as_ptr())
        }
    }

    pub fn retrieve(&mut self, query: &str, max_results: usize) -> Vec<(String, String, f64)> {
        let c_query = CString::new(query).unwrap_or_default();
        let mut results = vec![NibrasRagResult {
            chunk: NibrasRagChunk {
                chunk_id: 0,
                note_id: 0,
                note_title: [0; NIBRAS_RAG_MAX_TITLE_LEN],
                start_line: 0,
                end_line: 0,
                passage_sha256: [0; NIBRAS_RAG_SHA256_HEX_LEN],
                text: [0; NIBRAS_RAG_MAX_PASSAGE_LEN],
                token_count: 0,
            },
            score: 0.0,
            confidence: 0.0,
            citation: [0; NIBRAS_RAG_MAX_CITATION_LEN],
            match_reason: [0; 160],
        }; max_results.min(16)];

        let count = unsafe {
            nibras_rag_retrieve(
                &mut self.raw,
                c_query.as_ptr(),
                results.as_mut_ptr(),
                results.len(),
            )
        };

        let mut out = Vec::new();
        for i in 0..count {
            let citation = c_chars_to_string(&results[i].citation);
            let text = c_chars_to_string(&results[i].chunk.text);
            let score = results[i].score;
            out.push((citation, text, score));
        }
        out
    }

    pub fn answer_meeting_query(&mut self, query: &str) -> Option<(String, String)> {
        let c_query = CString::new(query).unwrap_or_default();
        let mut answer_buf = vec![0 as c_char; 512];
        let mut citation_buf = vec![0 as c_char; 256];

        let rc = unsafe {
            nibras_rag_answer_meeting_query(
                &mut self.raw,
                c_query.as_ptr(),
                answer_buf.as_mut_ptr(),
                answer_buf.len(),
                citation_buf.as_mut_ptr(),
                citation_buf.len(),
            )
        };

        if rc == 0 {
            let answer = c_chars_to_string(&answer_buf);
            let citation = c_chars_to_string(&citation_buf);
            Some((answer, citation))
        } else {
            None
        }
    }
}

impl Drop for PersonalRag {
    fn drop(&mut self) {
        unsafe {
            nibras_rag_close(&mut self.raw);
        }
    }
}
