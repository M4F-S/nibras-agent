/* test_nibras_suite.c - Comprehensive Verification Suite for Nibras
 * Target: Best Personal AI Agent | Dubai Create AI Agents Championship 2026-2027
 * Engine: Almaz Cognitive Organism | C99 Hardened
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <assert.h>
#include <math.h>
#include <time.h>
#include <unistd.h>

#include "nibras_adapters.h"
#include "nibras_memory_store.h"
#include "grounding_shield.h"
#include "nibras_rag.h"

static void test_arabic_normalizer(void) {
    printf("[TEST 1/5] Running Zero-Allocation UTF-8 Arabic Normalizer tests...\n");

    /* 1. Tashkeel and Tatweel stripping */
    const char *raw1 = "مُدِيرُ الْمَشْرُوعِ فِي دُبَيّــــــ كَانَ هُنَا";
    char norm1[256];
    arabic_normalize_utf8_hardened(raw1, norm1, sizeof(norm1));
    /* Diacritics and tatweel stripped: "مدير المشروع في دبي كان هنا" */
    assert(strstr(norm1, "مدير") != NULL);
    assert(strstr(norm1, "المشروع") != NULL);
    assert(strstr(norm1, "دبي") != NULL);
    assert(strstr(norm1, "ـ") == NULL); /* Tatweel stripped */

    /* 2. Alef / Hamza / Ta-Marbuta unification */
    const char *raw2 = "إدارة أزمات آل مكتوم بمدرسةٍ ذكية";
    char norm2[256];
    arabic_normalize_utf8_hardened(raw2, norm2, sizeof(norm2));
    /* إ -> ا, أ -> ا, آ -> ا, ة -> ه */
    assert(strstr(norm2, "اداره") != NULL);
    assert(strstr(norm2, "ازمات") != NULL);
    assert(strstr(norm2, "ال مكتوم") != NULL);
    assert(strstr(norm2, "بمدرسه") != NULL);

    /* 3. Eastern and Persian digit folding */
    const char *raw3 = "عام ٢٠٢٧ و سنة ۰۹۸۷۶";
    char norm3[256];
    arabic_normalize_utf8_hardened(raw3, norm3, sizeof(norm3));
    assert(strstr(norm3, "2027") != NULL);
    assert(strstr(norm3, "09876") != NULL);

    /* 4. Idempotence test */
    char norm3_again[256];
    arabic_normalize_utf8_hardened(norm3, norm3_again, sizeof(norm3_again));
    assert(strcmp(norm3, norm3_again) == 0);

    /* 5. Buffer safety: small destination buffer */
    char tiny[6];
    size_t written = arabic_normalize_utf8_hardened("دبي العالمية", tiny, sizeof(tiny));
    assert(written < sizeof(tiny));
    assert(tiny[written] == '\0');

    printf("  [PASS] Zero-allocation UTF-8 Arabic Normalizer verified.\n");
}

static void test_deterministic_entity_extractor(void) {
    printf("[TEST 2/5] Running Deterministic Arabic Entity Extractor tests...\n");

    /* 1. Executive Appointment Extraction */
    const char *text1 = "اجتماع مع رئيس هيئة الصحة بدبي غداً الساعة 10 صباحاً لمراجعة السياسة";
    char norm1[256];
    arabic_normalize_utf8_hardened(text1, norm1, sizeof(norm1));

    nibras_extracted_info_t info1;
    size_t count1 = nibras_extract_arabic_entities(norm1, &info1);
    assert(count1 >= 3);
    assert(info1.is_appointment == true);
    assert(info1.is_action_item == true);
    assert(info1.is_executive_decision == true); /* Contains "سياسه" */
    assert(info1.suggest_pin == true);
    assert(info1.suggested_salience >= 0.90);
    assert(strstr(info1.datetime_str, "غدا") != NULL);
    assert(strstr(info1.datetime_str, "الساعه 10 صباحا") != NULL);

    /* 2. Action Item with Weekday */
    const char *text2 = "متابعة سداد رسوم ترخيص DIFC يوم الأحد القادم";
    char norm2[256];
    arabic_normalize_utf8_hardened(text2, norm2, sizeof(norm2));

    nibras_extracted_info_t info2;
    nibras_extract_arabic_entities(norm2, &info2);
    assert(info2.is_action_item == true);
    assert(info2.is_appointment == false);
    assert(strstr(info2.action_verb, "متابعه") != NULL);
    assert(strstr(info2.datetime_str, "الاحد") != NULL);
    assert(info2.suggested_salience >= 0.80);

    /* 3. Statutory Mandate / Permanent Decision */
    const char *text3 = "قرار مجلس الإدارة: اعتماد التسجيل النهائي في DIFC";
    char norm3[256];
    arabic_normalize_utf8_hardened(text3, norm3, sizeof(norm3));

    nibras_extracted_info_t info3;
    nibras_extract_arabic_entities(norm3, &info3);
    assert(info3.is_executive_decision == true);
    assert(info3.suggest_pin == true);
    assert(info3.suggested_salience >= 0.95);
    assert(info3.suggested_valence == 1);

    /* 4. Ephemeral Operational Chatter */
    const char *text4 = "شربت قهوة مع الأصدقاء في المطعم اليوم";
    char norm4[256];
    arabic_normalize_utf8_hardened(text4, norm4, sizeof(norm4));

    nibras_extracted_info_t info4;
    nibras_extract_arabic_entities(norm4, &info4);
    assert(info4.is_action_item == false);
    assert(info4.is_appointment == false);
    assert(info4.is_executive_decision == false);
    assert(info4.suggest_pin == false);
    assert(info4.suggested_salience <= 0.60);

    /* 5. Friction / Penalty Valence Detection */
    const char *text5 = "تنبيه بوجود تأخير في المعاملة وغرامة مالية على التأخير";
    char norm5[256];
    arabic_normalize_utf8_hardened(text5, norm5, sizeof(norm5));

    nibras_extracted_info_t info5;
    nibras_extract_arabic_entities(norm5, &info5);
    assert(info5.suggested_valence == -1);

    printf("  [PASS] Deterministic Arabic Entity Extractor verified.\n");
}

static void test_fademem_decay_and_pruning(void) {
    printf("[TEST 3/5] Running FadeMem Biological Decay & Token Pruning tests...\n");

    /* Test nodes: 1 ephemeral noise node + 1 pinned executive decision */
    nibras_memory_node_t nodes[2] = {
        {
            .id = 1,
            .summary = "Ephemeral chatter about office lunch",
            .valence = 0,
            .initial_intensity = 0.35,
            .current_intensity = 0.35,
            .created_utc = 1792600000ULL,
            .token_cost = 500,
            .is_pinned = false,
            .state = TRAJ_ACTIVE_EPISODIC
        },
        {
            .id = 2,
            .summary = "Critical executive statutory decision",
            .valence = 1,
            .initial_intensity = 0.95,
            .current_intensity = 0.95,
            .created_utc = 1792600000ULL,
            .token_cost = 800,
            .is_pinned = true,
            .state = TRAJ_DISTILLED_RULE
        }
    };

    nibras_telemetry_snapshot_t snap;
    /* Simulate 7 days forward (7 * 86400 seconds) */
    uint64_t future_time = 1792600000ULL + (7 * 86400ULL);
    nibras_fademem_step_hardened(nodes, 2, future_time, 86400.0, 1.0, 3 * 86400.0, &snap);

    /* Node 1 decayed exponentially below 0.25 -> PRUNED */
    assert(nodes[0].state == TRAJ_NOISE_PRUNED);
    assert(nodes[0].current_intensity < 0.25);

    /* Node 2 is pinned -> Stayed at intensity 1.0 and DISTILLED_RULE */
    assert(nodes[1].state == TRAJ_DISTILLED_RULE);
    assert(nodes[1].current_intensity == 1.0);
    assert(snap.distilled_rules_count == 1);

    /* Token reduction exceeds 85% */
    assert(snap.tokens_saved_pct >= 85);

    printf("  [PASS] FadeMem Biological Decay & Token Pruning verified (Saved: %u%%).\n", snap.tokens_saved_pct);
}

static void test_jsonl_memory_store(void) {
    printf("[TEST 4/5] Running Persistent JSONL Memory Store tests...\n");

    const char *test_store_path = "build/test_store.jsonl";
    unlink(test_store_path);

    nibras_memory_store_t store;
    int rc = nibras_store_init(&store, test_store_path);
    assert(rc == 0);
    assert(store.count == 0);

    /* Ingest 3 tasks */
    uint64_t id1 = nibras_store_ingest(&store, "اجتماع مع هيئة الصحة بدبي غداً الساعة 10 صباحاً", 150, NULL);
    uint64_t id2 = nibras_store_ingest(&store, "قرار مجلس الإدارة: اعتماد الميزانية والامتثال للقانون 45", 250, NULL);
    uint64_t id3 = nibras_store_ingest(&store, "شربت قهوة مع الزملاء وناقشنا مواضيع عامة", 80, NULL);

    assert(id1 == 1);
    assert(id2 == 2);
    assert(id3 == 3);
    assert(store.count == 3);

    /* Verify id2 is pinned by default because of "قرار" */
    nibras_memory_node_t *n2 = nibras_store_find_by_id(&store, id2);
    assert(n2 != NULL);
    assert(n2->is_pinned == true);
    assert(n2->state == TRAJ_DISTILLED_RULE);

    /* Pin id1 explicitly */
    nibras_store_set_pin(&store, id1, true);
    nibras_memory_node_t *n1 = nibras_store_find_by_id(&store, id1);
    assert(n1 != NULL);
    assert(n1->is_pinned == true);

    /* Search store */
    nibras_memory_node_t matches[8];
    size_t found = nibras_store_search(&store, "الصحة", matches, 8);
    assert(found >= 1);
    assert(matches[0].id == id1);

    found = nibras_store_search(&store, "قهوة", matches, 8);
    assert(found >= 1);
    assert(matches[0].id == id3);

    /* Reload store from file into fresh instance */
    nibras_memory_store_t loaded_store;
    nibras_store_init(&loaded_store, test_store_path);
    assert(loaded_store.count == 3);

    nibras_memory_node_t *ln1 = nibras_store_find_by_id(&loaded_store, id1);
    assert(ln1 != NULL);
    assert(ln1->is_pinned == true);

    nibras_memory_node_t *ln3 = nibras_store_find_by_id(&loaded_store, id3);
    assert(ln3 != NULL);
    assert(ln3->is_pinned == false);

    /* Step decay 7 days forward on loaded store */
    uint64_t future = (uint64_t)time(NULL) + (7 * 86400ULL);
    nibras_telemetry_snapshot_t snap;
    nibras_store_step_decay(&loaded_store, future, 86400.0, 1.0, 3 * 86400.0, &snap);

    /* Node 3 (chatter) should be pruned */
    ln3 = nibras_store_find_by_id(&loaded_store, id3);
    assert(ln3->state == TRAJ_NOISE_PRUNED);

    /* Nodes 1 and 2 (pinned) should remain distilled rules at 1.0 intensity */
    ln1 = nibras_store_find_by_id(&loaded_store, id1);
    assert(ln1->state == TRAJ_DISTILLED_RULE);
    assert(ln1->current_intensity == 1.0);

    /* Clean up temporary test file */
    unlink(test_store_path);
    printf("  [PASS] Persistent JSONL Memory Store verified.\n");
}

static void test_sovereign_privacy_and_grounding(void) {
    printf("[TEST 5/5] Running UAE PDPL Privacy & Grounding Shield tests...\n");

    /* 1. Non-destructive PII Redaction */
    const char *prompt_with_pii = "Patient John Doe, Emirates ID 784-1988-1234567-1, DOB: 1988-05-12, take Metformin 500mg BID";
    char redacted[512];
    size_t red_len = grounding_redact_prompt_pii_c99(prompt_with_pii, redacted, sizeof(redacted));
    assert(red_len > 0);
    assert(strstr(redacted, "784-1988-1234567-1") == NULL);
    assert(strstr(redacted, "[EMIRATES_ID_REDACTED]") != NULL);
    assert(strstr(redacted, "Metformin 500mg BID") != NULL);

    /* 2. Privacy-Scrubbed Search Query Extractor (DESC ISR v3.1 compliance) */
    const char *raw_search = "What is the penalty for DHA Ejadah delay resubmission under SN 03/2019 for customer 12345?";
    char scrubbed[256];
    size_t scrub_len = grounding_scrub_search_query(raw_search, scrubbed, sizeof(scrubbed));
    assert(scrub_len > 0);
    assert(strstr(scrubbed, "DHA") != NULL);
    assert(strstr(scrubbed, "Ejadah") != NULL);
    assert(strstr(scrubbed, "SN 03/2019") != NULL);
    assert(strstr(scrubbed, "12345") == NULL); /* PII customer number excluded */

    printf("  [PASS] UAE PDPL Privacy & Grounding Shield verified.\n");
}

static bool validate_utf8(const char *s) {
    if (!s) return false;
    const unsigned char *p = (const unsigned char *)s;
    while (*p != '\0') {
        if (*p <= 0x7F) {
            p++;
        } else if ((*p & 0xE0) == 0xC0) {
            if (*p < 0xC2) return false; /* Overlong encoding */
            if ((p[1] & 0xC0) != 0x80) return false;
            p += 2;
        } else if ((*p & 0xF0) == 0xE0) {
            if (*p == 0xE0 && p[1] < 0xA0) return false; /* Overlong */
            if ((p[1] & 0xC0) != 0x80 || (p[2] & 0xC0) != 0x80) return false;
            p += 3;
        } else if ((*p & 0xF8) == 0xF0) {
            if (*p == 0xF0 && p[1] < 0x90) return false; /* Overlong */
            if ((p[1] & 0xC0) != 0x80 || (p[2] & 0xC0) != 0x80 || (p[3] & 0xC0) != 0x80) return false;
            p += 4;
        } else {
            return false;
        }
    }
    return true;
}

static void test_utf8_truncation_regression(void) {
    printf("[TEST N-1] Running UTF-8 Truncation & String Boundary Regression Test...\n");

    const char *reg_store_path = "build/test_utf8_regression.jsonl";
    unlink(reg_store_path);

    nibras_memory_store_t store;
    int rc = nibras_store_init(&store, reg_store_path);
    assert(rc == 0);

    /* 40+ character mixed Arabic/English appointment with trailing Arabic text and datetime */
    const char *mixed_title = "اجتماع Joint AI Governance Committee 2026 في دبي غداً الساعة 10 صباحاً";
    uint64_t id = nibras_store_ingest(&store, mixed_title, 0, NULL);
    assert(id > 0);

    /* 1. Inspect raw JSONL file directly to verify strict UTF-8 validity */
    FILE *f = fopen(reg_store_path, "r");
    assert(f != NULL);
    char line[2048];
    while (fgets(line, sizeof(line), f)) {
        assert(validate_utf8(line) == true);
        /* Ensure no dangling lead bytes 0xD8/0xD9/0xDB right before quote or bracket */
        size_t len = strlen(line);
        for (size_t i = 0; i < len; ++i) {
            unsigned char b = (unsigned char)line[i];
            if (b >= 0xC0 && (i + 1 == len || (line[i + 1] & 0xC0) != 0x80)) {
                assert(false && "Split UTF-8 lead byte detected in JSONL output!");
            }
        }
    }
    fclose(f);

    /* 2. Reload into fresh store instance and assert decoded properties */
    nibras_memory_store_t reload_store;
    rc = nibras_store_init(&reload_store, reg_store_path);
    assert(rc == 0);
    assert(reload_store.count == 1);

    nibras_memory_node_t *node = nibras_store_find_by_id(&reload_store, id);
    assert(node != NULL);
    assert(validate_utf8(node->summary) == true);
    assert(validate_utf8(node->raw_arabic) == true);
    assert(validate_utf8(node->category) == true);

    printf("  [PASS] Decoded Summary: \"%s\" (len=%zu bytes, 100%% valid UTF-8)\n",
           node->summary, strlen(node->summary));

    unlink(reg_store_path);
    printf("  [PASS] N-1 Arabic UTF-8 truncation regression suite verified.\n");
}

static void test_bm25_ranked_search(void) {
    printf("[TEST Round-2] Running BM25 Ranked Retrieval & FadeMem Decay Pruning Test...\n");

    const char *test_bm25_path = "build/test_bm25_store.jsonl";
    unlink(test_bm25_path);

    nibras_memory_store_t store;
    int rc = nibras_store_init(&store, test_bm25_path);
    assert(rc == 0);

    /* Ingest 5 realistic executive notes:
     * 1. Note 1: Pinned executive decision containing both "DIFC" and "ترخيص" (target top-rank)
     * 2. Note 2: Consolidated action item containing only "ترخيص" (partial match)
     * 3. Note 3: Ephemeral chatter containing "DIFC" (will be pruned by decay)
     * 4. Note 4: Unrelated appointment with health authority
     * 5. Note 5: Routine English server ping
     */
    uint64_t id1 = nibras_store_ingest(&store, "قرار مجلس الإدارة: اعتماد ترخيص DIFC الجديد وتفويض الإدارة التنفيذية", 180, NULL);
    uint64_t id2 = nibras_store_ingest(&store, "موافقة على سداد رسوم ترخيص المكاتب والشركات في دبي", 120, NULL);
    uint64_t id3 = nibras_store_ingest(&store, "جلسة قهوة سريعة في DIFC مع فريق العمل لمناقشة عطلة الأسبوع", 90, NULL);
    uint64_t id4 = nibras_store_ingest(&store, "اجتماع مع هيئة الصحة بدبي غداً الساعة 10 صباحاً لمراجعة اللائحة", 140, NULL);
    uint64_t id5 = nibras_store_ingest(&store, "Routine CI build status check and daily server uptime ping", 60, NULL);

    assert(id1 == 1 && id2 == 2 && id3 == 3 && id4 == 4 && id5 == 5);
    assert(store.count == 5);

    /* Verify Note 1 is pinned */
    nibras_memory_node_t *n1 = nibras_store_find_by_id(&store, id1);
    assert(n1 != NULL && n1->is_pinned == true);

    /* Verify Note 3 is unpinned ephemeral chatter with low salience */
    nibras_memory_node_t *n3 = nibras_store_find_by_id(&store, id3);
    assert(n3 != NULL && n3->is_pinned == false);
    assert(n3->initial_intensity <= 0.65);

    /* Pre-decay search: 2-word mixed Arabic/English query */
    const char *query = "DIFC ترخيص";
    nibras_search_result_t results[8];
    size_t num = nibras_store_search_ranked(&store, query, results, 8);

    assert(num >= 2);
    /* Note 1 matches both terms and must rank first */
    assert(results[0].node.id == id1);
    assert(results[0].score > 0.0);
    assert(strstr(results[0].match_reason, "difc") != NULL);
    assert(strstr(results[0].match_reason, "ترخيص") != NULL);

    /* Step FadeMem decay 7 days forward:
     * Note 3 decays below 0.25 and becomes TRAJ_NOISE_PRUNED.
     * Note 1 remains TRAJ_DISTILLED_RULE at intensity 1.0.
     */
    uint64_t future_now = (uint64_t)time(NULL) + (7 * 86400ULL);
    nibras_telemetry_snapshot_t snap;
    nibras_store_step_decay(&store, future_now, 86400.0, 1.0, 3 * 86400.0, &snap);

    /* Verify Note 3 is now pruned */
    n3 = nibras_store_find_by_id(&store, id3);
    assert(n3 != NULL);
    assert(n3->state == TRAJ_NOISE_PRUNED);

    /* Post-decay BM25 ranked search:
     * Assert the right note (Note 1) ranks first and the pruned one (Note 3) is ABSENT.
     */
    size_t post_num = nibras_store_search_ranked(&store, query, results, 8);
    assert(post_num == 2);
    assert(results[0].node.id == id1);
    assert(results[1].node.id == id2);
    assert(results[0].score > results[1].score);

    /* Confirm pruned note 3 is completely absent */
    for (size_t i = 0; i < post_num; ++i) {
        assert(results[i].node.id != id3);
        assert(results[i].node.state != TRAJ_NOISE_PRUNED);
    }

    /* Test UTF-8 safety with punctuation and spaces */
    nibras_search_result_t safe_results[8];
    size_t safe_num = nibras_store_search_ranked(&store, "  DIFC,   ترخيص!  ", safe_results, 8);
    assert(safe_num == post_num);
    assert(safe_results[0].node.id == id1);

    /* Test backward-compatible nibras_store_search */
    nibras_memory_node_t legacy_matches[8];
    size_t legacy_num = nibras_store_search(&store, query, legacy_matches, 8);
    assert(legacy_num == 2);
    assert(legacy_matches[0].id == id1);
    assert(legacy_matches[1].id == id2);

    printf("  [PASS] Top result: #%llu \"%s\" (BM25: %.2f)\n",
           (unsigned long long)results[0].node.id, results[0].node.summary, results[0].score);
    printf("  [PASS] Reason: %s\n", results[0].match_reason);
    printf("  [PASS] Pruned node #3 verified absent from ranked results.\n");

    unlink(test_bm25_path);
    printf("  [PASS] BM25 Ranked Retrieval test passed deterministically.\n");
}

static void test_personal_rag_citations(void) {
    printf("[TEST Round-3] Running Personal RAG & Note Citation Grounding Test...\n");

    /* 1. SHA-256 NIST Test Vectors Verification */
    char h1[65], h2[65];
    nibras_sha256_hex("", 0, h1);
    assert(strcmp(h1, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855") == 0);
    nibras_sha256_hex("abc", 3, h2);
    assert(strcmp(h2, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad") == 0);
    printf("  [PASS] FIPS 180-2 SHA-256 verified against NIST test vectors.\n");

    /* 2. SQLite FTS5 RAG Engine Initialization & Note Chunking */
    nibras_rag_t rag;
    int rc = nibras_rag_init(&rag, ":memory:");
    assert(rc == 0);

    const char *dha_minutes = 
        "Date: 2026-10-15 (15 October 2026)\n"
        "Attendees: Mohamed Fathy (Founder & Lead Architect), Chair of Dubai Health Authority (DHA)\n"
        "Discussion: Formal review of UAE Health Data Law No. 2 of 2019 and AI Security Standards\n"
        "Decision: Formal adoption of statutory AI Security Policy and NABIDH integration roadmap\n"
        "Action: Phase 1 compliance audit verification by end of Q4 2026";

    size_t chunks = nibras_rag_index_note(&rag, 101, "DHA Executive Meeting Minutes 2026", dha_minutes);
    assert(chunks >= 1);
    assert(rag.total_chunks_indexed == chunks);
    assert(rag.total_notes_indexed == 1);

    /* Index additional board resolution */
    const char *difc_resolution = 
        "Date: 2026-09-20\n"
        "Attendees: Board of Directors, Legal Counsel\n"
        "Decision: Approved incorporation under DIFC Companies Law No. 5 of 2018\n"
        "Action: Submit commercial license application to DIFC registrar";

    size_t difc_chunks = nibras_rag_index_note(&rag, 102, "DIFC Corporate Licensing Board Resolution", difc_resolution);
    assert(difc_chunks >= 1);

    /* 3. Retrieval with Exact Citations (Title + Line Range + SHA-256) */
    const char *query = "when did I last meet the DHA chair and what did we decide?";
    nibras_rag_result_t results[4];
    size_t num = nibras_rag_retrieve(&rag, query, results, 4);

    assert(num >= 1);
    assert(strcmp(results[0].chunk.note_title, "DHA Executive Meeting Minutes 2026") == 0);
    assert(results[0].chunk.start_line == 1);
    assert(results[0].chunk.end_line == 5);
    assert(strlen(results[0].chunk.passage_sha256) == 64);
    assert(results[0].score > 0.0);
    assert(results[0].confidence >= 0.90);

    /* Verify citation format: NoteTitle:L<start>-L<end> [sha256:<hash>] */
    assert(strstr(results[0].citation, "DHA Executive Meeting Minutes 2026:L1-L5") != NULL);
    assert(strstr(results[0].citation, results[0].chunk.passage_sha256) != NULL);
    printf("  [PASS] Retrieved Citation: %s\n", results[0].citation);

    /* 4. Grounding Shield Factual Cross-Verification */
    /* Accurate claim grounded in note */
    grounding_claim_t acc_claim = {
        .claim_text = "Chair of Dubai Health Authority (DHA) approved statutory AI Security Policy",
        .entity_name = "DHA",
        .numerical_value = 0.0,
        .has_numerical = false
    };
    grounding_result_t g_acc = nibras_rag_verify_claim(&rag, &acc_claim);
    assert(g_acc.verdict == GROUNDING_VERIFIED);
    assert(strstr(g_acc.citation, "DHA Executive Meeting Minutes 2026") != NULL);
    assert(g_acc.confidence >= 0.95);
    printf("  [PASS] Grounding Shield: Accurate claim VERIFIED with note citation.\n");

    /* Conflicting numerical claim trips hallucination */
    grounding_claim_t false_claim = {
        .claim_text = "Meeting with DHA chair was regarding statutory penalty 999.0",
        .entity_name = "DHA",
        .numerical_value = 999.0,
        .has_numerical = true
    };
    grounding_result_t g_false = nibras_rag_verify_claim(&rag, &false_claim);
    assert(g_false.verdict == GROUNDING_HALLUCINATION_TRIPPED);
    printf("  [PASS] Grounding Shield: Numerical hallucination TRIPPED and blocked.\n");

    /* Unrecorded entity claim returns UNVERIFIABLE */
    grounding_claim_t unrec_claim = {
        .claim_text = "Meeting with Minister of Transport took place last week",
        .entity_name = "Minister of Transport",
        .numerical_value = 0.0,
        .has_numerical = false
    };
    grounding_result_t g_unrec = nibras_rag_verify_claim(&rag, &unrec_claim);
    assert(g_unrec.verdict == GROUNDING_UNVERIFIABLE);
    printf("  [PASS] Grounding Shield: Unrecorded entity claim returned UNVERIFIABLE.\n");

    /* 5. High-Level Grounded Meeting Recall QA */
    char answer[512];
    nibras_rag_result_t cit;
    grounding_result_t g_qa;
    int ans_rc = nibras_rag_answer_meeting_query(&rag, query, answer, sizeof(answer), &cit, &g_qa);
    assert(ans_rc == 1);
    assert(strstr(answer, "2026-10-15") != NULL);
    assert(strstr(answer, "Formal adoption of statutory AI Security Policy") != NULL);
    assert(strstr(answer, "DHA Executive Meeting Minutes 2026:L1-L5") != NULL);
    assert(g_qa.verdict == GROUNDING_VERIFIED);
    printf("  [PASS] Grounded QA: Found authoritative date 2026-10-15 and decisions.\n");

    /* Assert Non-Invented Date Guarantee for unrecorded meeting */
    ans_rc = nibras_rag_answer_meeting_query(&rag, "when did I last meet the Minister of Transport?", answer, sizeof(answer), &cit, &g_qa);
    assert(ans_rc == 0);
    assert(strstr(answer, "[UNVERIFIED]") != NULL);
    assert(strstr(answer, "Refusing to invent dates") != NULL);
    assert(g_qa.verdict == GROUNDING_UNVERIFIABLE);
    printf("  [PASS] Anti-Hallucination: Refused to invent dates for unrecorded meeting.\n");

    nibras_rag_close(&rag);
    printf("  [PASS] Round-3 Personal RAG with Note Citations verified successfully!\n");
}

int main(void) {
    printf("==================================================================\n");
    printf(" NIBRAS (نبراس): COMPREHENSIVE SYSTEM VERIFICATION SUITE\n");
    printf("==================================================================\n");

    test_arabic_normalizer();
    test_deterministic_entity_extractor();
    test_fademem_decay_and_pruning();
    test_jsonl_memory_store();
    test_utf8_truncation_regression();
    test_bm25_ranked_search();
    test_personal_rag_citations();
    test_sovereign_privacy_and_grounding();

    printf("==================================================================\n");
    printf(" [SUCCESS] All 8 Nibras verification suites passed deterministically!\n");
    printf("==================================================================\n");
    return 0;
}
