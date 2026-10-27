/* nibras_rag.h - Personal Sovereign RAG & Citation Grounding Engine
 * Target: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027
 * Engine: Almaz Cognitive Organism | C99 SQLite FTS5 + BM25 Grounding Shield
 * Compliance: UAE Federal Decree-Law No. 45 of 2021 (PDPL)
 */
#ifndef NIBRAS_RAG_H
#define NIBRAS_RAG_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <sqlite3.h>

#include "grounding_shield.h"
#include "nibras_adapters.h"
#include "nibras_memory_store.h"

#ifdef __cplusplus
extern "C" {
#endif

#define NIBRAS_RAG_MAX_TITLE_LEN    128
#define NIBRAS_RAG_MAX_PASSAGE_LEN  1024
#define NIBRAS_RAG_MAX_CITATION_LEN 256
#define NIBRAS_RAG_SHA256_HEX_LEN   65
#define NIBRAS_RAG_MAX_RESULTS      16

/* A chunk extracted from a user note / document */
typedef struct {
    uint64_t chunk_id;
    uint64_t note_id;
    char     note_title[NIBRAS_RAG_MAX_TITLE_LEN];
    uint32_t start_line;
    uint32_t end_line;
    char     passage_sha256[NIBRAS_RAG_SHA256_HEX_LEN];
    char     text[NIBRAS_RAG_MAX_PASSAGE_LEN];
    uint32_t token_count;
} nibras_rag_chunk_t;

/* Retrieval result with exact citation and relevance metrics */
typedef struct {
    nibras_rag_chunk_t chunk;
    double             score;              /* BM25 relevance score */
    double             confidence;         /* Verification confidence (0.0 - 1.0) */
    char               citation[NIBRAS_RAG_MAX_CITATION_LEN]; /* Title:L<start>-L<end> [sha256:<hash>] */
    char               match_reason[160];  /* Match explanation */
} nibras_rag_result_t;

/* Personal RAG Engine State */
typedef struct {
    sqlite3 *db;
    bool     is_open;
    char     db_path[256];
    size_t   total_chunks_indexed;
    size_t   total_notes_indexed;
} nibras_rag_t;

/* Compute FIPS 180-2 SHA-256 producing 64 lowercase hex characters + NUL */
void nibras_sha256_hex(const void *data, size_t len, char hex_out[NIBRAS_RAG_SHA256_HEX_LEN]);

/* Initialize Personal RAG engine with SQLite FTS5 (use ":memory:" or file path) */
int nibras_rag_init(nibras_rag_t *rag, const char *db_path);

/* Close RAG engine and free SQLite database handle */
void nibras_rag_close(nibras_rag_t *rag);

/* Ingest and chunk a multi-line note/document into FTS5 with line ranges and SHA-256 */
size_t nibras_rag_index_note(
    nibras_rag_t *rag,
    uint64_t      note_id,
    const char   *title,
    const char   *full_text
);

/* Index all active (non-pruned) memory nodes from a nibras_memory_store_t */
size_t nibras_rag_index_memory_store(
    nibras_rag_t                *rag,
    const nibras_memory_store_t *store
);

/* Retrieve top-k chunks with BM25 ranking and exact citations */
size_t nibras_rag_retrieve(
    nibras_rag_t        *rag,
    const char          *query,
    nibras_rag_result_t *out_results,
    size_t               max_results
);

/* Verify a factual claim against retrieved authoritative chunks using grounding shield */
grounding_result_t nibras_rag_verify_claim(
    nibras_rag_t            *rag,
    const grounding_claim_t *claim
);

/* High-level grounded QA: Answers meeting/decision queries strictly backed by citations; refuses to invent dates */
int nibras_rag_answer_meeting_query(
    nibras_rag_t        *rag,
    const char          *query,
    char                *out_answer,
    size_t               answer_cap,
    nibras_rag_result_t *out_citation,
    grounding_result_t  *out_grounding
);

#ifdef __cplusplus
}
#endif

#endif /* NIBRAS_RAG_H */
