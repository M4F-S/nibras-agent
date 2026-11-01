//! Comprehensive Civic Navigator Automated Test Suite
//! Milestone M-07 (Sprint SPRINT-07)
//! Canonical Master Specification §4: AGENT 3: NIBRAS

use nibras::civic::*;
use nibras::safety::*;

// ============================================================================
// 1. GDRFA 10-YEAR GOLDEN VISA TESTS (§4.1.1)
// ============================================================================

#[test]
fn test_golden_visa_real_estate_investor_qualification_boundary() {
    // Exact threshold: AED 2,000,000
    let qualified = RealEstateInvestorInput {
        property_valuation_aed: 2_000_000.0,
        equity_paid_aed: 2_000_000.0,
        is_mortgaged: false,
        has_bank_noc: true,
        is_off_plan: false,
        has_oqood_registration: true,
    };
    let assessment = evaluate_real_estate_investor(&qualified);
    assert!(assessment.is_eligible, "Expected qualification at exact AED 2,000,000 boundary");
    assert_eq!(assessment.category, GoldenVisaCategory::RealEstateInvestor);
    assert!(assessment.checks.iter().all(|c| c.passed));

    // Boundary failure: AED 1,999,999.99 equity
    let boundary_fail = RealEstateInvestorInput {
        property_valuation_aed: 2_500_000.0,
        equity_paid_aed: 1_999_999.99,
        is_mortgaged: false,
        has_bank_noc: true,
        is_off_plan: false,
        has_oqood_registration: true,
    };
    let fail_assessment = evaluate_real_estate_investor(&boundary_fail);
    assert!(!fail_assessment.is_eligible, "Expected disqualification when equity < AED 2M");
    assert!(fail_assessment.summary_statement.contains("DISQUALIFIED"));
}

#[test]
fn test_golden_visa_real_estate_mortgaged_and_offplan() {
    // Mortgaged with bank NOC confirming >= AED 2M paid equity
    let mortgaged_ok = RealEstateInvestorInput {
        property_valuation_aed: 3_000_000.0,
        equity_paid_aed: 2_100_000.0,
        is_mortgaged: true,
        has_bank_noc: true,
        is_off_plan: false,
        has_oqood_registration: true,
    };
    assert!(evaluate_real_estate_investor(&mortgaged_ok).is_eligible);

    // Mortgaged without bank NOC
    let mortgaged_missing_noc = RealEstateInvestorInput {
        property_valuation_aed: 3_000_000.0,
        equity_paid_aed: 2_100_000.0,
        is_mortgaged: true,
        has_bank_noc: false,
        is_off_plan: false,
        has_oqood_registration: true,
    };
    assert!(!evaluate_real_estate_investor(&mortgaged_missing_noc).is_eligible);

    // Off-plan with DLD Oqood
    let offplan_ok = RealEstateInvestorInput {
        property_valuation_aed: 2_400_000.0,
        equity_paid_aed: 2_000_000.0,
        is_mortgaged: false,
        has_bank_noc: true,
        is_off_plan: true,
        has_oqood_registration: true,
    };
    assert!(evaluate_real_estate_investor(&offplan_ok).is_eligible);

    // Off-plan without Oqood
    let offplan_missing = RealEstateInvestorInput {
        property_valuation_aed: 2_400_000.0,
        equity_paid_aed: 2_000_000.0,
        is_mortgaged: false,
        has_bank_noc: true,
        is_off_plan: true,
        has_oqood_registration: false,
    };
    assert!(!evaluate_real_estate_investor(&offplan_missing).is_eligible);
}

#[test]
fn test_golden_visa_skilled_professional_boundaries() {
    // Exact salary boundary: AED 30,000.00
    let qualified_boundary = SkilledProfessionalInput {
        mohre_level: MohreSkillLevel::Level2Professionals,
        monthly_basic_salary_aed: 30_000.0,
        degree_attested_mofa_moe: true,
        active_employment_contract: true,
    };
    assert!(evaluate_skilled_professional(&qualified_boundary).is_eligible);

    // 1 AED below basic salary boundary: AED 29,999.00
    let salary_fail = SkilledProfessionalInput {
        mohre_level: MohreSkillLevel::Level1Managers,
        monthly_basic_salary_aed: 29_999.0,
        degree_attested_mofa_moe: true,
        active_employment_contract: true,
    };
    let res = evaluate_skilled_professional(&salary_fail);
    assert!(!res.is_eligible);
    assert!(res.checks.iter().any(|c| c.rule_name.contains("Salary") && !c.passed));

    // MoHRE Level 3 (Technician) is ineligible
    let level_fail = SkilledProfessionalInput {
        mohre_level: MohreSkillLevel::Level3Technicians,
        monthly_basic_salary_aed: 45_000.0,
        degree_attested_mofa_moe: true,
        active_employment_contract: true,
    };
    assert!(!evaluate_skilled_professional(&level_fail).is_eligible);

    // Missing degree attestation
    let degree_fail = SkilledProfessionalInput {
        mohre_level: MohreSkillLevel::Level1Managers,
        monthly_basic_salary_aed: 40_000.0,
        degree_attested_mofa_moe: false,
        active_employment_contract: true,
    };
    assert!(!evaluate_skilled_professional(&degree_fail).is_eligible);
}

#[test]
fn test_golden_visa_family_sponsorship_rules() {
    // Spouse: always eligible, 10 years
    let spouse = FamilyMember {
        relation: FamilyRelation::Spouse,
        age: 38,
        is_married: true,
        is_student: false,
        is_person_of_determination: false,
    };
    let s_eval = assess_family_member(&spouse);
    assert!(s_eval.eligible);
    assert_eq!(s_eval.visa_duration_years, 10);

    // Unmarried daughter of any age (e.g. 29)
    let daughter_unmarried = FamilyMember {
        relation: FamilyRelation::Daughter,
        age: 29,
        is_married: false,
        is_student: false,
        is_person_of_determination: false,
    };
    assert!(assess_family_member(&daughter_unmarried).eligible);

    // Married daughter (ineligible under parent)
    let daughter_married = FamilyMember {
        relation: FamilyRelation::Daughter,
        age: 24,
        is_married: true,
        is_student: false,
        is_person_of_determination: false,
    };
    assert!(!assess_family_member(&daughter_married).eligible);

    // Son age 24 (<= 25)
    let son_24 = FamilyMember {
        relation: FamilyRelation::Son,
        age: 24,
        is_married: false,
        is_student: false,
        is_person_of_determination: false,
    };
    assert!(assess_family_member(&son_24).eligible);

    // Son age 27 without exemption (ineligible)
    let son_27 = FamilyMember {
        relation: FamilyRelation::Son,
        age: 27,
        is_married: false,
        is_student: false,
        is_person_of_determination: false,
    };
    assert!(!assess_family_member(&son_27).eligible);

    // Son age 28 with Person of Determination exemption
    let son_pod = FamilyMember {
        relation: FamilyRelation::Son,
        age: 28,
        is_married: false,
        is_student: false,
        is_person_of_determination: true,
    };
    assert!(assess_family_member(&son_pod).eligible);

    // Parents (1-year renewable)
    let parent = FamilyMember {
        relation: FamilyRelation::Parent,
        age: 65,
        is_married: true,
        is_student: false,
        is_person_of_determination: false,
    };
    let p_eval = assess_family_member(&parent);
    assert!(p_eval.eligible);
    assert_eq!(p_eval.visa_duration_years, 1);
}

// ============================================================================
// 2. DLD EJARI & DECREE NO. 43 OF 2013 RENTAL INCREASE TESTS (§4.1.2)
// ============================================================================

#[test]
fn test_ejari_5_tier_bracket_exact_percentages() {
    // Current rent >= Market rent (Gap = 0%) -> Tier 1 (0% increase)
    let t1_neg = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 100_000.0,
        market_rent_aed: 95_000.0,
        notice_days: 90,
    });
    assert_eq!(t1_neg.tier, 1);
    assert_eq!(t1_neg.effective_permitted_increase_pct, 0.0);

    // Tier 1: Gap = 10.0% -> 0% increase (Decree 43/2013 Art. 1a)
    // 90,000 / 100,000 = 10% gap
    let t1 = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 90_000.0,
        market_rent_aed: 100_000.0,
        notice_days: 90,
    });
    assert_eq!(t1.tier, 1);
    assert_eq!(t1.statutory_bracket_max_pct, 0.0);
    assert_eq!(t1.statutory_new_rent_aed, 90_000.0);

    // Tier 2: Gap = 15.0% (11% - 20%) -> 5% increase (Decree 43/2013 Art. 1b)
    // 85,000 / 100,000 = 15% gap
    let t2 = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 85_000.0,
        market_rent_aed: 100_000.0,
        notice_days: 90,
    });
    assert_eq!(t2.tier, 2);
    assert_eq!(t2.statutory_bracket_max_pct, 5.0);
    assert_eq!(t2.statutory_new_rent_aed, 89_250.0);

    // Tier 3: Gap = 25.0% (21% - 30%) -> 10% increase (Decree 43/2013 Art. 1c)
    // 90,000 / 120,000 = 25% gap
    let t3 = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 90_000.0,
        market_rent_aed: 120_000.0,
        notice_days: 95,
    });
    assert_eq!(t3.tier, 3);
    assert_eq!(t3.statutory_bracket_max_pct, 10.0);
    assert_eq!(t3.statutory_new_rent_aed, 99_000.0);
    assert_eq!(t3.max_annual_increase_amount_aed, 9_000.0);

    // Tier 4: Gap = 35.0% (31% - 40%) -> 15% increase (Decree 43/2013 Art. 1d)
    // 65,000 / 100,000 = 35% gap
    let t4 = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 65_000.0,
        market_rent_aed: 100_000.0,
        notice_days: 90,
    });
    assert_eq!(t4.tier, 4);
    assert_eq!(t4.statutory_bracket_max_pct, 15.0);
    assert_eq!(t4.statutory_new_rent_aed, 74_750.0);

    // Tier 5: Gap = 45.0% (> 40%) -> 20% increase (Decree 43/2013 Art. 1e)
    // 55,000 / 100,000 = 45% gap
    let t5 = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 55_000.0,
        market_rent_aed: 100_000.0,
        notice_days: 90,
    });
    assert_eq!(t5.tier, 5);
    assert_eq!(t5.statutory_bracket_max_pct, 20.0);
    assert_eq!(t5.statutory_new_rent_aed, 66_000.0);
}

#[test]
fn test_ejari_90_day_statutory_notice_invalidation() {
    // 89 days notice served (1 day short of 90-day mandate)
    // Even though Gap = 30% (Tier 3: 10% allowed), the increase is rendered null and void!
    let invalid_notice = evaluate_rental_increase(&RentalEvaluationInput {
        current_rent_aed: 70_000.0,
        market_rent_aed: 100_000.0,
        notice_days: 89,
    });
    assert_eq!(invalid_notice.tier, 3);
    assert_eq!(invalid_notice.statutory_bracket_max_pct, 10.0);
    assert!(!invalid_notice.notice_compliant);
    assert_eq!(invalid_notice.effective_permitted_increase_pct, 0.0);
    assert_eq!(invalid_notice.statutory_new_rent_aed, 70_000.0);
    assert_eq!(invalid_notice.max_annual_increase_amount_aed, 0.0);
    assert!(invalid_notice.legal_reference.contains("Law No. 33 of 2008, Article 14"));
    assert!(invalid_notice.reasoning.contains("null and void"));
}

// ============================================================================
// 3. DEWA RESIDENTIAL UTILITY SLAB TESTS (§4.1.3)
// ============================================================================

#[test]
fn test_dewa_electricity_progressive_slabs() {
    // 1,500 kWh: All in Green slab (1500 * 0.23 = 345.00)
    let (slabs_1, cost_1) = calculate_electricity_slabs(1500.0);
    assert_eq!(slabs_1.len(), 1);
    assert_eq!(cost_1, 345.00);

    // 3,500 kWh:
    // Green:  2000 * 0.23 = 460.00
    // Yellow: 1500 * 0.28 = 420.00
    // Subtotal = 880.00
    let (slabs_2, cost_2) = calculate_electricity_slabs(3500.0);
    assert_eq!(slabs_2.len(), 2);
    assert_eq!(cost_2, 880.00);

    // 5,000 kWh:
    // Green:  2000 * 0.23 = 460.00
    // Yellow: 2000 * 0.28 = 560.00
    // Orange: 1000 * 0.32 = 320.00
    // Subtotal = 1340.00
    let (slabs_3, cost_3) = calculate_electricity_slabs(5000.0);
    assert_eq!(slabs_3.len(), 3);
    assert_eq!(cost_3, 1340.00);

    // 7,000 kWh:
    // Green:  2000 * 0.23 = 460.00
    // Yellow: 2000 * 0.28 = 560.00
    // Orange: 2000 * 0.32 = 640.00
    // Red:    1000 * 0.38 = 380.00
    // Subtotal = 2040.00
    let (slabs_4, cost_4) = calculate_electricity_slabs(7000.0);
    assert_eq!(slabs_4.len(), 4);
    assert_eq!(cost_4, 2040.00);
}

#[test]
fn test_dewa_water_progressive_slabs() {
    // 8,000 IG:
    // Green:  6000 * 0.035 = 210.00
    // Yellow: 2000 * 0.040 = 80.00
    // Subtotal = 290.00
    let (slabs, cost) = calculate_water_slabs(8000.0);
    assert_eq!(slabs.len(), 2);
    assert_eq!(cost, 290.00);

    // 15,000 IG:
    // Green:  6000 * 0.035 = 210.00
    // Yellow: 6000 * 0.040 = 240.00
    // Red:    3000 * 0.046 = 138.00
    // Subtotal = 588.00
    let (_, cost_high) = calculate_water_slabs(15000.0);
    assert_eq!(cost_high, 588.00);
}

#[test]
fn test_dewa_full_monthly_bill_with_fuel_and_housing() {
    // Electricity: 3500 kWh -> Subtotal: 880.00, Fuel: 3500 * 0.05 = 175.00 -> Total: 1055.00
    // Water: 8000 IG -> Subtotal: 290.00, Fuel: 8000 * 0.006 = 48.00 -> Total: 338.00
    // Housing fee: 84,000 * 0.05 / 12 = 350.00
    // Total monthly bill: 1055.00 + 338.00 + 350.00 = 1743.00
    let input = DewaCalculationInput {
        electricity_kwh: 3500.0,
        water_ig: 8000.0,
        annual_ejari_aed: 84_000.0,
        property_type: Some(PropertyType::Apartment),
    };

    let bill = calculate_dewa_bill(&input);
    assert_eq!(bill.electricity_tariff_subtotal_aed, 880.00);
    assert_eq!(bill.electricity_fuel_surcharge_aed, 175.00);
    assert_eq!(bill.electricity_total_aed, 1055.00);

    assert_eq!(bill.water_tariff_subtotal_aed, 290.00);
    assert_eq!(bill.water_fuel_surcharge_aed, 48.00);
    assert_eq!(bill.water_total_aed, 338.00);

    assert_eq!(bill.monthly_housing_fee_aed, 350.00);
    assert_eq!(bill.total_monthly_bill_aed, 1743.00);
    assert_eq!(bill.security_deposit_aed, Some(2000.0));
}

// ============================================================================
// 4. RTA SMART MOBILITY TESTS (§4.1.4)
// ============================================================================

#[test]
fn test_rta_salik_dynamic_pricing() {
    // 2 peak @ 6.00 = 12.00, 2 off-peak @ 4.00 = 8.00, 1 free @ 0.00 = 0.00 -> 20.00 AED
    let salik = calculate_salik(2, 2, 1);
    assert_eq!(salik.total_crossings, 5);
    assert_eq!(salik.peak_cost_aed, 12.00);
    assert_eq!(salik.offpeak_cost_aed, 8.00);
    assert_eq!(salik.free_cost_aed, 0.00);
    assert_eq!(salik.total_toll_aed, 20.00);
}

#[test]
fn test_rta_parking_zone_rates() {
    assert_eq!(calculate_parking(ParkingZone::ZoneA, 3.0).total_fee_aed, 12.00);
    assert_eq!(calculate_parking(ParkingZone::ZoneB, 4.0).total_fee_aed, 12.00);
    assert_eq!(calculate_parking(ParkingZone::ZoneC, 5.0).total_fee_aed, 10.00);
    assert_eq!(calculate_parking(ParkingZone::ZoneD, 2.5).total_fee_aed, 5.00);
}

#[test]
fn test_rta_vehicle_registration_renewal_readiness() {
    // Vehicle age 4, valid inspection 10 days ago, 13-mo insurance, 0 fines
    let ready = evaluate_vehicle_renewal(&VehicleRenewalInput {
        vehicle_age_years: 4,
        inspection_passed: true,
        inspection_days_ago: 10,
        has_13_month_insurance: true,
        outstanding_fines_aed: 0.0,
    });
    assert!(ready.ready_for_renewal);
    assert_eq!(ready.total_payable_aed, 540.00); // 170 + 350 + 20

    // Stale inspection (> 30 days)
    let stale_insp = evaluate_vehicle_renewal(&VehicleRenewalInput {
        vehicle_age_years: 4,
        inspection_passed: true,
        inspection_days_ago: 35,
        has_13_month_insurance: true,
        outstanding_fines_aed: 0.0,
    });
    assert!(!stale_insp.ready_for_renewal);
    assert!(!stale_insp.inspection_valid);
}

// ============================================================================
// 5. KHDA PRIVATE SCHOOL FEES TESTS (§4.1.5)
// ============================================================================

#[test]
fn test_khda_rating_drop_strict_fee_freeze() {
    // Rating dropped from Outstanding to Very Good: Strict 0% freeze
    let dropped = evaluate_khda_fee(&KhdaFeeInput {
        current_fee_aed: 50_000.0,
        eci_percentage: 2.6,
        previous_rating: SchoolRating::Outstanding,
        current_rating: SchoolRating::VeryGood,
    });
    assert_eq!(dropped.rating_trend, RatingTrend::Dropped);
    assert_eq!(dropped.multiplier, 0.0);
    assert_eq!(dropped.allowed_increase_percentage, 0.0);
    assert_eq!(dropped.permitted_new_fee_aed, 50_000.0);
    assert!(dropped.statutory_rule_summary.contains("Strict 0% fee freeze"));
}

#[test]
fn test_khda_rating_maintained_and_improvements() {
    // Maintained rating (Good -> Good): 1.0 * ECI (2.6%)
    let maintained = evaluate_khda_fee(&KhdaFeeInput {
        current_fee_aed: 40_000.0,
        eci_percentage: 2.6,
        previous_rating: SchoolRating::Good,
        current_rating: SchoolRating::Good,
    });
    assert_eq!(maintained.rating_trend, RatingTrend::Maintained);
    assert_eq!(maintained.multiplier, 1.0);
    assert_eq!(maintained.allowed_increase_percentage, 2.6);
    assert_eq!(maintained.permitted_new_fee_aed, 41_040.0);

    // Improved (Good -> Very Good): 1.75 * ECI (1.75 * 2.6 = 4.55%)
    let good_to_vg = evaluate_khda_fee(&KhdaFeeInput {
        current_fee_aed: 45_000.0,
        eci_percentage: 2.6,
        previous_rating: SchoolRating::Good,
        current_rating: SchoolRating::VeryGood,
    });
    assert_eq!(good_to_vg.rating_trend, RatingTrend::Improved);
    assert_eq!(good_to_vg.multiplier, 1.75);
    assert_eq!(good_to_vg.allowed_increase_percentage, 4.55);
    assert_eq!(good_to_vg.permitted_new_fee_aed, 47_047.50);

    // Improved (Very Good -> Outstanding): 1.5 * ECI (1.5 * 2.6 = 3.90%)
    let vg_to_out = evaluate_khda_fee(&KhdaFeeInput {
        current_fee_aed: 60_000.0,
        eci_percentage: 2.6,
        previous_rating: SchoolRating::VeryGood,
        current_rating: SchoolRating::Outstanding,
    });
    assert_eq!(vg_to_out.multiplier, 1.5);
    assert_eq!(vg_to_out.allowed_increase_percentage, 3.90);
    assert_eq!(vg_to_out.permitted_new_fee_aed, 62_340.00);

    // Improved (Acceptable -> Good): 2.0 * ECI (2.0 * 2.6 = 5.20%)
    let acc_to_good = evaluate_khda_fee(&KhdaFeeInput {
        current_fee_aed: 30_000.0,
        eci_percentage: 2.6,
        previous_rating: SchoolRating::Acceptable,
        current_rating: SchoolRating::Good,
    });
    assert_eq!(acc_to_good.multiplier, 2.0);
    assert_eq!(acc_to_good.allowed_increase_percentage, 5.20);
    assert_eq!(acc_to_good.permitted_new_fee_aed, 31_560.00);
}

// ============================================================================
// 6. UAE PASS PKI & DIGITAL SIGNATURE TESTS (§4.1.6)
// ============================================================================

#[test]
fn test_uae_pass_local_cryptographic_verification() {
    let doc_content = b"Official Ejari Tenancy Contract - Unit 101, Dubai Marina, Dubai, UAE";
    let secret = b"nibras-test-secret-2026";
    let now = 1770000000;

    let envelope = create_signed_envelope(
        doc_content,
        "Tenancy Contract",
        "Mohamed Fathy",
        "محمد فتحي",
        "784-1985-1234567-1",
        secret,
        IdentityAssuranceLevel::Qualified,
        now,
        86400,
    );

    // Valid verification
    let res = verify_envelope(&envelope, doc_content, secret, now + 100);
    assert!(res.is_valid);
    assert!(res.document_hash_matched);
    assert!(res.signature_cryptographically_verified);
    assert!(res.timestamp_valid);
    assert!(res.legally_binding);
    assert!(res.zero_cloud_telemetry_guaranteed);

    // Tampered document detection
    let tampered_content = b"Tampered Ejari Tenancy Contract - Unit 999";
    let tampered_res = verify_envelope(&envelope, tampered_content, secret, now + 100);
    assert!(!tampered_res.is_valid);
    assert!(!tampered_res.document_hash_matched);

    // Expired timestamp
    let expired_res = verify_envelope(&envelope, doc_content, secret, now + 90000);
    assert!(!expired_res.is_valid);
    assert!(!expired_res.timestamp_valid);
}

// ============================================================================
// 7. C99 FFI SAFETY, GROUNDING & MEMORY LINKAGE TESTS
// ============================================================================

#[test]
fn test_c99_arabic_normalizer_via_ffi() {
    let raw = "مُدِيرُ الْمَشْرُوعِ فِي دُبَيّــــــ كَانَ هُنَا";
    let normalized = normalize_arabic(raw);
    assert!(normalized.contains("مدير"));
    assert!(normalized.contains("المشروع"));
    assert!(normalized.contains("دبي"));
    assert!(!normalized.contains("ـ")); // Tatweel stripped
}

#[test]
fn test_c99_arabic_entity_extraction_via_ffi() {
    let text = "اجتماع مع دبي للعقارات غدا الساعه 10 صباحا";
    let normalized = normalize_arabic(text);
    let info = extract_arabic_info(&normalized);
    assert!(info.is_appointment || !info.entities.is_empty());
}

#[test]
fn test_c99_grounding_shield_claim_verification() {
    let source = SourceSnippet {
        source_id: "DECREE_43_2013".to_string(),
        source_title: "Dubai Decree No. 43 of 2013 on Rental Increases".to_string(),
        snippet_text: "Maximum permitted rent increase is 20.0 percent when market rent gap exceeds 40 percent.".to_string(),
        source_numerical: 20.0,
        doc_id: "DECREE-43-2013".to_string(),
        section: "Article 1(e)".to_string(),
        page: 2,
        passage_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
    };

    // Accurate claim matching source numerical 20.0
    let verified = verify_grounding_claim(
        "Decree 43 allows maximum 20.0 percent rent increase",
        "DECREE_43_2013",
        Some(20.0),
        &[source.clone()],
    );
    assert_eq!(verified.verdict, GroundingVerdict::Verified);

    // Hallucinated claim with numerical 35.0 (discrepancy)
    let hallucination = verify_grounding_claim(
        "Decree 43 allows maximum 35.0 percent rent increase",
        "DECREE_43_2013",
        Some(35.0),
        &[source],
    );
    assert_eq!(hallucination.verdict, GroundingVerdict::HallucinationTripped);
}

#[test]
fn test_c99_pii_redactor_via_ffi() {
    let prompt = "Patient Emirates ID 784-1985-1234567-1 phone +971501234567 requested Ejari renewal";
    let redacted = redact_pii(prompt);
    assert!(!redacted.contains("784-1985-1234567-1"));
    assert!(!redacted.contains("+971501234567"));
    assert!(redacted.contains("Ejari renewal"));
}

#[test]
fn test_c99_personal_rag_in_memory() {
    let mut rag = PersonalRag::new(":memory:").expect("Failed to initialize in-memory RAG");
    let indexed = rag.index_note(
        1,
        "Dubai Tenancy Decree 43",
        "The rental increase bracket in Dubai is governed by Decree No. 43 of 2013.\nLine 2: Statutory 90 days notice is required under Law 33 of 2008.\nLine 3: Housing fee is 5 percent of annual rent divided across 12 months.",
    );
    assert!(indexed > 0, "Expected at least 1 indexed chunk");

    let results = rag.retrieve("housing fee 5 percent", 5);
    assert!(!results.is_empty(), "Expected retrieval results for housing fee query");
}
