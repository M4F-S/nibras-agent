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
#include <math.h>
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

/* Helper to ensure NUL-terminated copy */
static void nib_strlcpy(char *dst, const char *src, size_t cap) {
    if (!dst || cap == 0) return;
    size_t n = src ? strlen(src) : 0;
    if (n >= cap) n = cap - 1;
    if (n) memcpy(dst, src, n);
    dst[n] = '\0';
}

/* longest prefix of s, at most cap bytes, that does not split a UTF-8 sequence */
static size_t nib_utf8_prefix(const char *s, size_t cap) {
    if (!s || cap == 0) return 0;
    size_t n = 0;
    while (n < cap && s[n] != '\0') n++;
    while (n > 0 && ((unsigned char)s[n] & 0xC0) == 0x80) n--;
    return n;
}

/* "title (datetime)" into dst, truncating on codepoint boundaries */
static void nib_summary(char *dst, size_t cap, const char *title, const char *dt) {
    if (!dst || cap == 0) return;
    size_t tl = nib_utf8_prefix(title ? title : "", cap > 4 ? 64 : 0);
    size_t dl = nib_utf8_prefix(dt ? dt : "", 48);
    if (dl && tl + 3 + dl >= cap) dl = nib_utf8_prefix(dt ? dt : "", (tl + 3 < cap) ? cap - tl - 4 : 0);
    size_t w = 0;
    if (tl) { memcpy(dst, title, tl); w = tl; }
    if (dl && w + 3 < cap) {
        memcpy(dst + w, " (", 2); w += 2;
        memcpy(dst + w, dt, dl); w += dl; dst[w++] = ')';
    }
    dst[w] = '\0';
}

/* Helper to ensure parent directory exists */
static void ensure_parent_dir(const char *filepath) {
    if (!filepath) return;
    char tmp[256];
    nib_strlcpy(tmp, filepath, sizeof(tmp));

    char *last_slash = strrchr(tmp, '/');
    if (!last_slash) return;

    *last_slash = '\0';
    /* Simple mkdir */
    mkdir(tmp, 0755);
}

/* Character delimiter check for BM25 tokenization */
static bool is_token_delim(unsigned char c) {
    if (c <= ' ') return true;
    if (c >= 0x80) return false; /* UTF-8 lead or continuation byte */
    switch (c) {
        case ',': case '.': case ';': case ':': case '!': case '?':
        case '-': case '_': case '(': case ')': case '[': case ']':
        case '{': case '}': case '"': case '\'': case '/': case '\\':
        case '|': case '#': case '@': case '+': case '=': case '*':
        case '&': case '%': case '^': case '~': case '`': case '<': case '>':
            return true;
        default:
            return false;
    }
}

/* Light stemming for Arabic definite article "الـ" */
static void light_stem_arabic(char *token, size_t *inout_len) {
    if (!token || !inout_len || *inout_len < 8) return;
    unsigned char *u = (unsigned char *)token;
    /* Check for "ال" prefix (0xD8 0xA7 0xD9 0x84) */
    if (u[0] == 0xD8 && u[1] == 0xA7 && u[2] == 0xD9 && u[3] == 0x84) {
        size_t rem = *inout_len - 4;
        memmove(token, token + 4, rem);
        token[rem] = '\0';
        *inout_len = rem;
    }
}

/* Helper to insert a token into unique tokens list */
static void add_unique_token(
    const char *tok,
    char        out_tokens[NIBRAS_MAX_DOC_TOKENS][NIBRAS_MAX_TOKEN_LEN],
    uint16_t    out_freqs[NIBRAS_MAX_DOC_TOKENS],
    size_t     *inout_unique_count
) {
    if (!tok || tok[0] == '\0' || !inout_unique_count) return;
    for (size_t i = 0; i < *inout_unique_count; ++i) {
        if (strcmp(out_tokens[i], tok) == 0) {
            if (out_freqs) out_freqs[i]++;
            return;
        }
    }
    if (*inout_unique_count < NIBRAS_MAX_DOC_TOKENS) {
        nib_strlcpy(out_tokens[*inout_unique_count], tok, NIBRAS_MAX_TOKEN_LEN);
        if (out_freqs) out_freqs[*inout_unique_count] = 1;
        (*inout_unique_count)++;
    }
}

/* Tokenize text into normalized tokens with frequencies and total word count */
static size_t nibras_tokenize_text(
    const char *src,
    char        out_tokens[NIBRAS_MAX_DOC_TOKENS][NIBRAS_MAX_TOKEN_LEN],
    uint16_t    out_freqs[NIBRAS_MAX_DOC_TOKENS],
    uint16_t   *out_total_words
) {
    if (!src || !out_tokens) return 0;

    /* Normalize with UTF-8 safety */
    char safe_src[512];
    size_t safe_src_len = nib_utf8_prefix(src, sizeof(safe_src) - 1);
    memcpy(safe_src, src, safe_src_len);
    safe_src[safe_src_len] = '\0';

    char norm_buf[512];
    arabic_normalize_utf8_hardened(safe_src, norm_buf, sizeof(norm_buf));

    size_t unique_count = 0;
    uint16_t total_words = 0;
    const unsigned char *p = (const unsigned char *)norm_buf;

    while (*p != '\0') {
        while (*p != '\0' && is_token_delim(*p)) {
            p++;
        }
        if (*p == '\0') break;

        char token[NIBRAS_MAX_TOKEN_LEN];
        size_t t_idx = 0;

        while (*p != '\0' && !is_token_delim(*p)) {
            if (*p < 0x80) {
                if (t_idx + 1 < NIBRAS_MAX_TOKEN_LEN) {
                    token[t_idx++] = (char)tolower(*p);
                }
                p++;
            } else {
                size_t cp_len = 1;
                if ((*p & 0xE0) == 0xC0) cp_len = 2;
                else if ((*p & 0xF0) == 0xE0) cp_len = 3;
                else if ((*p & 0xF8) == 0xF0) cp_len = 4;

                if (t_idx + cp_len < NIBRAS_MAX_TOKEN_LEN) {
                    for (size_t k = 0; k < cp_len && p[k] != '\0'; ++k) {
                        token[t_idx++] = (char)p[k];
                    }
                }
                p += cp_len;
            }
        }

        size_t safe_len = nib_utf8_prefix(token, t_idx);
        token[safe_len] = '\0';

        if (safe_len > 0) {
            total_words++;
            add_unique_token(token, out_tokens, out_freqs, &unique_count);

            /* Also index light-stemmed form if different (e.g. "الترخيص" -> "ترخيص") */
            char stemmed[NIBRAS_MAX_TOKEN_LEN];
            nib_strlcpy(stemmed, token, sizeof(stemmed));
            size_t stem_len = safe_len;
            light_stem_arabic(stemmed, &stem_len);
            if (stem_len != safe_len && stem_len > 0) {
                add_unique_token(stemmed, out_tokens, out_freqs, &unique_count);
            }
        }
    }

    if (out_total_words) {
        *out_total_words = total_words;
    }
    return unique_count;
}

/* Index a single document into the in-memory index */
static void nibras_store_index_node_internal(nibras_memory_store_t *store, size_t doc_idx) {
    if (!store || doc_idx >= store->count) return;

    const nibras_memory_node_t *node = &store->nodes[doc_idx];
    nibras_doc_index_t *doc_idx_ptr = &store->index[doc_idx];
    memset(doc_idx_ptr, 0, sizeof(*doc_idx_ptr));

    char full_text[512];
    snprintf(full_text, sizeof(full_text), "%s %s %s", node->summary, node->raw_arabic, node->category);

    char temp_tokens[NIBRAS_MAX_DOC_TOKENS][NIBRAS_MAX_TOKEN_LEN];
    uint16_t temp_freqs[NIBRAS_MAX_DOC_TOKENS];
    uint16_t total_w = 0;

    size_t u_cnt = nibras_tokenize_text(full_text, temp_tokens, temp_freqs, &total_w);

    doc_idx_ptr->term_count = (uint16_t)u_cnt;
    doc_idx_ptr->total_words = total_w > 0 ? total_w : 1;

    for (size_t i = 0; i < u_cnt; ++i) {
        nib_strlcpy(doc_idx_ptr->terms[i].term, temp_tokens[i], NIBRAS_MAX_TOKEN_LEN);
        doc_idx_ptr->terms[i].freq = temp_freqs[i];
    }
}

void nibras_store_build_index(nibras_memory_store_t *store) {
    if (!store) return;
    for (size_t i = 0; i < store->count; ++i) {
        nibras_store_index_node_internal(store, i);
    }
}

int nibras_store_init(nibras_memory_store_t *store, const char *filepath) {
    if (!store) return -1;
    memset(store, 0, sizeof(*store));
    store->next_id = 1;

    if (filepath && filepath[0] != '\0') {
        nib_strlcpy(store->filepath, filepath, sizeof(store->filepath));
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
    nibras_store_index_node_internal(store, store->count - 1);

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
        nib_strlcpy(node.category, "appointment", sizeof(node.category));
    } else if (info.is_executive_decision) {
        nib_strlcpy(node.category, "decision", sizeof(node.category));
    } else if (info.is_action_item) {
        nib_strlcpy(node.category, "action", sizeof(node.category));
    } else if (info.datetime_str[0] != '\0') {
        nib_strlcpy(node.category, "calendar", sizeof(node.category));
    } else {
        nib_strlcpy(node.category, "chatter", sizeof(node.category));
    }

    /* Generate structured summary without mid-codepoint UTF-8 slicing */
    if (info.is_appointment && info.datetime_str[0] != '\0') {
        nib_summary(node.summary, sizeof(node.summary), info.appointment_title, info.datetime_str);
    } else if (info.is_appointment) {
        nib_summary(node.summary, sizeof(node.summary), info.appointment_title, NULL);
    } else if (info.is_executive_decision) {
        nib_summary(node.summary, sizeof(node.summary), norm_text, NULL);
    } else if (info.is_action_item && info.datetime_str[0] != '\0') {
        nib_summary(node.summary, sizeof(node.summary), info.action_verb, info.datetime_str);
    } else if (info.is_action_item) {
        nib_summary(node.summary, sizeof(node.summary), info.action_verb, NULL);
    } else {
        nib_summary(node.summary, sizeof(node.summary), norm_text, NULL);
    }

    /* Safe UTF-8 copy for raw text */
    size_t raw_len = nib_utf8_prefix(raw_text, sizeof(node.raw_arabic) - 1);
    if (raw_len) memcpy(node.raw_arabic, raw_text, raw_len);
    node.raw_arabic[raw_len] = '\0';

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
    nibras_store_build_index(store);
    return 0;
}

typedef struct {
    size_t doc_idx;
    double score;
    char   match_reason[160];
} nibras_bm25_candidate_t;

size_t nibras_store_search_ranked(
    const nibras_memory_store_t *store,
    const char                  *query,
    nibras_search_result_t      *out_results,
    size_t                       max_results
) {
    if (!store || !query || !out_results || max_results == 0 || store->count == 0) {
        return 0;
    }

    /* Tokenize query with UTF-8 prefix safety */
    char q_tokens[NIBRAS_MAX_DOC_TOKENS][NIBRAS_MAX_TOKEN_LEN];
    uint16_t q_freqs[NIBRAS_MAX_DOC_TOKENS];
    uint16_t q_total_words = 0;
    size_t q_term_count = nibras_tokenize_text(query, q_tokens, q_freqs, &q_total_words);

    if (q_term_count == 0) return 0;

    /* Compute collection statistics among active (non-pruned) documents */
    size_t N_active = 0;
    double total_len = 0.0;

    for (size_t i = 0; i < store->count; ++i) {
        if (store->nodes[i].state != TRAJ_NOISE_PRUNED) {
            N_active++;
            total_len += (double)store->index[i].total_words;
        }
    }

    if (N_active == 0) return 0;
    double avgdl = total_len / (double)N_active;
    if (avgdl < 1.0) avgdl = 1.0;

    /* Compute IDF for each query term across active collection */
    double q_idf[NIBRAS_MAX_DOC_TOKENS];
    size_t q_df[NIBRAS_MAX_DOC_TOKENS];

    for (size_t j = 0; j < q_term_count; ++j) {
        size_t df = 0;
        for (size_t i = 0; i < store->count; ++i) {
            if (store->nodes[i].state == TRAJ_NOISE_PRUNED) continue;

            const nibras_doc_index_t *didx = &store->index[i];
            for (size_t t = 0; t < didx->term_count; ++t) {
                if (strcmp(didx->terms[t].term, q_tokens[j]) == 0) {
                    df++;
                    break;
                }
            }
        }
        q_df[j] = df;
        if (df > 0) {
            /* Standard non-negative BM25 IDF: ln((N - df + 0.5)/(df + 0.5) + 1.0) */
            q_idf[j] = log(((double)(N_active - df) + 0.5) / ((double)df + 0.5) + 1.0);
            if (q_idf[j] < 0.0) q_idf[j] = 0.0;
        } else {
            q_idf[j] = 0.0;
        }
    }

    /* Score active documents using BM25 */
    const double k1 = 1.2;
    const double b = 0.75;

    nibras_bm25_candidate_t candidates[NIBRAS_MAX_MEMORY_NODES];
    size_t candidate_count = 0;

    for (size_t i = 0; i < store->count; ++i) {
        /* Filter out decayed / noise-pruned nodes — rank strictly after decay */
        if (store->nodes[i].state == TRAJ_NOISE_PRUNED) continue;

        const nibras_doc_index_t *didx = &store->index[i];
        double doc_len = (double)didx->total_words;
        double doc_score = 0.0;
        char reason_buf[160] = {0};
        size_t reason_w = 0;
        size_t match_terms = 0;

        for (size_t j = 0; j < q_term_count; ++j) {
            if (q_df[j] == 0) continue;

            uint16_t tf = 0;
            for (size_t t = 0; t < didx->term_count; ++t) {
                if (strcmp(didx->terms[t].term, q_tokens[j]) == 0) {
                    tf = didx->terms[t].freq;
                    break;
                }
            }

            if (tf > 0) {
                double tf_comp = ((double)tf * (k1 + 1.0)) /
                                 ((double)tf + k1 * (1.0 - b + b * (doc_len / avgdl)));
                double term_score = q_idf[j] * tf_comp;
                doc_score += term_score;
                match_terms++;

                int n = snprintf(reason_buf + reason_w, sizeof(reason_buf) - reason_w,
                                 "%s'%s' (tf=%u, idf=%.2f)",
                                 (match_terms > 1 ? ", " : "matched: "),
                                 q_tokens[j], (unsigned int)tf, q_idf[j]);
                if (n > 0 && (size_t)n < sizeof(reason_buf) - reason_w) {
                    reason_w += (size_t)n;
                }
            }
        }

        if (doc_score > 0.0) {
            candidates[candidate_count].doc_idx = i;
            candidates[candidate_count].score = doc_score;
            nib_strlcpy(candidates[candidate_count].match_reason, reason_buf, sizeof(candidates[candidate_count].match_reason));
            candidate_count++;
        }
    }

    if (candidate_count == 0) return 0;

    /* Deterministic sorting descending by BM25 score with stable tie-breaks */
    for (size_t i = 0; i < candidate_count - 1; ++i) {
        for (size_t j = i + 1; j < candidate_count; ++j) {
            bool swap = false;
            if (candidates[j].score > candidates[i].score + 1e-6) {
                swap = true;
            } else if (fabs(candidates[j].score - candidates[i].score) <= 1e-6) {
                size_t idx_i = candidates[i].doc_idx;
                size_t idx_j = candidates[j].doc_idx;
                if (store->nodes[idx_j].is_pinned && !store->nodes[idx_i].is_pinned) {
                    swap = true;
                } else if (store->nodes[idx_j].current_intensity > store->nodes[idx_i].current_intensity + 1e-6) {
                    swap = true;
                } else if (store->nodes[idx_j].created_utc > store->nodes[idx_i].created_utc) {
                    swap = true;
                } else if (store->nodes[idx_j].id < store->nodes[idx_i].id) {
                    swap = true;
                }
            }
            if (swap) {
                nibras_bm25_candidate_t tmp = candidates[i];
                candidates[i] = candidates[j];
                candidates[j] = tmp;
            }
        }
    }

    /* Populate top-k output results */
    size_t return_count = candidate_count < max_results ? candidate_count : max_results;
    for (size_t k = 0; k < return_count; ++k) {
        size_t d_idx = candidates[k].doc_idx;
        out_results[k].node = store->nodes[d_idx];
        out_results[k].score = candidates[k].score;
        nib_strlcpy(out_results[k].match_reason, candidates[k].match_reason, sizeof(out_results[k].match_reason));
    }

    return return_count;
}

size_t nibras_store_search(
    const nibras_memory_store_t *store,
    const char                  *query,
    nibras_memory_node_t        *out_matches,
    size_t                       max_matches
) {
    if (!store || !query || !out_matches || max_matches == 0) return 0;

    nibras_search_result_t results[NIBRAS_MAX_MEMORY_NODES];
    size_t count = nibras_store_search_ranked(store, query, results, max_matches);
    for (size_t i = 0; i < count; ++i) {
        out_matches[i] = results[i].node;
    }
    return count;
}
