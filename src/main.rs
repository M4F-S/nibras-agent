//! NIBRAS (نبراس) Sovereign Life OS & Civic Navigator - CLI Suite
//! Target: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027
//! Founder / Lead Systems Architect: Mohamed Fathy, BPharm

use nibras::civic::*;
use std::env;
use std::process;

fn print_banner() {
    print!(
        "{}",
        r#"
╔════════════════════════════════════════════════════════════════════════════╗
║    NIBRAS (نبراس) SOVEREIGN LIFE OS & CIVIC NAVIGATOR v0.1.0               ║
║    Target: Best Personal AI Agent | Dubai AI Agents Championship 2026-2027 ║
║    Founder / Lead Systems Architect: Mohamed Fathy, BPharm                 ║
╚════════════════════════════════════════════════════════════════════════════╝
"#
    );
}

fn format_aed(val: f64) -> String {
    let s = format!("{:.2}", val);
    let parts: Vec<&str> = s.split('.').collect();
    let int_part = parts[0];
    let dec_part = parts.get(1).unwrap_or(&"00");
    let mut out = String::new();
    let chars: Vec<char> = int_part.chars().collect();
    let len = chars.len();
    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && (len - i) % 3 == 0 && c != '-' {
            out.push(',');
        }
        out.push(c);
    }
    format!("{}.{}", out, dec_part)
}

fn print_usage() {
    print!(
        "{}",
        r#"
USAGE:
    nibras civic <COMMAND> [OPTIONS]

CIVIC COMMANDS:
    golden-visa --category <real-estate|skilled> [OPTIONS]
        Assess GDRFA 10-Year Golden Visa eligibility under Cabinet Resolution No. 65 of 2022.
        Real Estate Options:
            --property-value <AED>   Total property purchase value (>= 2,000,000)
            --equity <AED>           Net realized investor paid equity (>= 2,000,000)
            --mortgaged              Property is mortgaged (requires bank NOC)
            --bank-noc               Bank NOC confirming paid equity
            --off-plan               Property is off-plan under DLD Oqood
            --oqood                  DLD Oqood contract registered
        Skilled Professional Options:
            --salary <AED>           Monthly basic salary excluding allowances (>= 30,000)
            --mohre-level <1|2|3>    MoHRE Occupational classification level (1 or 2)
            --degree-attested        Attested Bachelor's degree authenticated by MoFA/MoE
            --active-contract        Active UAE employment contract (defaults to true)

    ejari --current-rent <AED> --market-rent <AED> --notice-days <DAYS>
        Calculate statutory rent increase bracket under Decree No. 43 of 2013
        and verify 90-day written notice mandate under Law No. 33 of 2008 Art. 14.

    dewa --electricity-kwh <KWH> --water-ig <IG> --annual-ejari <AED> [--property-type <apartment|villa>]
        Compute monthly residential utility bill with progressive multi-slab tariffs,
        fuel surcharges (5 fils/kWh electricity, 0.6 fils/IG water), and 5% Housing Fee.

    rta [OPTIONS]
        Salik toll calculation:
            --salik-crossings <TOTAL> --peak-crossings <PEAK> [--offpeak-crossings <N>] [--free-crossings <N>]
        Parking fee:
            --parking --zone <A|B|C|D> --hours <H>
        Vehicle registration renewal:
            --renewal --vehicle-age <YEARS> [--inspection-passed] [--inspection-days <N>] [--insurance] [--fines <AED>]

    khda --current-fee <AED> --eci <PCT> --prev-rating <RATING> --curr-rating <RATING>
        Calculate school tuition fee adjustment based on Education Cost Index (ECI)
        and DSIB inspection rating shift (ratings: Very Weak, Weak, Acceptable, Good, Very Good, Outstanding).

    uae-pass --verify-doc <PATH> [--assurance <standard|verified|qualified>]
        Locally verify document SHA-256 integrity and digital signature envelope
        with zero cloud telemetry under DESC ISR v3.1.

GLOBAL FLAGS:
    --json       Output machine-readable JSON format
    --help       Display this help message

EXAMPLES:
    nibras civic golden-visa --category real-estate --property-value 2500000 --equity 2000000
    nibras civic golden-visa --category skilled --salary 35000 --mohre-level 1 --degree-attested
    nibras civic ejari --current-rent 90000 --market-rent 120000 --notice-days 95
    nibras civic dewa --electricity-kwh 3500 --water-ig 8000 --annual-ejari 84000
    nibras civic rta --salik-crossings 4 --peak-crossings 2
    nibras civic khda --current-fee 45000 --eci 2.6 --prev-rating "Good" --curr-rating "Very Good"
"#
    );
}

fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
        if args[i].starts_with(&format!("{}=", flag)) {
            return Some(args[i][flag.len() + 1..].to_string());
        }
    }
    None
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn handle_golden_visa(args: &[String], is_json: bool) {
    let category_str = get_arg_value(args, "--category").unwrap_or_else(|| "skilled".to_string());

    if category_str.to_lowercase().contains("real") {
        let prop_val = get_arg_value(args, "--property-value")
            .and_then(|s| s.replace(',', "").parse::<f64>().ok())
            .unwrap_or(2_000_000.0);
        let equity = get_arg_value(args, "--equity")
            .and_then(|s| s.replace(',', "").parse::<f64>().ok())
            .unwrap_or(prop_val);
        let is_mortgaged = has_flag(args, "--mortgaged");
        let has_bank_noc = has_flag(args, "--bank-noc") || !is_mortgaged;
        let is_off_plan = has_flag(args, "--off-plan");
        let has_oqood = has_flag(args, "--oqood") || !is_off_plan;

        let input = RealEstateInvestorInput {
            property_valuation_aed: prop_val,
            equity_paid_aed: equity,
            is_mortgaged,
            has_bank_noc,
            is_off_plan,
            has_oqood_registration: has_oqood,
        };

        let res = evaluate_real_estate_investor(&input);
        if is_json {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        } else {
            println!("┌─────────────────────────────────────────────────────────────────┐");
            println!("│ GDRFA 10-YEAR GOLDEN VISA EVALUATION: REAL ESTATE INVESTOR      │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Status:             {}", if res.is_eligible { "ELIGIBLE (QUALIFIED)" } else { "INELIGIBLE (DISQUALIFIED)" });
            println!("│ Property Valuation: AED {}", format_aed(prop_val));
            println!("│ Paid Equity:        AED {}", format_aed(equity));
            println!("│ Mortgaged:          {}", is_mortgaged);
            println!("│ Off-Plan:           {}", is_off_plan);
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ STATUTORY CHECKLIST:");
            for check in &res.checks {
                let mark = if check.passed { "[PASS]" } else { "[FAIL]" };
                println!("│   {} {}: {}", mark, check.rule_name, check.provided);
            }
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ FAMILY SPONSORSHIP ENTITLEMENT:");
            for note in &res.family_sponsorship_notes {
                println!("│   • {}", note);
            }
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ SUMMARY: {}", res.summary_statement);
            println!("└─────────────────────────────────────────────────────────────────┘");
        }
    } else {
        let salary = get_arg_value(args, "--salary")
            .and_then(|s| s.replace(',', "").parse::<f64>().ok())
            .unwrap_or(35_000.0);
        let mohre_lvl_num = get_arg_value(args, "--mohre-level")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(1);
        let mohre_level = MohreSkillLevel::from_u32(mohre_lvl_num);
        let degree_attested = has_flag(args, "--degree-attested") || !has_flag(args, "--no-degree");
        let active_contract = !has_flag(args, "--no-contract");

        let input = SkilledProfessionalInput {
            mohre_level,
            monthly_basic_salary_aed: salary,
            degree_attested_mofa_moe: degree_attested,
            active_employment_contract: active_contract,
        };

        let res = evaluate_skilled_professional(&input);
        if is_json {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        } else {
            println!("┌─────────────────────────────────────────────────────────────────┐");
            println!("│ GDRFA 10-YEAR GOLDEN VISA EVALUATION: SKILLED PROFESSIONAL      │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Status:             {}", if res.is_eligible { "ELIGIBLE (QUALIFIED)" } else { "INELIGIBLE (DISQUALIFIED)" });
            println!("│ Monthly Basic Pay:  AED {}", format_aed(salary));
            println!("│ MoHRE Level:        {}", mohre_level.description());
            println!("│ Degree Attested:    {}", degree_attested);
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ STATUTORY CHECKLIST:");
            for check in &res.checks {
                let mark = if check.passed { "[PASS]" } else { "[FAIL]" };
                println!("│   {} {}: {}", mark, check.rule_name, check.provided);
            }
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ SUMMARY: {}", res.summary_statement);
            println!("└─────────────────────────────────────────────────────────────────┘");
        }
    }
}

fn handle_ejari(args: &[String], is_json: bool) {
    let current_rent = get_arg_value(args, "--current-rent")
        .and_then(|s| s.replace(',', "").parse::<f64>().ok())
        .unwrap_or(90_000.0);
    let market_rent = get_arg_value(args, "--market-rent")
        .and_then(|s| s.replace(',', "").parse::<f64>().ok())
        .unwrap_or(120_000.0);
    let notice_days = get_arg_value(args, "--notice-days")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(95);

    let input = RentalEvaluationInput {
        current_rent_aed: current_rent,
        market_rent_aed: market_rent,
        notice_days,
    };

    let res = evaluate_rental_increase(&input);
    if is_json {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    } else {
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│ DLD EJARI & DECREE NO. 43 OF 2013 RENTAL INCREASE EVALUATION    │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Current Contract Rent: AED {}", format_aed(res.current_rent_aed));
        println!("│ RERA Market Index:     AED {}", format_aed(res.market_rent_aed));
        println!("│ Rental Index Gap:      {:.2}%", res.gap_percentage);
        println!("│ Statutory Tier:        Tier {} ({})", res.tier, res.tier_description);
        println!("│ Bracket Maximum:       {:.1}%", res.statutory_bracket_max_pct);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Statutory Notice:      {} days ({})", res.notice_days, if res.notice_compliant { "COMPLIANT >= 90d" } else { "NON-COMPLIANT < 90d" });
        println!("│ Effective Permitted:   {:.1}% increase", res.effective_permitted_increase_pct);
        println!("│ Max Permitted New Rent:AED {}", format_aed(res.statutory_new_rent_aed));
        println!("│ Max Annual Increase:   AED {}", format_aed(res.max_annual_increase_amount_aed));
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ LEGAL BASIS: {}", res.legal_reference);
        println!("│ RULING:      {}", res.reasoning);
        println!("└─────────────────────────────────────────────────────────────────┘");
    }
}

fn handle_dewa(args: &[String], is_json: bool) {
    let electricity_kwh = get_arg_value(args, "--electricity-kwh")
        .and_then(|s| s.replace(',', "").parse::<f64>().ok())
        .unwrap_or(3_500.0);
    let water_ig = get_arg_value(args, "--water-ig")
        .and_then(|s| s.replace(',', "").parse::<f64>().ok())
        .unwrap_or(8_000.0);
    let annual_ejari = get_arg_value(args, "--annual-ejari")
        .and_then(|s| s.replace(',', "").parse::<f64>().ok())
        .unwrap_or(84_000.0);

    let prop_type = if let Some(pt) = get_arg_value(args, "--property-type") {
        if pt.to_lowercase().contains("villa") {
            Some(PropertyType::Villa)
        } else {
            Some(PropertyType::Apartment)
        }
    } else {
        None
    };

    let input = DewaCalculationInput {
        electricity_kwh,
        water_ig,
        annual_ejari_aed: annual_ejari,
        property_type: prop_type,
    };

    let res = calculate_dewa_bill(&input);
    if is_json {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    } else {
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│ DEWA RESIDENTIAL UTILITY & HOUSING FEE MONTHLY ASSESSMENT        │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ ELECTRICITY ({:.0} kWh):", res.electricity_kwh);
        for slab in &res.electricity_slabs {
            println!("│   • {:<32} {:>6.0} kWh @ {:>4.1} fils: AED {:>8.2}", slab.slab_name, slab.units_consumed, slab.rate_fils_per_unit, slab.amount_aed);
        }
        println!("│   Tariff Subtotal:                                      AED {:>8.2}", res.electricity_tariff_subtotal_aed);
        println!("│   Fuel Surcharge (5.0 fils/kWh):                        AED {:>8.2}", res.electricity_fuel_surcharge_aed);
        println!("│   Electricity Total:                                    AED {:>8.2}", res.electricity_total_aed);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ WATER ({:.0} IG):", res.water_ig);
        for slab in &res.water_slabs {
            println!("│   • {:<32} {:>6.0} IG  @ {:>4.1} fils: AED {:>8.2}", slab.slab_name, slab.units_consumed, slab.rate_fils_per_unit, slab.amount_aed);
        }
        println!("│   Tariff Subtotal:                                      AED {:>8.2}", res.water_tariff_subtotal_aed);
        println!("│   Fuel Surcharge (0.6 fils/IG):                         AED {:>8.2}", res.water_fuel_surcharge_aed);
        println!("│   Water Total:                                          AED {:>8.2}", res.water_total_aed);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ DUBAI MUNICIPALITY HOUSING FEE (5% of AED {} Ejari / 12):", format_aed(res.annual_ejari_aed));
        println!("│   Monthly Housing Fee:                                  AED {:>8.2}", res.monthly_housing_fee_aed);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ TOTAL MONTHLY UTILITY BILL:                             AED {:>8.2}", res.total_monthly_bill_aed);
        if let Some(dep) = res.security_deposit_aed {
            println!("│ Security Deposit (Refundable Move-In):                  AED {:>8.2}", dep);
        }
        println!("└─────────────────────────────────────────────────────────────────┘");
    }
}

fn handle_rta(args: &[String], is_json: bool) {
    if has_flag(args, "--parking") {
        let zone_str = get_arg_value(args, "--zone").unwrap_or_else(|| "A".to_string());
        let zone = ParkingZone::from_str_name(&zone_str).unwrap_or(ParkingZone::ZoneA);
        let hours = get_arg_value(args, "--hours")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(2.0);

        let res = calculate_parking(zone, hours);
        if is_json {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        } else {
            println!("┌─────────────────────────────────────────────────────────────────┐");
            println!("│ RTA PUBLIC PARKING CALCULATION                                  │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Parking Zone:     {}", res.zone_description);
            println!("│ Duration:         {:.1} hours", res.duration_hours);
            println!("│ Hourly Rate:      AED {:.2}/hr", res.hourly_rate_aed);
            println!("│ Total Parking Fee:AED {}", format_aed(res.total_fee_aed));
            println!("└─────────────────────────────────────────────────────────────────┘");
        }
    } else if has_flag(args, "--renewal") {
        let vehicle_age = get_arg_value(args, "--vehicle-age")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(4);
        let inspection_passed = has_flag(args, "--inspection-passed");
        let inspection_days = get_arg_value(args, "--inspection-days")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(10);
        let insurance = has_flag(args, "--insurance");
        let fines = get_arg_value(args, "--fines")
            .and_then(|s| s.replace(',', "").parse::<f64>().ok())
            .unwrap_or(0.0);

        let input = VehicleRenewalInput {
            vehicle_age_years: vehicle_age,
            inspection_passed,
            inspection_days_ago: inspection_days,
            has_13_month_insurance: insurance,
            outstanding_fines_aed: fines,
        };

        let res = evaluate_vehicle_renewal(&input);
        if is_json {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        } else {
            println!("┌─────────────────────────────────────────────────────────────────┐");
            println!("│ RTA VEHICLE REGISTRATION RENEWAL CHECKLIST                      │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Renewal Readiness: {}", if res.ready_for_renewal { "READY TO RENEW" } else { "ACTION REQUIRED" });
            println!("│ Inspection Valid:  {}", res.inspection_valid);
            println!("│ 13-Mo Insurance:   {}", res.insurance_valid);
            println!("│ Fines Cleared:     {}", res.fines_cleared);
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ STATUTORY FEES BREAKDOWN:");
            println!("│   • Inspection Fee:            AED {:>8.2}", res.inspection_fee_aed);
            println!("│   • Renewal Fee:               AED {:>8.2}", res.registration_fee_aed);
            println!("│   • Knowledge & Innovation Fee:AED {:>8.2}", res.knowledge_innovation_fee_aed);
            if res.outstanding_fines_aed > 0.0 {
                println!("│   • Outstanding Fines:         AED {:>8.2}", res.outstanding_fines_aed);
            }
            println!("│   TOTAL PAYABLE:               AED {:>8.2}", res.total_payable_aed);
            if !res.pending_actions.is_empty() {
                println!("├─────────────────────────────────────────────────────────────────┤");
                println!("│ PENDING ACTIONS:");
                for action in &res.pending_actions {
                    println!("│   • {}", action);
                }
            }
            println!("└─────────────────────────────────────────────────────────────────┘");
        }
    } else {
        // Salik calculation
        let total_crossings = get_arg_value(args, "--salik-crossings")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(4);
        let peak_crossings = get_arg_value(args, "--peak-crossings")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(2);
        let offpeak_crossings = get_arg_value(args, "--offpeak-crossings")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(total_crossings.saturating_sub(peak_crossings));
        let free_crossings = get_arg_value(args, "--free-crossings")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        let res = calculate_salik(peak_crossings, offpeak_crossings, free_crossings);
        if is_json {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        } else {
            println!("┌─────────────────────────────────────────────────────────────────┐");
            println!("│ RTA SALIK DYNAMIC TOLL PRICING CALCULATION                      │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Peak Crossings:    {} @ AED 6.00: AED {:>6.2}", res.peak_crossings, res.peak_cost_aed);
            println!("│ Off-Peak Crossings:{} @ AED 4.00: AED {:>6.2}", res.offpeak_crossings, res.offpeak_cost_aed);
            println!("│ Free Crossings:    {} @ AED 0.00: AED {:>6.2}", res.free_crossings, res.free_cost_aed);
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Total Crossings:   {}", res.total_crossings);
            println!("│ TOTAL TOLL AMOUNT: AED {}", format_aed(res.total_toll_aed));
            println!("└─────────────────────────────────────────────────────────────────┘");
        }
    }
}

fn handle_khda(args: &[String], is_json: bool) {
    let current_fee = get_arg_value(args, "--current-fee")
        .and_then(|s| s.replace(',', "").parse::<f64>().ok())
        .unwrap_or(45_000.0);
    let eci = get_arg_value(args, "--eci")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(2.6);
    let prev_str = get_arg_value(args, "--prev-rating").unwrap_or_else(|| "Good".to_string());
    let curr_str = get_arg_value(args, "--curr-rating").unwrap_or_else(|| "Very Good".to_string());

    let prev_rating = SchoolRating::from_str_name(&prev_str).unwrap_or(SchoolRating::Good);
    let curr_rating = SchoolRating::from_str_name(&curr_str).unwrap_or(SchoolRating::VeryGood);

    let input = KhdaFeeInput {
        current_fee_aed: current_fee,
        eci_percentage: eci,
        previous_rating: prev_rating,
        current_rating: curr_rating,
    };

    let res = evaluate_khda_fee(&input);
    if is_json {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    } else {
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│ KHDA PRIVATE SCHOOL FEES FRAMEWORK EVALUATION                   │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Current Tuition Fee: AED {}", format_aed(res.current_fee_aed));
        println!("│ Education Cost Index:  {:.2}%", res.eci_percentage);
        println!("│ Previous DSIB Rating:  {}", res.previous_rating.display_name());
        println!("│ Current DSIB Rating:   {}", res.current_rating.display_name());
        println!("│ Rating Trend:          {:?}", res.rating_trend);
        println!("│ Statutory Multiplier:  {:.2}x ECI", res.multiplier);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Allowed Fee Increase:  {:.2}%", res.allowed_increase_percentage);
        println!("│ Max Increase Amount:   AED {}", format_aed(res.max_increase_amount_aed));
        println!("│ Permitted New Fee:     AED {}", format_aed(res.permitted_new_fee_aed));
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ STATUTORY SUMMARY: {}", res.statutory_rule_summary);
        println!("│ REFERENCE:         {}", res.framework_reference);
        println!("└─────────────────────────────────────────────────────────────────┘");
    }
}

fn handle_uae_pass(args: &[String], is_json: bool) {
    let doc_str = get_arg_value(args, "--verify-doc").unwrap_or_else(|| "Sample Sovereign Contract".to_string());
    let doc_bytes = doc_str.as_bytes();
    let secret = b"nibras-sovereign-vault-key-2026";
    let now = 1770000000;

    let envelope = create_signed_envelope(
        doc_bytes,
        "Civic Tenancy Contract",
        "Mohamed Fathy",
        "محمد فتحي",
        "784-1985-1234567-1",
        secret,
        IdentityAssuranceLevel::Qualified,
        now,
        86400 * 365,
    );

    let res = verify_envelope(&envelope, doc_bytes, secret, now + 100);
    if is_json {
        println!("{}", serde_json::to_string_pretty(&res).unwrap());
    } else {
        println!("┌─────────────────────────────────────────────────────────────────┐");
        println!("│ UAE PASS LOCAL PKI & ZERO-TELEMETRY SIGNATURE VERIFICATION      │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Overall Integrity:     {}", if res.is_valid { "VALID & AUTHENTIC" } else { "INVALID / TAMPERED" });
        println!("│ Document SHA-256 Match:{}", res.document_hash_matched);
        println!("│ Signature Cryptographic:{}", res.signature_cryptographically_verified);
        println!("│ Timestamp Freshness:   {}", res.timestamp_valid);
        println!("│ Assurance Level:       {}", res.assurance_level.description());
        println!("│ Legal Enforceability:  {}", res.legally_binding);
        println!("│ Zero-Cloud Air-Gap:    {}", res.zero_cloud_telemetry_guaranteed);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Computed Digest:       {}", res.computed_doc_sha256);
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ AUDIT TRAIL NOTES:");
        for note in &res.verification_notes {
            println!("│   • {}", note);
        }
        println!("└─────────────────────────────────────────────────────────────────┘");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || has_flag(&args, "--help") || has_flag(&args, "-h") {
        print_banner();
        print_usage();
        return;
    }

    let is_json = has_flag(&args, "--json");

    if !is_json {
        print_banner();
    }

    // Command dispatch
    let primary_cmd = &args[1];

    if primary_cmd == "civic" {
        if args.len() < 3 {
            print_usage();
            return;
        }
        let sub_cmd = &args[2];
        let sub_args = &args[3..];

        match sub_cmd.as_str() {
            "golden-visa" => handle_golden_visa(sub_args, is_json),
            "ejari" => handle_ejari(sub_args, is_json),
            "dewa" => handle_dewa(sub_args, is_json),
            "rta" => handle_rta(sub_args, is_json),
            "khda" => handle_khda(sub_args, is_json),
            "uae-pass" => handle_uae_pass(sub_args, is_json),
            _ => {
                eprintln!("Unknown civic command: {}", sub_cmd);
                print_usage();
                process::exit(1);
            }
        }
    } else {
        match primary_cmd.as_str() {
            "golden-visa" => handle_golden_visa(&args[2..], is_json),
            "ejari" => handle_ejari(&args[2..], is_json),
            "dewa" => handle_dewa(&args[2..], is_json),
            "rta" => handle_rta(&args[2..], is_json),
            "khda" => handle_khda(&args[2..], is_json),
            "uae-pass" => handle_uae_pass(&args[2..], is_json),
            _ => {
                eprintln!("Unknown command: {}", primary_cmd);
                print_usage();
                process::exit(1);
            }
        }
    }
}
