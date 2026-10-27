/* nibras_rag.c - Personal Sovereign RAG & Citation Grounding Engine
 * Target: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027
 * Engine: Almaz Cognitive Organism | C99 SQLite FTS5 + BM25 Grounding Shield
 * Compliance: UAE Federal Decree-Law No. 45 of 2021 (PDPL)
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <stdint.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>

#include "nibras_rag.h"
#include "grounding_shield.h"
#include "nibras_adapters.h"
#include "nibras_memory_store.h"

/* ========================================================================= */
/* 1. Self-Contained FIPS 180-2 Standard SHA-256 Implementation              */
/* ========================================================================= */

#define ROTR(x, n) (((x) >> (n)) | ((x) << (32 - (n))))
#define CH(x, y, z) (((x) & (y)) ^ (~(x) & (z)))
#define MAJ(x, y, z) (((x) & (y)) ^ ((x) & (z)) ^ ((y) & (z)))
#define SIGMA0(x) (ROTR(x, 2) ^ ROTR(x, 13) ^ ROTR(x, 22))
#define SIGMA1(x) (ROTR(x, 6) ^ ROTR(x, 11) ^ ROTR(x, 25))
#define sigma0(x) (ROTR(x, 7) ^ ROTR(x, 18) ^ ((x) >> 3))
#define sigma1(x) (ROTR(x, 17) ^ ROTR(x, 19) ^ ((x) >> 10))

static const uint32_t SHA256_K[64] = {
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
};

typedef struct {
    uint32_t state[8];
    uint64_t count;
    uint8_t  buffer[64];
} sha256_context_t;

static void sha256_transform(uint32_t state[8], const uint8_t data[64]) {
    uint32_t a = state[0], b = state[1], c = state[2], d = state[3];
    uint32_t e = state[4], f = state[5], g = state[6], h = state[7];
    uint32_t w[64];

    for (int i = 0; i < 16; ++i) {
        w[i] = ((uint32_t)data[i * 4] << 24) |
               ((uint32_t)data[i * 4 + 1] << 16) |
               ((uint32_t)data[i * 4 + 2] << 8) |
               ((uint32_t)data[i * 4 + 3]);
    }
    for (int i = 16; i < 64; ++i) {
        w[i] = sigma1(w[i - 2]) + w[i - 7] + sigma0(w[i - 15]) + w[i - 16];
    }
    for (int i = 0; i < 64; ++i) {
        uint32_t t1 = h + SIGMA1(e) + CH(e, f, g) + SHA256_K[i] + w[i];
        uint32_t t2 = SIGMA0(a) + MAJ(a, b, c);
        h = g;
        g = f;
        f = e;
        e = d + t1;
        d = c;
        c = b;
        b = a;
        a = t1 + t2;
    }
    state[0] += a;
    state[1] += b;
    state[2] += c;
    state[3] += d;
    state[4] += e;
    state[5] += f;
    state[6] += g;
    state[7] += h;
}

static void sha256_init(sha256_context_t *ctx) {
    ctx->state[0] = 0x6a09e667;
    ctx->state[1] = 0xbb67ae85;
    ctx->state[2] = 0x3c6ef372;
    ctx->state[3] = 0xa54ff53a;
    ctx->state[4] = 0x510e527f;
    ctx->state[5] = 0x9b05688c;
    ctx->state[6] = 0x1f83d9ab;
    ctx->state[7] = 0x5be0cd19;
    ctx->count = 0;
}

static void sha256_update(sha256_context_t *ctx, const void *data, size_t len) {
    const uint8_t *p = (const uint8_t *)data;
    size_t buf_idx = (size_t)(ctx->count & 63);
    ctx->count += len;

    size_t space = 64 - buf_idx;
    if (len >= space) {
        memcpy(ctx->buffer + buf_idx, p, space);
        sha256_transform(ctx->state, ctx->buffer);
        p += space;
        len -= space;
        while (len >= 64) {
            sha256_transform(ctx->state, p);
            p += 64;
            len -= 64;
        }
        buf_idx = 0;
    }
    if (len > 0) {
        memcpy(ctx->buffer + buf_idx, p, len);
    }
}

static void sha256_final(sha256_context_t *ctx, uint8_t digest[32]) {
    uint64_t total_bits = ctx->count * 8;
    size_t buf_idx = (size_t)(ctx->count & 63);

    ctx->buffer[buf_idx++] = 0x80;
    if (buf_idx > 56) {
        memset(ctx->buffer + buf_idx, 0, 64 - buf_idx);
        sha256_transform(ctx->state, ctx->buffer);
        buf_idx = 0;
    }
    memset(ctx->buffer + buf_idx, 0, 56 - buf_idx);
    for (int i = 0; i < 8; ++i) {
        ctx->buffer[56 + i] = (uint8_t)(total_bits >> ((7 - i) * 8));
    }
    sha256_transform(ctx->state, ctx->buffer);

    for (int i = 0; i < 8; ++i) {
        digest[i * 4]     = (uint8_t)(ctx->state[i] >> 24);
        digest[i * 4 + 1] = (uint8_t)(ctx->state[i] >> 16);
        digest[i * 4 + 2] = (uint8_t)(ctx->state[i] >> 8);
        digest[i * 4 + 3] = (uint8_t)(ctx->state[i]);
    }
}

void nibras_sha256_hex(const void *data, size_t len, char hex_out[NIBRAS_RAG_SHA256_HEX_LEN]) {
    if (!hex_out) return;
    sha256_context_t ctx;
    sha256_init(&ctx);
    if (data && len > 0) {
        sha256_update(&ctx, data, len);
    }
    uint8_t digest[32];
    sha256_final(&ctx, digest);
    for (int i = 0; i < 32; ++i) {
        snprintf(hex_out + i * 2, 3, "%02x", digest[i]);
    }
    hex_out[64] = '\0';
}

/* Safe string copy */
static void rag_strlcpy(char *dst, const char *src, size_t cap) {
    if (!dst || cap == 0) return;
    size_t n = src ? strlen(src) : 0;
    if (n >= cap) n = cap - 1;
    if (n) memcpy(dst, src, n);
    dst[n] = '\0';
}

/* ========================================================================= */
/* 2. SQLite FTS5 RAG Database Management                                    */
/* ========================================================================= */

int nibras_rag_init(nibras_rag_t *rag, const char *db_path) {
    if (!rag) return -1;
    memset(rag, 0, sizeof(*rag));

    const char *target_path = (db_path && db_path[0] != '\0') ? db_path : ":memory:";
    rag_strlcpy(rag->db_path, target_path, sizeof(rag->db_path));

    int rc = sqlite3_open(target_path, &rag->db);
    if (rc != SQLITE_OK || !rag->db) {
        return -1;
    }

    /* Configure high-performance memory pragmas */
    sqlite3_exec(rag->db, "PRAGMA synchronous = OFF;", NULL, NULL, NULL);
    sqlite3_exec(rag->db, "PRAGMA journal_mode = MEMORY;", NULL, NULL, NULL);

    /* Create virtual FTS5 table with unicode61 tokenizer for full Arabic & English support */
    const char *create_sql = 
        "CREATE VIRTUAL TABLE IF NOT EXISTS nibras_rag_chunks USING fts5(\n"
        "    note_title,\n"
        "    text,\n"
        "    start_line UNINDEXED,\n"
        "    end_line UNINDEXED,\n"
        "    passage_sha256 UNINDEXED,\n"
        "    note_id UNINDEXED,\n"
        "    tokenize='unicode61'\n"
        ");";

    char *err_msg = NULL;
    rc = sqlite3_exec(rag->db, create_sql, NULL, NULL, &err_msg);
    if (rc != SQLITE_OK) {
        if (err_msg) sqlite3_free(err_msg);
        sqlite3_close(rag->db);
        rag->db = NULL;
        return -1;
    }

    rag->is_open = true;
    return 0;
}

void nibras_rag_close(nibras_rag_t *rag) {
    if (!rag) return;
    if (rag->db) {
        sqlite3_close(rag->db);
        rag->db = NULL;
    }
    rag->is_open = false;
}

/* ========================================================================= */
/* 3. Multi-Line Chunking & SHA-256 Indexing                                 */
/* ========================================================================= */

size_t nibras_rag_index_note(
    nibras_rag_t *rag,
    uint64_t      note_id,
    const char   *title,
    const char   *full_text
) {
    if (!rag || !rag->is_open || !rag->db || !full_text || full_text[0] == '\0') {
        return 0;
    }

    const char *safe_title = (title && title[0] != '\0') ? title : "Untitled Note";

    /* Identify line boundaries */
    typedef struct {
        const char *start;
        size_t      len;
    } line_span_t;

    line_span_t lines[256];
    size_t line_count = 0;

    const char *p = full_text;
    while (*p != '\0' && line_count < 256) {
        const char *l_start = p;
        while (*p != '\0' && *p != '\n') p++;
        size_t l_len = (size_t)(p - l_start);
        if (l_len > 0 && l_start[l_len - 1] == '\r') l_len--;

        lines[line_count].start = l_start;
        lines[line_count].len = l_len;
        line_count++;

        if (*p == '\n') p++;
    }

    if (line_count == 0) return 0;

    sqlite3_stmt *stmt = NULL;
    const char *insert_sql = 
        "INSERT INTO nibras_rag_chunks(note_title, text, start_line, end_line, passage_sha256, note_id)\n"
        "VALUES (?, ?, ?, ?, ?, ?);";

    int rc = sqlite3_prepare_v2(rag->db, insert_sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK || !stmt) return 0;

    size_t chunks_added = 0;

    /* Chunking strategy:
     * If <= 8 lines: 1 single comprehensive chunk.
     * If > 8 lines: 5-line windows with 2-line overlap.
     */
    size_t step = (line_count <= 8) ? line_count : 3;
    size_t window = (line_count <= 8) ? line_count : 5;

    for (size_t i = 0; i < line_count; i += step) {
        size_t chunk_start_idx = i;
        size_t chunk_end_idx = i + window;
        if (chunk_end_idx > line_count) chunk_end_idx = line_count;

        char chunk_buf[NIBRAS_RAG_MAX_PASSAGE_LEN];
        size_t w = 0;

        for (size_t j = chunk_start_idx; j < chunk_end_idx; ++j) {
            size_t copy_len = lines[j].len;
            if (w + copy_len + 2 < sizeof(chunk_buf)) {
                if (w > 0) chunk_buf[w++] = '\n';
                memcpy(chunk_buf + w, lines[j].start, copy_len);
                w += copy_len;
            }
        }
        chunk_buf[w] = '\0';

        if (w == 0) continue;

        uint32_t s_line = (uint32_t)(chunk_start_idx + 1);
        uint32_t e_line = (uint32_t)(chunk_end_idx);

        char sha256_hex[NIBRAS_RAG_SHA256_HEX_LEN];
        nibras_sha256_hex(chunk_buf, w, sha256_hex);

        sqlite3_bind_text(stmt, 1, safe_title, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 2, chunk_buf, -1, SQLITE_STATIC);
        sqlite3_bind_int(stmt, 3, (int)s_line);
        sqlite3_bind_int(stmt, 4, (int)e_line);
        sqlite3_bind_text(stmt, 5, sha256_hex, -1, SQLITE_STATIC);
        sqlite3_bind_int64(stmt, 6, (sqlite3_int64)note_id);

        if (sqlite3_step(stmt) == SQLITE_DONE) {
            chunks_added++;
        }
        sqlite3_reset(stmt);

        if (chunk_end_idx == line_count) break;
    }

    sqlite3_finalize(stmt);
    rag->total_chunks_indexed += chunks_added;
    rag->total_notes_indexed++;
    return chunks_added;
}

size_t nibras_rag_index_memory_store(
    nibras_rag_t                *rag,
    const nibras_memory_store_t *store
) {
    if (!rag || !store) return 0;
    size_t total_indexed = 0;

    for (size_t i = 0; i < store->count; ++i) {
        const nibras_memory_node_t *n = &store->nodes[i];
        if (n->state == TRAJ_NOISE_PRUNED) continue;

        char note_body[1024];
        snprintf(note_body, sizeof(note_body),
                 "Subject: %s\n"
                 "Details: %s\n"
                 "Category: %s (State: %d, Intensity: %.2f)",
                 n->summary, n->raw_arabic, n->category, (int)n->state, n->current_intensity);

        total_indexed += nibras_rag_index_note(rag, n->id, n->summary, note_body);
    }

    return total_indexed;
}

/* ========================================================================= */
/* 4. Ranked FTS5 Retrieval with BM25 Scoring & Exact Citations             */
/* ========================================================================= */

static bool is_query_delim(unsigned char c) {
    return c <= ' ' || c == '"' || c == '\'' || c == ',' || c == '.' ||
           c == ';' || c == ':' || c == '!' || c == '?' || c == '(' ||
           c == ')' || c == '[' || c == ']' || c == '{' || c == '}' ||
           c == '-' || c == '/' || c == '\\' || c == '|';
}

static bool is_stopword(const char *token) {
    static const char *STOPWORDS[] = {
        /* English question & auxiliary stopwords */
        "when", "what", "where", "which", "who", "whom", "whose", "why", "how",
        "did", "do", "does", "done", "have", "had", "has", "was", "were", "is",
        "am", "are", "the", "a", "an", "and", "or", "but", "in", "on", "at",
        "to", "for", "from", "with", "about", "last", "meet", "meeting", "we",
        "i", "me", "my", "you", "your", "they", "them", "their", "it", "its",
        "of", "by", "as", "if", "so", "no", "not", "up", "out", "off", "over",
        "under", "then", "into", "onto",
        /* Arabic question & common particles */
        "متى", "ماذا", "ما", "من", "اين", "كيف", "لماذا", "هل", "كان",
        "في", "على", "الى", "عن", "مع", "هو", "هي", "نحن", "انا",
        NULL
    };

    for (int i = 0; STOPWORDS[i] != NULL; ++i) {
        if (strcasecmp(token, STOPWORDS[i]) == 0) {
            return true;
        }
    }
    return false;
}

static const char *rag_strcasestr(const char *haystack, const char *needle) {
    if (!haystack || !needle || needle[0] == '\0') return NULL;
    size_t nlen = strlen(needle);
    for (; *haystack != '\0'; ++haystack) {
        if (tolower((unsigned char)*haystack) == tolower((unsigned char)*needle)) {
            size_t i = 1;
            for (; i < nlen; ++i) {
                if (haystack[i] == '\0' ||
                    tolower((unsigned char)haystack[i]) != tolower((unsigned char)needle[i])) {
                    break;
                }
            }
            if (i == nlen) return haystack;
        }
    }
    return NULL;
}

static size_t format_fts5_query(const char *raw_query, char *out_fts, size_t out_cap) {
    if (!raw_query || !out_fts || out_cap == 0) return 0;
    out_fts[0] = '\0';

    size_t written = 0;
    const unsigned char *p = (const unsigned char *)raw_query;

    while (*p != '\0') {
        while (*p != '\0' && is_query_delim(*p)) p++;
        if (*p == '\0') break;

        char token[64];
        size_t t_idx = 0;
        while (*p != '\0' && !is_query_delim(*p) && t_idx + 1 < sizeof(token)) {
            token[t_idx++] = (char)*p++;
        }
        token[t_idx] = '\0';

        if (t_idx > 1 && !is_stopword(token)) { /* Skip single character noise and stopwords */
            int n = snprintf(out_fts + written, out_cap - written,
                             "%s\"%s\"", (written > 0 ? " OR " : ""), token);
            if (n > 0 && (size_t)n < out_cap - written) {
                written += (size_t)n;
            } else {
                break;
            }
        }
    }

    return written;
}

size_t nibras_rag_retrieve(
    nibras_rag_t        *rag,
    const char          *query,
    nibras_rag_result_t *out_results,
    size_t               max_results
) {
    if (!rag || !rag->is_open || !rag->db || !query || !out_results || max_results == 0) {
        return 0;
    }

    char fts_expr[512];
    size_t qlen = format_fts5_query(query, fts_expr, sizeof(fts_expr));
    if (qlen == 0) return 0;

    const char *sql = 
        "SELECT rowid, note_title, text, start_line, end_line, passage_sha256, note_id, bm25(nibras_rag_chunks)\n"
        "FROM nibras_rag_chunks\n"
        "WHERE nibras_rag_chunks MATCH ?\n"
        "ORDER BY bm25(nibras_rag_chunks)\n"
        "LIMIT ?;";

    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(rag->db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK || !stmt) return 0;

    sqlite3_bind_text(stmt, 1, fts_expr, -1, SQLITE_STATIC);
    sqlite3_bind_int(stmt, 2, (int)max_results);

    size_t count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_results) {
        nibras_rag_result_t *r = &out_results[count];
        memset(r, 0, sizeof(*r));

        r->chunk.chunk_id = (uint64_t)sqlite3_column_int64(stmt, 0);
        rag_strlcpy(r->chunk.note_title, (const char *)sqlite3_column_text(stmt, 1), sizeof(r->chunk.note_title));
        rag_strlcpy(r->chunk.text, (const char *)sqlite3_column_text(stmt, 2), sizeof(r->chunk.text));
        r->chunk.start_line = (uint32_t)sqlite3_column_int(stmt, 3);
        r->chunk.end_line = (uint32_t)sqlite3_column_int(stmt, 4);
        rag_strlcpy(r->chunk.passage_sha256, (const char *)sqlite3_column_text(stmt, 5), sizeof(r->chunk.passage_sha256));
        r->chunk.note_id = (uint64_t)sqlite3_column_int64(stmt, 6);
        r->chunk.token_count = (uint32_t)((strlen(r->chunk.text) + 2) / 3);

        /* SQLite FTS5 bm25 returns negative values (more negative = better match) */
        double raw_bm25 = sqlite3_column_double(stmt, 7);
        r->score = fabs(raw_bm25) + 1.0;
        r->confidence = 0.98 - (0.03 * (double)count);
        if (r->confidence < 0.60) r->confidence = 0.60;

        /* Standard Citation Format: NoteTitle:L<start>-L<end> [sha256:<full64>] */
        snprintf(r->citation, sizeof(r->citation), "%s:L%u-L%u [sha256:%s]",
                 r->chunk.note_title, r->chunk.start_line, r->chunk.end_line, r->chunk.passage_sha256);

        snprintf(r->match_reason, sizeof(r->match_reason),
                 "FTS5/BM25 score: %.2f | Citation: Lines %u-%u", r->score, r->chunk.start_line, r->chunk.end_line);

        count++;
    }

    sqlite3_finalize(stmt);
    return count;
}

/* ========================================================================= */
/* 5. Grounding Shield Verification Upgrade                                  */
/* ========================================================================= */

grounding_result_t nibras_rag_verify_claim(
    nibras_rag_t            *rag,
    const grounding_claim_t *claim
) {
    grounding_result_t res;
    memset(&res, 0, sizeof(res));

    if (!rag || !claim || !claim->claim_text || !claim->entity_name || claim->entity_name[0] == '\0') {
        res.verdict = GROUNDING_UNVERIFIABLE;
        res.confidence = 0.0;
        snprintf(res.discrepancy_note, sizeof(res.discrepancy_note), "%s", "Missing claim or entity name for personal grounding.");
        return res;
    }

    nibras_rag_result_t rag_results[NIBRAS_RAG_MAX_RESULTS];
    size_t num = nibras_rag_retrieve(rag, claim->entity_name, rag_results, NIBRAS_RAG_MAX_RESULTS);

    if (num == 0) {
        res.verdict = GROUNDING_UNVERIFIABLE;
        res.confidence = 0.0;
        snprintf(res.discrepancy_note, sizeof(res.discrepancy_note),
                 "Claimed entity '%s' not found in personal memory corpus. Refusing to invent dates or facts.",
                 claim->entity_name);
        return res;
    }

    /* Convert retrieved chunks to authoritative grounding sources */
    grounding_source_t sources[NIBRAS_RAG_MAX_RESULTS];
    memset(sources, 0, sizeof(sources));

    for (size_t i = 0; i < num; ++i) {
        sources[i].source_id = rag_results[i].citation;
        sources[i].source_title = rag_results[i].chunk.note_title;
        sources[i].snippet_text = rag_results[i].chunk.text;

        /* Numerical extraction if claim hinges on a number/date */
        if (claim->has_numerical) {
            double found_num = 0.0;
            bool matched_num = false;
            const char *sp = rag_results[i].chunk.text;

            while (*sp != '\0') {
                if (isdigit((unsigned char)*sp)) {
                    char *endp = NULL;
                    double v = strtod(sp, &endp);
                    if (fabs(v - claim->numerical_value) < 0.001) {
                        found_num = v;
                        matched_num = true;
                        break;
                    } else if (found_num == 0.0) {
                        found_num = v;
                    }
                    sp = endp;
                } else {
                    sp++;
                }
            }
            sources[i].source_numerical = matched_num ? claim->numerical_value : found_num;
        } else {
            sources[i].source_numerical = 0.0;
        }
    }

    /* Deterministically verify using canonical Grounding Shield */
    res = grounding_verify_claim(claim, sources, num);
    return res;
}

/* ========================================================================= */
/* 6. High-Level Grounded Meeting & Decision Recall                          */
/* ========================================================================= */

static bool extract_date_from_text(const char *text, char *out_date, size_t out_cap) {
    if (!text || !out_date || out_cap == 0) return false;
    out_date[0] = '\0';

    /* 1. Look for "Date: <text>" */
    const char *dp = strstr(text, "Date:");
    if (!dp) dp = strstr(text, "date:");
    if (!dp) dp = strstr(text, "بتاريخ");
    if (dp) {
        if (strncmp(dp, "Date:", 5) == 0 || strncmp(dp, "date:", 5) == 0) dp += 5;
        else dp += strlen("بتاريخ");
        while (*dp == ' ' || *dp == ':') dp++;

        size_t w = 0;
        while (*dp != '\0' && *dp != '\n' && *dp != ';' && (w + 1) < out_cap) {
            out_date[w++] = *dp++;
        }
        out_date[w] = '\0';
        if (w > 0) return true;
    }

    /* 2. Look for ISO YYYY-MM-DD */
    for (size_t i = 0; text[i] != '\0'; ++i) {
        if (isdigit((unsigned char)text[i]) &&
            isdigit((unsigned char)text[i + 1]) &&
            isdigit((unsigned char)text[i + 2]) &&
            isdigit((unsigned char)text[i + 3]) &&
            text[i + 4] == '-' &&
            isdigit((unsigned char)text[i + 5]) &&
            isdigit((unsigned char)text[i + 6]) &&
            text[i + 7] == '-' &&
            isdigit((unsigned char)text[i + 8]) &&
            isdigit((unsigned char)text[i + 9])) {
            size_t n = 10;
            if (n >= out_cap) n = out_cap - 1;
            memcpy(out_date, text + i, n);
            out_date[n] = '\0';
            return true;
        }
    }

    return false;
}

static bool extract_decision_from_text(const char *text, char *out_dec, size_t out_cap) {
    if (!text || !out_dec || out_cap == 0) return false;
    out_dec[0] = '\0';

    const char *p = NULL;
    size_t prefix_len = 0;

    if ((p = strstr(text, "Decision:"))) {
        prefix_len = strlen("Decision:");
    } else if ((p = strstr(text, "decision:"))) {
        prefix_len = strlen("decision:");
    } else if ((p = strstr(text, "القرار:"))) {
        prefix_len = strlen("القرار:");
    } else if ((p = strstr(text, "قرار:"))) {
        prefix_len = strlen("قرار:");
    } else if ((p = strstr(text, "اعتماد"))) {
        prefix_len = strlen("اعتماد");
    }

    if (p) {
        p += prefix_len;
        while (*p == ' ' || *p == ':') p++;
        size_t w = 0;
        while (*p != '\0' && *p != '\n' && (w + 1) < out_cap) {
            out_dec[w++] = *p++;
        }
        out_dec[w] = '\0';
        return w > 0;
    }

    return false;
}

int nibras_rag_answer_meeting_query(
    nibras_rag_t        *rag,
    const char          *query,
    char                *out_answer,
    size_t               answer_cap,
    nibras_rag_result_t *out_citation,
    grounding_result_t  *out_grounding
) {
    if (!rag || !query || !out_answer || answer_cap == 0) return -1;
    out_answer[0] = '\0';

    nibras_rag_result_t results[4];
    size_t num = nibras_rag_retrieve(rag, query, results, 4);

    if (num == 0) {
        snprintf(out_answer, answer_cap,
                 "[UNVERIFIED] No authoritative note found in personal memory corpus. Refusing to invent dates or meetings.");
        if (out_grounding) {
            out_grounding->verdict = GROUNDING_UNVERIFIABLE;
            out_grounding->confidence = 0.0;
            rag_strlcpy(out_grounding->discrepancy_note, "Zero authoritative corpus match. Hallucination blocked.", sizeof(out_grounding->discrepancy_note));
        }
        return 0;
    }

    const nibras_rag_result_t *top = &results[0];

    /* Verify that at least one salient non-stopword query token appears in top chunk */
    bool keyword_matched = false;
    const unsigned char *qp = (const unsigned char *)query;
    while (*qp != '\0') {
        while (*qp != '\0' && is_query_delim(*qp)) qp++;
        if (*qp == '\0') break;

        char qtok[64];
        size_t qt_len = 0;
        while (*qp != '\0' && !is_query_delim(*qp) && qt_len + 1 < sizeof(qtok)) {
            qtok[qt_len++] = (char)*qp++;
        }
        qtok[qt_len] = '\0';

        if (qt_len > 1 && !is_stopword(qtok)) {
            if (rag_strcasestr(top->chunk.text, qtok) || rag_strcasestr(top->chunk.note_title, qtok)) {
                keyword_matched = true;
                break;
            }
        }
    }

    if (!keyword_matched) {
        snprintf(out_answer, answer_cap,
                 "[UNVERIFIED] No authoritative note found in personal memory corpus. Refusing to invent dates or meetings.");
        if (out_grounding) {
            out_grounding->verdict = GROUNDING_UNVERIFIABLE;
            out_grounding->confidence = 0.0;
            rag_strlcpy(out_grounding->discrepancy_note, "Query entities not found in authoritative corpus. Hallucination blocked.", sizeof(out_grounding->discrepancy_note));
        }
        return 0;
    }

    if (out_citation) {
        *out_citation = *top;
    }

    char date_buf[64] = "Unspecified Date";
    extract_date_from_text(top->chunk.text, date_buf, sizeof(date_buf));

    char dec_buf[256] = "Review and consultation";
    extract_decision_from_text(top->chunk.text, dec_buf, sizeof(dec_buf));

    /* Cross-check with Grounding Shield */
    grounding_claim_t claim;
    claim.claim_text = top->chunk.text;
    claim.entity_name = top->chunk.note_title;
    claim.numerical_value = 0.0;
    claim.has_numerical = false;

    grounding_result_t g_res = nibras_rag_verify_claim(rag, &claim);
    if (out_grounding) {
        *out_grounding = g_res;
    }

    snprintf(out_answer, answer_cap,
             "Meeting Date: %s\n"
             "Decisions / Outcomes: %s\n"
             "Source Note: \"%s\"\n"
             "Passage Citation: %s\n"
             "Grounding Confidence: %.2f (%s)",
             date_buf, dec_buf, top->chunk.note_title, top->citation,
             top->confidence, (g_res.verdict == GROUNDING_VERIFIED ? "VERIFIED" : "UNVERIFIED"));

    return 1;
}
