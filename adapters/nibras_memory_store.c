/* nibras_memory_store.c - Sovereign Persistent Memory Store (JSONL)
 * Implementation of persistent cognitive memory graph for Nibras
 * Engine: Almaz Cognitive Organism | Zero Cloud Telemetry | C99 Hardened
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <time.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>

#include "nibras_memory_store.h"

/* Helper to escape strings into valid JSON */
static void json_escape(const char *src, char *dst, size_t dst_cap) {
    if (!src || !dst || dst_cap == 0) return;
    size_t d = 0;
    for (size_t s = 0; src[s] != '\0' && (d + 2) < dst_cap; ++s) {
        if (src[s] == '"' || src[s] == '\\') {
            dst[d++] = '\\';
            dst[d++] = src[s];
        } else if (src[s] == '\n') {
            dst[d++] = '\\';
            dst[d++] = 'n';
        } else if (src[s] == '\r') {
            dst[d++] = '\\';
            dst[d++] = 'r';
        } else if (src[s] == '\t') {
            dst[d++] = '\\';
            dst[d++] = 't';
        } else {
            dst[d++] = src[s];
        }
    }
    dst[d] = '\0';
}

/* Helper to unescape JSON strings */
static void json_unescape(const char *src, char *dst, size_t dst_cap) {
    if (!src || !dst || dst_cap == 0) return;
    size_t d = 0;
    for (size_t s = 0; src[s] != '\0' && (d + 1) < dst_cap; ++s) {
        if (src[s] == '\\' && src[s + 1] != '\0') {
            s++;
            if (src[s] == 'n') dst[d++] = '\n';
            else if (src[s] == 'r') dst[d++] = '\r';
            else if (src[s] == 't') dst[d++] = '\t';
            else dst[d++] = src[s];
        } else {
            dst[d++] = src[s];
        }
    }
    dst[d] = '\0';
}

/* Helper to parse a JSON string value by key */
static bool parse_json_string(const char *json, const char *key, char *out_val, size_t out_cap) {
    if (!json || !key || !out_val || out_cap == 0) return false;
    char pattern[64];
    snprintf(pattern, sizeof(pattern), "\"%s\"", key);

    const char *pos = strstr(json, pattern);
    if (!pos) return false;

    pos += strlen(pattern);
    while (*pos == ' ' || *pos == '\t' || *pos == ':') pos++;
    if (*pos != '"') return false;
    pos++; /* Skip opening quote */

    /* Read until closing quote */
    char raw_buf[512];
    size_t r = 0;
    bool escape = false;

    while (*pos != '\0' && (r + 1) < sizeof(raw_buf)) {
        if (!escape && *pos == '\\') {
            escape = true;
            raw_buf[r++] = *pos++;
            continue;
        }
        if (!escape && *pos == '"') {
            break;
        }
        escape = false;
        raw_buf[r++] = *pos++;
    }
    raw_buf[r] = '\0';
    json_unescape(raw_buf, out_val, out_cap);
    return true;
}

/* Helper to parse integer/float values from JSON */
static bool parse_json_value(const char *json, const char *key, char *out_val, size_t out_cap) {
    if (!json || !key || !out_val || out_cap == 0) return false;
    char pattern[64];
    snprintf(pattern, sizeof(pattern), "\"%s\"", key);

    const char *pos = strstr(json, pattern);
    if (!pos) return false;

    pos += strlen(pattern);
    while (*pos == ' ' || *pos == '\t' || *pos == ':') pos++;

    size_t i = 0;
    while (*pos != '\0' && *pos != ',' && *pos != '}' && *pos != ' ' && *pos != '\n' && *pos != '\r' && (i + 1) < out_cap) {
        out_val[i++] = *pos++;
    }
    out_val[i] = '\0';
    return (i > 0);
}

/* Helper to ensure parent directory exists */
static void ensure_parent_dir(const char *filepath) {
    if (!filepath) return;
    char tmp[256];
    strncpy(tmp, filepath, sizeof(tmp) - 1);
    tmp[sizeof(tmp) - 1] = '\0';

    char *last_slash = strrchr(tmp, '/');
    if (!last_slash) return;

    *last_slash = '\0';
    /* Simple mkdir */
    mkdir(tmp, 0755);
}

/* Case-insensitive substring match */
static bool strcasestr_match(const char *haystack, const char *needle) {
    if (!haystack || !needle) return false;
    if (needle[0] == '\0') return true;

    size_t needle_len = strlen(needle);
    for (; *haystack != '\0'; ++haystack) {
        if (tolower((unsigned char)*haystack) == tolower((unsigned char)*needle)) {
            size_t i = 1;
            for (; i < needle_len; ++i) {
                if (haystack[i] == '\0' ||
                    tolower((unsigned char)haystack[i]) != tolower((unsigned char)needle[i])) {
                    break;
                }
            }
            if (i == needle_len) return true;
        }
    }
    return false;
}

int nibras_store_init(nibras_memory_store_t *store, const char *filepath) {
    if (!store) return -1;
    memset(store, 0, sizeof(*store));
    store->next_id = 1;

    if (filepath && filepath[0] != '\0') {
        strncpy(store->filepath, filepath, sizeof(store->filepath) - 1);
        store->filepath[sizeof(store->filepath) - 1] = '\0';
        ensure_parent_dir(filepath);
        nibras_store_load(store, filepath);
    }
    return 0;
}

uint64_t nibras_store_add_node(nibras_memory_store_t *store, const nibras_memory_node_t *node) {
    if (!store || !node) return 0;
    if (store->count >= NIBRAS_MAX_MEMORY_NODES) return 0;

    nibras_memory_node_t *dst = &store->nodes[store->count];
    *dst = *node;

    if (dst->id == 0) {
        dst->id = store->next_id++;
    } else if (dst->id >= store->next_id) {
        store->next_id = dst->id + 1;
    }

    if (dst->created_utc == 0) {
        dst->created_utc = (uint64_t)time(NULL);
    }

    if (dst->is_pinned) {
        dst->state = TRAJ_DISTILLED_RULE;
        dst->current_intensity = 1.0;
    }

    store->count++;

    if (store->filepath[0] != '\0') {
        nibras_store_save(store, store->filepath);
    }

    return dst->id;
}

uint64_t nibras_store_ingest(
    nibras_memory_store_t   *store,
    const char              *raw_text,
    uint32_t                 token_cost,
    nibras_extracted_info_t *out_extracted_info
) {
    if (!store || !raw_text || raw_text[0] == '\0') return 0;

    /* 1. Normalize Arabic text in-place (zero heap allocation) */
    char norm_text[512];
    arabic_normalize_utf8_hardened(raw_text, norm_text, sizeof(norm_text));

    /* 2. Deterministic Arabic entity extraction */
    nibras_extracted_info_t info;
    nibras_extract_arabic_entities(norm_text, &info);
    if (out_extracted_info) {
        *out_extracted_info = info;
    }

    /* 3. Build memory node */
    nibras_memory_node_t node;
    memset(&node, 0, sizeof(node));

    node.created_utc = (uint64_t)time(NULL);
    node.initial_intensity = info.suggested_salience;
    node.current_intensity = info.suggested_salience;
    node.valence = info.suggested_valence;
    node.is_pinned = info.suggest_pin;
    node.state = info.suggest_pin ? TRAJ_DISTILLED_RULE : TRAJ_ACTIVE_EPISODIC;

    /* Modelled token cost calculation if 0 */
    if (token_cost == 0) {
        node.token_cost = (uint32_t)((strlen(raw_text) + 2) / 3) + 12;
    } else {
        node.token_cost = token_cost;
    }

    /* Assign category */
    if (info.is_appointment) {
        strncpy(node.category, "appointment", sizeof(node.category) - 1);
    } else if (info.is_executive_decision) {
        strncpy(node.category, "decision", sizeof(node.category) - 1);
    } else if (info.is_action_item) {
        strncpy(node.category, "action", sizeof(node.category) - 1);
    } else if (info.datetime_str[0] != '\0') {
        strncpy(node.category, "calendar", sizeof(node.category) - 1);
    } else {
        strncpy(node.category, "chatter", sizeof(node.category) - 1);
    }

    /* Generate structured summary */
    if (info.is_appointment && info.datetime_str[0] != '\0') {
        snprintf(node.summary, sizeof(node.summary), "%s (%s)", info.appointment_title, info.datetime_str);
    } else if (info.is_appointment) {
        snprintf(node.summary, sizeof(node.summary), "%s", info.appointment_title);
    } else if (info.is_executive_decision) {
        strncpy(node.summary, norm_text, sizeof(node.summary) - 1);
    } else if (info.is_action_item && info.datetime_str[0] != '\0') {
        snprintf(node.summary, sizeof(node.summary), "%s (%s)", info.action_verb, info.datetime_str);
    } else if (info.is_action_item) {
        snprintf(node.summary, sizeof(node.summary), "%s", info.action_verb);
    } else {
        strncpy(node.summary, norm_text, sizeof(node.summary) - 1);
    }
    node.summary[sizeof(node.summary) - 1] = '\0';

    strncpy(node.raw_arabic, raw_text, sizeof(node.raw_arabic) - 1);
    node.raw_arabic[sizeof(node.raw_arabic) - 1] = '\0';

    return nibras_store_add_node(store, &node);
}

nibras_memory_node_t *nibras_store_find_by_id(nibras_memory_store_t *store, uint64_t id) {
    if (!store || id == 0) return NULL;
    for (size_t i = 0; i < store->count; ++i) {
        if (store->nodes[i].id == id) {
            return &store->nodes[i];
        }
    }
    return NULL;
}

int nibras_store_set_pin(nibras_memory_store_t *store, uint64_t id, bool pin) {
    nibras_memory_node_t *node = nibras_store_find_by_id(store, id);
    if (!node) return -1;

    node->is_pinned = pin;
    if (pin) {
        node->state = TRAJ_DISTILLED_RULE;
        node->current_intensity = 1.0;
    } else {
        node->state = (node->current_intensity < 0.25) ? TRAJ_NOISE_PRUNED : TRAJ_ACTIVE_EPISODIC;
    }

    if (store->filepath[0] != '\0') {
        nibras_store_save(store, store->filepath);
    }
    return 0;
}

int nibras_store_invert_valence(nibras_memory_store_t *store, uint64_t id) {
    nibras_memory_node_t *node = nibras_store_find_by_id(store, id);
    if (!node) return -1;

    node->valence = -node->valence;
    node->valence_inverted = true;
    node->state = TRAJ_RECOVERY_VECTOR;

    if (store->filepath[0] != '\0') {
        nibras_store_save(store, store->filepath);
    }
    return 0;
}

void nibras_store_step_decay(
    nibras_memory_store_t       *store,
    uint64_t                     now_utc,
    double                       half_life_sec,
    double                       sim_time_scale,
    double                       consolidation_window_sec,
    nibras_telemetry_snapshot_t *out_snap
) {
    if (!store) return;
    nibras_telemetry_snapshot_t snap;
    memset(&snap, 0, sizeof(snap));

    nibras_fademem_step_hardened(
        store->nodes,
        store->count,
        now_utc,
        half_life_sec,
        sim_time_scale,
        consolidation_window_sec,
        &snap
    );

    if (out_snap) {
        *out_snap = snap;
    }

    if (store->filepath[0] != '\0') {
        nibras_store_save(store, store->filepath);
    }
}

void nibras_store_get_snapshot(
    const nibras_memory_store_t *store,
    nibras_telemetry_snapshot_t *out_snap
) {
    if (!store || !out_snap) return;
    memset(out_snap, 0, sizeof(*out_snap));

    double friction_sum = 0.0;
    size_t friction_count = 0;

    for (size_t i = 0; i < store->count; ++i) {
        const nibras_memory_node_t *n = &store->nodes[i];
        out_snap->total_raw_tokens += n->token_cost;

        if (n->state == TRAJ_DISTILLED_RULE || n->is_pinned) {
            out_snap->distilled_rules_count++;
            out_snap->active_tokens_after_prune += (n->token_cost > 0) ? ((n->token_cost + 5U) / 6U) : 0;
        } else if (n->state != TRAJ_NOISE_PRUNED) {
            out_snap->active_tokens_after_prune += n->token_cost;
        }

        if (n->valence < 0 && n->state != TRAJ_NOISE_PRUNED) {
            friction_sum += n->current_intensity;
            friction_count++;
        }
    }

    if (out_snap->total_raw_tokens > 0 &&
        out_snap->active_tokens_after_prune <= out_snap->total_raw_tokens) {
        uint64_t saved = (uint64_t)out_snap->total_raw_tokens - (uint64_t)out_snap->active_tokens_after_prune;
        out_snap->tokens_saved_pct = (uint32_t)((saved * 100ULL) / (uint64_t)out_snap->total_raw_tokens);
    }

    out_snap->cognitive_friction_index = (friction_count > 0)
                                       ? (friction_sum / (double)friction_count)
                                       : 0.0;
}

int nibras_store_save(const nibras_memory_store_t *store, const char *filepath) {
    if (!store || !filepath || filepath[0] == '\0') return -1;

    ensure_parent_dir(filepath);

    char tmp_path[300];
    snprintf(tmp_path, sizeof(tmp_path), "%s.tmp", filepath);

    FILE *f = fopen(tmp_path, "w");
    if (!f) return -1;

    char esc_summary[256];
    char esc_raw[512];
    char esc_cat[64];

    for (size_t i = 0; i < store->count; ++i) {
        const nibras_memory_node_t *n = &store->nodes[i];
        json_escape(n->summary, esc_summary, sizeof(esc_summary));
        json_escape(n->raw_arabic, esc_raw, sizeof(esc_raw));
        json_escape(n->category, esc_cat, sizeof(esc_cat));

        fprintf(f, "{\"id\":%llu,\"summary\":\"%s\",\"valence\":%d,\"initial_intensity\":%.4f,\"current_intensity\":%.4f,\"created_utc\":%llu,\"token_cost\":%u,\"valence_inverted\":%s,\"is_pinned\":%s,\"state\":%d,\"category\":\"%s\",\"raw_arabic\":\"%s\"}\n",
            (unsigned long long)n->id,
            esc_summary,
            n->valence,
            n->initial_intensity,
            n->current_intensity,
            (unsigned long long)n->created_utc,
            n->token_cost,
            n->valence_inverted ? "true" : "false",
            n->is_pinned ? "true" : "false",
            (int)n->state,
            esc_cat,
            esc_raw
        );
    }

    fclose(f);
    rename(tmp_path, filepath);
    return 0;
}

int nibras_store_load(nibras_memory_store_t *store, const char *filepath) {
    if (!store || !filepath || filepath[0] == '\0') return -1;

    FILE *f = fopen(filepath, "r");
    if (!f) return -1;

    store->count = 0;
    store->next_id = 1;

    char line[1024];
    while (fgets(line, sizeof(line), f) && store->count < NIBRAS_MAX_MEMORY_NODES) {
        if (line[0] != '{') continue;

        nibras_memory_node_t node;
        memset(&node, 0, sizeof(node));

        char val_buf[64];

        if (parse_json_value(line, "id", val_buf, sizeof(val_buf))) {
            node.id = (uint64_t)strtoull(val_buf, NULL, 10);
        }
        parse_json_string(line, "summary", node.summary, sizeof(node.summary));

        if (parse_json_value(line, "valence", val_buf, sizeof(val_buf))) {
            node.valence = (int32_t)strtol(val_buf, NULL, 10);
        }
        if (parse_json_value(line, "initial_intensity", val_buf, sizeof(val_buf))) {
            node.initial_intensity = strtod(val_buf, NULL);
        }
        if (parse_json_value(line, "current_intensity", val_buf, sizeof(val_buf))) {
            node.current_intensity = strtod(val_buf, NULL);
        }
        if (parse_json_value(line, "created_utc", val_buf, sizeof(val_buf))) {
            node.created_utc = (uint64_t)strtoull(val_buf, NULL, 10);
        }
        if (parse_json_value(line, "token_cost", val_buf, sizeof(val_buf))) {
            node.token_cost = (uint32_t)strtoul(val_buf, NULL, 10);
        }
        if (parse_json_value(line, "valence_inverted", val_buf, sizeof(val_buf))) {
            node.valence_inverted = (strcmp(val_buf, "true") == 0 || strcmp(val_buf, "1") == 0);
        }
        if (parse_json_value(line, "is_pinned", val_buf, sizeof(val_buf))) {
            node.is_pinned = (strcmp(val_buf, "true") == 0 || strcmp(val_buf, "1") == 0);
        }
        if (parse_json_value(line, "state", val_buf, sizeof(val_buf))) {
            node.state = (nibras_traj_state_t)strtol(val_buf, NULL, 10);
        }
        parse_json_string(line, "category", node.category, sizeof(node.category));
        parse_json_string(line, "raw_arabic", node.raw_arabic, sizeof(node.raw_arabic));

        if (node.id >= store->next_id) {
            store->next_id = node.id + 1;
        }

        store->nodes[store->count++] = node;
    }

    fclose(f);
    return 0;
}

size_t nibras_store_search(
    const nibras_memory_store_t *store,
    const char                  *query,
    nibras_memory_node_t        *out_matches,
    size_t                       max_matches
) {
    if (!store || !query || !out_matches || max_matches == 0) return 0;

    char norm_query[256];
    arabic_normalize_utf8_hardened(query, norm_query, sizeof(norm_query));

    size_t matches = 0;
    for (size_t i = 0; i < store->count && matches < max_matches; ++i) {
        const nibras_memory_node_t *n = &store->nodes[i];
        if (strcasestr_match(n->summary, query) ||
            strcasestr_match(n->summary, norm_query) ||
            strcasestr_match(n->raw_arabic, query) ||
            strcasestr_match(n->raw_arabic, norm_query) ||
            strcasestr_match(n->category, query)) {
            out_matches[matches++] = *n;
        }
    }
    return matches;
}
