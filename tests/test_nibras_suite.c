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

int main(void) {
    printf("==================================================================\n");
    printf(" NIBRAS (نبراس): COMPREHENSIVE SYSTEM VERIFICATION SUITE\n");
    printf("==================================================================\n");

    test_arabic_normalizer();
    test_deterministic_entity_extractor();
    test_fademem_decay_and_pruning();
    test_jsonl_memory_store();
    test_utf8_truncation_regression();
    test_sovereign_privacy_and_grounding();

    printf("==================================================================\n");
    printf(" [SUCCESS] All 6 Nibras verification suites passed deterministically!\n");
    printf("==================================================================\n");
    return 0;
}
