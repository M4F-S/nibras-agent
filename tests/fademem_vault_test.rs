//! SPRINT-08 Comprehensive Test Battery: FadeMem Biological Memory & Sovereign Vault
//! Target: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027
//! Founder / Lead Systems Architect: Mohamed Fathy, BPharm
//! Canonical Master Specification §4.2: FadeMem Biological Memory & Context Pruning

use nibras::memory::*;
use nibras::safety::ffi::{NibrasMemoryNode, TrajState};
use nibras::safety::C99MemoryStore;

#[test]
fn test_mathematical_ebbinghaus_retention_decay_accuracy() {
    let s_base = 86_400.0; // 24 hours in seconds
    let alpha = 0.5;
    let w_statutory = 1.0;

    // 1. At t = 0, R = e^0 = 1.0
    let r_0 = calculate_ebbinghaus_retention(0.0, s_base);
    assert!((r_0 - 1.0).abs() < 1e-9, "R(0) must equal exactly 1.0");

    // 2. At t = S, R = e^(-1) ≈ 0.36787944117
    let r_s = calculate_ebbinghaus_retention(s_base, s_base);
    let expected_e_minus_1 = (-1.0_f64).exp();
    assert!(
        (r_s - expected_e_minus_1).abs() < 1e-9,
        "R(S) must equal e^(-1), got {}, expected {}",
        r_s,
        expected_e_minus_1
    );

    // 3. Half-life relation: S = t_1/2 / ln(2)
    let half_life_sec = 86_400.0;
    let s_half = calculate_stability_from_half_life(half_life_sec);
    let r_half = calculate_ebbinghaus_retention(half_life_sec, s_half);
    assert!(
        (r_half - 0.5).abs() < 1e-9,
        "At t = t_1/2, retention must equal 0.5, got {}",
        r_half
    );

    // 4. Two half-lives: R = 0.25
    let r_two_half = calculate_ebbinghaus_retention(2.0 * half_life_sec, s_half);
    assert!(
        (r_two_half - 0.25).abs() < 1e-9,
        "At t = 2 * t_1/2, retention must equal 0.25, got {}",
        r_two_half
    );

    // 5. Three half-lives: R = 0.125
    let r_three_half = calculate_ebbinghaus_retention(3.0 * half_life_sec, s_half);
    assert!(
        (r_three_half - 0.125).abs() < 1e-9,
        "At t = 3 * t_1/2, retention must equal 0.125, got {}",
        r_three_half
    );

    // 6. Dynamic stability scaling with access count:
    // S = S_base * (1 + alpha * ln(1 + n_access)) * w_statutory
    let s_acc_0 = calculate_dynamic_stability(s_base, alpha, 0, w_statutory);
    assert!((s_acc_0 - s_base).abs() < 1e-9, "n_access = 0 must yield S_base");

    let s_acc_3 = calculate_dynamic_stability(s_base, alpha, 3, w_statutory);
    let expected_s_3 = s_base * (1.0 + alpha * (4.0_f64).ln());
    assert!(
        (s_acc_3 - expected_s_3).abs() < 1e-9,
        "Dynamic stability with 3 accesses must scale logarithmically"
    );
    assert!(s_acc_3 > s_acc_0, "More accesses must increase stability");

    // 7. Statutory weight scaling:
    let s_statutory = calculate_dynamic_stability(s_base, alpha, 0, 2.5);
    assert!(
        (s_statutory - s_base * 2.5).abs() < 1e-9,
        "Statutory weight 2.5 must scale stability by 2.5x"
    );
}

#[test]
fn test_pinned_node_immunity_over_30_and_365_days() {
    let mut engine = FadeMemEngine::new(FadeMemConfig {
        base_stability_sec: 86_400.0,
        default_half_life_sec: 86_400.0,
        alpha: 0.5,
        sim_time_scale: 1.0,
        consolidation_window_sec: 259_200.0,
        prune_threshold: 0.25,
        cfi_mode_threshold: 0.70,
    });

    let pinned_id = engine.add_node(
        "Statutory DIFC Law No. 1 of 2024 compliance policy",
        "statutory",
        true,
        500,
    );

    let unpinned_id = engine.add_node(
        "Discussed coffee preferences with office visitor",
        "chatter",
        false,
        500,
    );

    // Verify initial states
    let n_pinned = engine.find_node(pinned_id).unwrap();
    assert!(n_pinned.is_pinned);
    assert_eq!(n_pinned.retention, 1.0);
    assert_eq!(n_pinned.current_intensity, 1.0);
    assert_eq!(n_pinned.state, TrajState::DistilledRule);

    // Simulate 30 days (720 hours = 2,592,000 seconds)
    let snap_30d = engine.simulate_decay_hours(30.0 * 24.0);

    let n_pinned_30d = engine.find_node(pinned_id).unwrap();
    let n_unpinned_30d = engine.find_node(unpinned_id).unwrap();

    // Pinned invariant: immune to decay (R ≡ 1.0, I ≡ 1.0)
    assert!(n_pinned_30d.is_pinned);
    assert_eq!(n_pinned_30d.retention, 1.0);
    assert_eq!(n_pinned_30d.current_intensity, 1.0);
    assert_eq!(n_pinned_30d.state, TrajState::DistilledRule);

    // Unpinned node decayed below threshold -> pruned
    assert_eq!(n_unpinned_30d.state, TrajState::NoisePruned);
    assert!(n_unpinned_30d.retention < 0.25);
    assert_eq!(snap_30d.pruned_nodes_count, 1);
    assert_eq!(snap_30d.pinned_nodes_count, 1);

    // Simulate 365 days (8,760 hours = 31,536,000 seconds)
    let snap_365d = engine.simulate_decay_hours(335.0 * 24.0); // +335 days = 365 days total

    let n_pinned_365d = engine.find_node(pinned_id).unwrap();
    assert!(n_pinned_365d.is_pinned);
    assert_eq!(n_pinned_365d.retention, 1.0);
    assert_eq!(n_pinned_365d.current_intensity, 1.0);
    assert_eq!(n_pinned_365d.state, TrajState::DistilledRule);
    assert_eq!(snap_365d.pinned_nodes_count, 1);
}

#[test]
fn test_biological_pruning_threshold_point_two_five() {
    let half_life_sec = 86_400.0; // 24 hours
    let mut engine = FadeMemEngine::new(FadeMemConfig {
        base_stability_sec: calculate_stability_from_half_life(half_life_sec),
        default_half_life_sec: half_life_sec,
        alpha: 0.5,
        sim_time_scale: 1.0,
        consolidation_window_sec: 259_200.0,
        prune_threshold: 0.25,
        cfi_mode_threshold: 0.70,
    });

    let node_id = engine.add_node("Casual lunch meeting recap", "chatter", false, 300);

    // 1. Advance 24 hours (1 half life: R = 0.50 > 0.25)
    let snap_24h = engine.simulate_decay_hours(24.0);
    let node_24h = engine.find_node(node_id).unwrap();
    assert!(
        (node_24h.retention - 0.50).abs() < 1e-4,
        "Retention after 24h should be ~0.50, got {}",
        node_24h.retention
    );
    assert_eq!(node_24h.state, TrajState::ActiveEpisodic);
    assert_eq!(snap_24h.active_tokens_after_prune, 300);
    assert_eq!(snap_24h.pruned_nodes_count, 0);

    // 2. Advance to 47.9 hours (just before 2 half-lives: R > 0.25)
    engine.simulate_decay_hours(23.9);
    let node_before_prune = engine.find_node(node_id).unwrap();
    assert!(node_before_prune.retention > 0.25);
    assert_eq!(node_before_prune.state, TrajState::ActiveEpisodic);

    // 3. Advance past 48 hours (e.g. 72 hours = 3 half-lives: R = 0.125 < 0.25)
    let snap_72h = engine.simulate_decay_hours(24.1);
    let node_after_prune = engine.find_node(node_id).unwrap();
    assert!(
        node_after_prune.retention < 0.25,
        "Retention after 72h must fall below 0.25, got {}",
        node_after_prune.retention
    );
    assert_eq!(node_after_prune.state, TrajState::NoisePruned);
    assert_eq!(snap_72h.active_tokens_after_prune, 0);
    assert_eq!(snap_72h.pruned_nodes_count, 1);
    assert_eq!(snap_72h.tokens_saved_pct, 100);

    // 4. Verify node was archived to offline archive
    assert!(engine.offline_archive.iter().any(|a| a.id == node_id));
}

#[test]
fn test_sustained_greater_than_85pct_token_footprint_reduction_benchmark() {
    let mut engine = FadeMemEngine::new(FadeMemConfig {
        base_stability_sec: calculate_stability_from_half_life(86_400.0),
        default_half_life_sec: 86_400.0,
        alpha: 0.5,
        sim_time_scale: 1.0,
        consolidation_window_sec: 259_200.0,
        prune_threshold: 0.25,
        cfi_mode_threshold: 0.70,
    });

    // Ingest 2 permanent pinned statutory mandates
    let p1 = engine.add_node(
        "Decree No. 43 of 2013 Rent Increase Cap Table & Rules",
        "statutory",
        true,
        900,
    );
    let p2 = engine.add_node(
        "DIFC Data Protection Law No. 5 of 2020 Data Controller Mandate",
        "statutory",
        true,
        900,
    );

    // Ingest 60 conversational chatter episodic nodes over a simulated trajectory
    for i in 1..=60 {
        engine.add_node(
            &format!("Daily transient episodic update #{} regarding office logistics", i),
            "chatter",
            false,
            350,
        );
    }

    // Initial snapshot: 2 * 900 + 60 * 350 = 1,800 + 21,000 = 22,800 total raw tokens
    let initial_snap = engine.get_telemetry_snapshot(None);
    assert_eq!(initial_snap.total_raw_tokens, 22_800);
    assert_eq!(initial_snap.active_nodes_count, 62);

    // Simulate 72 hours (3 days) of elapsed operational time
    let snap_72h = engine.simulate_decay_hours(72.0);

    // Verify pinned nodes remain active and unpruned
    assert!(engine.find_node(p1).unwrap().is_pinned);
    assert!(engine.find_node(p2).unwrap().is_pinned);

    // Pinned nodes are distilled: (900 + 5) / 6 = 150 tokens each = 300 active tokens total
    assert_eq!(snap_72h.distilled_rules_count, 2);
    assert_eq!(snap_72h.active_tokens_after_prune, 300);

    // Mathematically verify token reduction percentage:
    // (22,800 - 300) / 22,800 = 22,500 / 22,800 ≈ 98.68% > 85%
    let saved_pct = snap_72h.tokens_saved_pct;
    assert!(
        saved_pct >= 85,
        "Token reduction must sustain >85%, got {}%",
        saved_pct
    );
    assert_eq!(saved_pct, 98);

    // Step further to Day 7 and Day 14; verify reduction sustains >85%
    let snap_7d = engine.simulate_decay_hours(96.0); // +4 days = 7 days
    assert!(
        snap_7d.tokens_saved_pct >= 85,
        "Token reduction must sustain >85% at Day 7, got {}%",
        snap_7d.tokens_saved_pct
    );

    let snap_14d = engine.simulate_decay_hours(168.0); // +7 days = 14 days
    assert!(
        snap_14d.tokens_saved_pct >= 85,
        "Token reduction must sustain >85% at Day 14, got {}%",
        snap_14d.tokens_saved_pct
    );
}

#[test]
fn test_cognitive_friction_index_formula_and_mode_switch() {
    // Formula: CFI = 0.3 * Delta_complexity + 0.3 * (1 - Confidence_RAG) + 0.4 * Risk_Regulatory

    // Case 1: Low complexity, high RAG confidence, low regulatory risk
    // 0.3 * 0.10 + 0.3 * (1 - 0.95) + 0.4 * 0.05 = 0.03 + 0.015 + 0.020 = 0.065
    let cfi_low = calculate_cfi(0.10, 0.95, 0.05);
    assert!((cfi_low - 0.065).abs() < 1e-9);
    assert_eq!(
        evaluate_output_mode(cfi_low),
        AgentOutputMode::ConversationalChatter
    );

    // Case 2: High complexity, low RAG confidence, high regulatory risk
    // 0.3 * 0.85 + 0.3 * (1 - 0.40) + 0.4 * 0.90 = 0.255 + 0.180 + 0.360 = 0.795
    let cfi_high = calculate_cfi(0.85, 0.40, 0.90);
    assert!((cfi_high - 0.795).abs() < 1e-9);
    assert_eq!(
        evaluate_output_mode(cfi_high),
        AgentOutputMode::StructuredDeclarative
    );

    // Case 3: Boundary switch testing around 0.70 threshold
    let cfi_sub_threshold = 0.699;
    assert_eq!(
        evaluate_output_mode(cfi_sub_threshold),
        AgentOutputMode::ConversationalChatter
    );

    let cfi_at_threshold = 0.700;
    assert_eq!(
        evaluate_output_mode(cfi_at_threshold),
        AgentOutputMode::ConversationalChatter
    );

    let cfi_supra_threshold = 0.701;
    assert_eq!(
        evaluate_output_mode(cfi_supra_threshold),
        AgentOutputMode::StructuredDeclarative
    );
}

#[test]
fn test_sovereign_family_office_vault_crud_and_sha256_tamper_evidence() {
    let vault = FamilyVault::open_in_memory().expect("Failed to open air-gapped in-memory vault");

    // 1. Generic Key-Value Store & Retrieval
    let kv_res = vault
        .store_kv("ejari_contract_2026", "{\"contract_no\":\"EJ-2026-991\",\"rent_aed\":120000}", Some("housing"))
        .expect("Failed to store KV");
    assert_eq!(kv_res.key, "ejari_contract_2026");
    assert!(!kv_res.sha256_checksum.is_empty());

    let fetched_kv = vault.get_kv("ejari_contract_2026").expect("Failed to get KV");
    assert_eq!(fetched_kv.sha256_checksum, kv_res.sha256_checksum);

    // 2. Family Identity CRUD
    let identity = FamilyIdentity {
        id: None,
        full_name: "Mohamed Fathy".to_string(),
        role: "head".to_string(),
        nationality: "EGY".to_string(),
        dob: "1988-05-15".to_string(),
        emirates_id: "784-1988-1234567-1".to_string(),
        passport_number: "A12345678".to_string(),
        visa_status: "golden_visa_10yr".to_string(),
        sha256_checksum: String::new(),
        created_at: String::new(),
        updated_at: String::new(),
    };
    vault.store_identity(&identity).expect("Failed to store family identity");

    let fetched_id = vault
        .get_identity_by_eid("784-1988-1234567-1")
        .expect("Failed to get identity by EID");
    assert_eq!(fetched_id.full_name, "Mohamed Fathy");
    assert_eq!(fetched_id.visa_status, "golden_visa_10yr");

    // 3. Passport CRUD
    let passport = PassportRecord {
        id: None,
        holder_name: "Mohamed Fathy".to_string(),
        passport_number: "A12345678".to_string(),
        issuing_country: "EGY".to_string(),
        expiry_date: "2032-05-15".to_string(),
        sha256_checksum: String::new(),
        created_at: String::new(),
    };
    vault.store_passport(&passport).expect("Failed to store passport");
    let fetched_pass = vault.get_passport("A12345678").expect("Failed to get passport");
    assert_eq!(fetched_pass.expiry_date, "2032-05-15");

    // 4. Emirates ID CRUD
    let eid = EmiratesIdRecord {
        id: None,
        holder_name: "Mohamed Fathy".to_string(),
        eid_number: "784-1988-1234567-1".to_string(),
        expiry_date: "2036-05-15".to_string(),
        card_number: "0123456789".to_string(),
        sha256_checksum: String::new(),
        created_at: String::new(),
    };
    vault.store_emirates_id(&eid).expect("Failed to store EID");
    let fetched_eid = vault.get_emirates_id("784-1988-1234567-1").expect("Failed to get EID");
    assert_eq!(fetched_eid.card_number, "0123456789");

    // 5. Ejari Contract CRUD
    let ejari = EjariContractRecord {
        id: None,
        contract_number: "EJ-2026-991".to_string(),
        tenant_name: "Mohamed Fathy".to_string(),
        landlord_name: "Emaar Properties PJSC".to_string(),
        property_address: "Downtown Dubai, Burj Crown Apt 1402".to_string(),
        annual_rent_aed: 120_000.0,
        start_date: "2026-01-01".to_string(),
        end_date: "2026-12-31".to_string(),
        premise_number: "123456789".to_string(),
        sha256_checksum: String::new(),
        created_at: String::new(),
    };
    vault.store_ejari_contract(&ejari).expect("Failed to store Ejari contract");
    let fetched_ejari = vault.get_ejari_contract("EJ-2026-991").expect("Failed to get Ejari contract");
    assert_eq!(fetched_ejari.annual_rent_aed, 120_000.0);

    // 6. Title Deed CRUD
    let title_deed = TitleDeedRecord {
        id: None,
        title_deed_number: "TD-2026-8877".to_string(),
        owner_name: "Mohamed Fathy".to_string(),
        property_type: "Apartment".to_string(),
        community: "Downtown Dubai".to_string(),
        purchase_value_aed: 2_500_000.0,
        is_mortgaged: false,
        sha256_checksum: String::new(),
        created_at: String::new(),
    };
    vault.store_title_deed(&title_deed).expect("Failed to store Title Deed");
    let fetched_deed = vault.get_title_deed("TD-2026-8877").expect("Failed to get Title Deed");
    assert_eq!(fetched_deed.purchase_value_aed, 2_500_000.0);

    // 7. KHDA Student Profile CRUD
    let student = KhdaStudentProfile {
        id: None,
        student_name: "Omar Fathy".to_string(),
        school_name: "Dubai College".to_string(),
        student_id: "KHDA-DC-2026-44".to_string(),
        grade: "Grade 6".to_string(),
        annual_tuition_aed: 65_000.0,
        dsib_rating: "Outstanding".to_string(),
        sha256_checksum: String::new(),
        created_at: String::new(),
    };
    vault.store_khda_profile(&student).expect("Failed to store KHDA profile");
    let fetched_student = vault.get_khda_profile("KHDA-DC-2026-44").expect("Failed to get KHDA profile");
    assert_eq!(fetched_student.dsib_rating, "Outstanding");

    // 8. DEWA Premise CRUD
    let dewa = DewaPremiseRecord {
        id: None,
        premise_number: "123456789".to_string(),
        account_number: "2000123456".to_string(),
        consumer_name: "Mohamed Fathy".to_string(),
        premise_type: "Apartment".to_string(),
        tariff_category: "residential".to_string(),
        sha256_checksum: String::new(),
        created_at: String::new(),
    };
    vault.store_dewa_premise(&dewa).expect("Failed to store DEWA premise");
    let fetched_dewa = vault.get_dewa_premise("123456789").expect("Failed to get DEWA premise");
    assert_eq!(fetched_dewa.tariff_category, "residential");

    // 9. Verify Checksums across all stored records
    let verif_results = vault.verify_all_kv().expect("Failed to run full verification scan");
    assert!(!verif_results.is_empty());
    assert!(verif_results.iter().all(|r| r.is_valid && !r.tampered));

    // 10. Adversarial Tamper Detection Verification:
    // Maliciously tamper with SQLite raw row data behind the scenes
    vault
        .simulate_tamper_kv("ejari_contract_2026", "{\"contract_no\":\"EJ-2026-991\",\"rent_aed\":999999}")
        .expect("Failed to simulate tamper");

    // Tampered record must fail verification and trigger TamperDetected error
    let tamper_get = vault.get_kv("ejari_contract_2026");
    assert!(
        matches!(tamper_get, Err(VaultError::TamperDetected { .. })),
        "Tampered record MUST trigger VaultError::TamperDetected"
    );

    let audit_res = vault.verify_kv("ejari_contract_2026").unwrap();
    assert!(!audit_res.is_valid);
    assert!(audit_res.tampered);

    // Also tamper Ejari rent field directly in SQL
    vault
        .simulate_tamper_ejari("EJ-2026-991", 500_000.0)
        .expect("Failed to simulate ejari tamper");
    let tampered_ejari_get = vault.get_ejari_contract("EJ-2026-991");
    assert!(
        matches!(tampered_ejari_get, Err(VaultError::TamperDetected { .. })),
        "Tampered Ejari record MUST trigger VaultError::TamperDetected"
    );
}

#[test]
fn test_c99_fademem_and_memory_store_ffi_linkage() {
    let mut store = C99MemoryStore::new();
    assert_eq!(store.count(), 0);

    let node1 = NibrasMemoryNode {
        id: 1,
        summary: [0; 128],
        valence: 1,
        initial_intensity: 0.95,
        current_intensity: 0.95,
        created_utc: 1_700_000_000,
        token_cost: 500,
        valence_inverted: false,
        is_pinned: true,
        state: TrajState::ActiveEpisodic,
        category: [0; 32],
        raw_arabic: [0; 256],
    };

    let id1 = store.add_node(&node1);
    assert_eq!(id1, 1);
    assert_eq!(store.count(), 1);

    let fetched = store.find_by_id(1);
    assert!(fetched.is_some());
    assert!(fetched.unwrap().is_pinned);

    // Step decay across C99 kernel
    let snap = store.step_decay(1_700_000_000 + 86_400 * 30, 86_400.0, 1.0, 259_200.0);
    assert_eq!(snap.distilled_rules_count, 1);

    // Verify pinned node in C99 kernel stayed distilled and intensity 1.0
    let node_after = store.find_by_id(1).unwrap();
    assert_eq!(node_after.state, TrajState::DistilledRule);
    assert_eq!(node_after.current_intensity, 1.0);
}
