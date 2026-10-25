/* nibras_memory_store.h - Sovereign Persistent Memory Store (JSONL)
 * Target: Best Personal AI Agent | Dubai Create AI Agents Championship 2026-2027
 * Engine: Almaz Cognitive Organism | UAE PDPL Law No. 45 of 2021 Compliant
 */
#ifndef NIBRAS_MEMORY_STORE_H
#define NIBRAS_MEMORY_STORE_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include "nibras_adapters.h"

#ifdef __cplusplus
extern "C" {
#endif

#define NIBRAS_MAX_MEMORY_NODES 1024

typedef struct {
    nibras_memory_node_t nodes[NIBRAS_MAX_MEMORY_NODES];
    size_t               count;
    char                 filepath[256];
    uint64_t             next_id;
} nibras_memory_store_t;

/* Initialize store from JSONL file (creates directory if necessary, loads existing nodes) */
int nibras_store_init(nibras_memory_store_t *store, const char *filepath);

/* Ingest raw task/note: normalizes Arabic, extracts entities, sets salience, pins if needed, persists */
uint64_t nibras_store_ingest(
    nibras_memory_store_t   *store,
    const char              *raw_text,
    uint32_t                 token_cost,
    nibras_extracted_info_t *out_extracted_info
);

/* Add a pre-constructed node to memory store (auto-assigns ID and timestamps if 0) */
uint64_t nibras_store_add_node(nibras_memory_store_t *store, const nibras_memory_node_t *node);

/* Find node by ID */
nibras_memory_node_t *nibras_store_find_by_id(nibras_memory_store_t *store, uint64_t id);

/* Pin / Unpin a memory node (pinned nodes stay at intensity 1.0) */
int nibras_store_set_pin(nibras_memory_store_t *store, uint64_t id, bool pin);

/* Invert valence for friction recovery vector */
int nibras_store_invert_valence(nibras_memory_store_t *store, uint64_t id);

/* Advance time and step FadeMem decay across all nodes */
void nibras_store_step_decay(
    nibras_memory_store_t       *store,
    uint64_t                     now_utc,
    double                       half_life_sec,
    double                       sim_time_scale,
    double                       consolidation_window_sec,
    nibras_telemetry_snapshot_t *out_snap
);

/* Save memory store to JSONL */
int nibras_store_save(const nibras_memory_store_t *store, const char *filepath);

/* Load memory store from JSONL */
int nibras_store_load(nibras_memory_store_t *store, const char *filepath);

/* Search memory store by text (case-insensitive substring match in summary or raw_arabic) */
size_t nibras_store_search(
    const nibras_memory_store_t *store,
    const char                  *query,
    nibras_memory_node_t        *out_matches,
    size_t                       max_matches
);

/* Compute current telemetry snapshot without advancing time */
void nibras_store_get_snapshot(
    const nibras_memory_store_t *store,
    nibras_telemetry_snapshot_t *out_snap
);

#ifdef __cplusplus
}
#endif

#endif /* NIBRAS_MEMORY_STORE_H */
