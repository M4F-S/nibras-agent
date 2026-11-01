//! DLD Ejari & Decree No. 43 of 2013 Rental Increase Bracket Engine
//! Implements Decree No. 43 of 2013 and Law No. 33 of 2008 (Amending Law No. 26 of 2007).
//! Canonical Master Specification §4.1.2.

use serde::{Deserialize, Serialize};

pub const STATUTORY_NOTICE_DAYS_MANDATE: u32 = 90;

/// Rental Increase Tier per Decree No. 43 of 2013 Art. 1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RentalTier {
    Tier1NoIncrease = 1,       // Gap <= 10%: 0% increase (Art. 1a)
    Tier2FivePercent = 2,      // 11% <= Gap <= 20%: 5% max increase (Art. 1b)
    Tier3TenPercent = 3,       // 21% <= Gap <= 30%: 10% max increase (Art. 1c)
    Tier4FifteenPercent = 4,   // 31% <= Gap <= 40%: 15% max increase (Art. 1d)
    Tier5TwentyPercent = 5,    // Gap > 40%: 20% max increase (Art. 1e)
}

impl RentalTier {
    pub fn max_increase_percentage(&self) -> f64 {
        match self {
            RentalTier::Tier1NoIncrease => 0.0,
            RentalTier::Tier2FivePercent => 5.0,
            RentalTier::Tier3TenPercent => 10.0,
            RentalTier::Tier4FifteenPercent => 15.0,
            RentalTier::Tier5TwentyPercent => 20.0,
        }
    }

    pub fn legal_clause(&self) -> &'static str {
        match self {
            RentalTier::Tier1NoIncrease => "Decree No. 43 of 2013, Article 1(a)",
            RentalTier::Tier2FivePercent => "Decree No. 43 of 2013, Article 1(b)",
            RentalTier::Tier3TenPercent => "Decree No. 43 of 2013, Article 1(c)",
            RentalTier::Tier4FifteenPercent => "Decree No. 43 of 2013, Article 1(d)",
            RentalTier::Tier5TwentyPercent => "Decree No. 43 of 2013, Article 1(e)",
        }
    }
}

/// Evaluation input for Ejari rental increase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalEvaluationInput {
    pub current_rent_aed: f64,
    pub market_rent_aed: f64,
    pub notice_days: u32,
}

/// Detailed evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RentalIncreaseAssessment {
    pub current_rent_aed: f64,
    pub market_rent_aed: f64,
    pub gap_percentage: f64,
    pub tier: u8,
    pub tier_description: String,
    pub statutory_bracket_max_pct: f64,
    pub notice_days: u32,
    pub notice_compliant: bool,
    pub effective_permitted_increase_pct: f64,
    pub statutory_new_rent_aed: f64,
    pub max_annual_increase_amount_aed: f64,
    pub legal_reference: String,
    pub reasoning: String,
}

/// Calculate the exact RERA rental index gap percentage:
/// Delta_gap = ((R_market - R_current) / R_market) * 100%
pub fn calculate_rental_gap(current_rent: f64, market_rent: f64) -> f64 {
    if market_rent <= 0.0 {
        return 0.0;
    }
    if current_rent >= market_rent {
        return 0.0;
    }
    ((market_rent - current_rent) / market_rent) * 100.0
}

/// Determine the 5-tier statutory rental bracket based on gap percentage
pub fn determine_rental_tier(gap_pct: f64) -> RentalTier {
    if gap_pct <= 10.0 {
        RentalTier::Tier1NoIncrease
    } else if gap_pct <= 20.0 {
        RentalTier::Tier2FivePercent
    } else if gap_pct <= 30.0 {
        RentalTier::Tier3TenPercent
    } else if gap_pct <= 40.0 {
        RentalTier::Tier4FifteenPercent
    } else {
        RentalTier::Tier5TwentyPercent
    }
}

/// Evaluate rental increase compliance under Decree No. 43 of 2013 & Law No. 33 of 2008
pub fn evaluate_rental_increase(input: &RentalEvaluationInput) -> RentalIncreaseAssessment {
    let gap_pct = calculate_rental_gap(input.current_rent_aed, input.market_rent_aed);
    let tier = determine_rental_tier(gap_pct);
    let bracket_max_pct = tier.max_increase_percentage();
    let notice_compliant = input.notice_days >= STATUTORY_NOTICE_DAYS_MANDATE;

    let (effective_pct, legal_ref, reasoning) = if !notice_compliant {
        (
            0.0,
            "Law No. 33 of 2008, Article 14".to_string(),
            format!(
                "NON-COMPLIANT NOTICE: Landlord served notice {} calendar days prior to expiry, breaching the 90-day statutory mandate of Law No. 33 of 2008 Art. 14. Any proposed increase is legally null and void (0.0% allowed).",
                input.notice_days
            ),
        )
    } else {
        let text = match tier {
            RentalTier::Tier1NoIncrease => format!(
                "Tier 1: Current rent is within 10.0% of RERA market index ({:.2}% gap). Under Decree No. 43 of 2013 Art. 1(a), no increase is permitted (0%).",
                gap_pct
            ),
            RentalTier::Tier2FivePercent => format!(
                "Tier 2: Current rent is 11% to 20% below RERA market index ({:.2}% gap). Maximum permitted increase is 5.0% under Decree No. 43 of 2013 Art. 1(b).",
                gap_pct
            ),
            RentalTier::Tier3TenPercent => format!(
                "Tier 3: Current rent is 21% to 30% below RERA market index ({:.2}% gap). Maximum permitted increase is 10.0% under Decree No. 43 of 2013 Art. 1(c).",
                gap_pct
            ),
            RentalTier::Tier4FifteenPercent => format!(
                "Tier 4: Current rent is 31% to 40% below RERA market index ({:.2}% gap). Maximum permitted increase is 15.0% under Decree No. 43 of 2013 Art. 1(d).",
                gap_pct
            ),
            RentalTier::Tier5TwentyPercent => format!(
                "Tier 5: Current rent is more than 40% below RERA market index ({:.2}% gap). Maximum permitted increase is 20.0% under Decree No. 43 of 2013 Art. 1(e).",
                gap_pct
            ),
        };
        (bracket_max_pct, tier.legal_clause().to_string(), text)
    };

    let max_annual_increase_amount_aed = input.current_rent_aed * (effective_pct / 100.0);
    let statutory_new_rent_aed = input.current_rent_aed + max_annual_increase_amount_aed;

    RentalIncreaseAssessment {
        current_rent_aed: input.current_rent_aed,
        market_rent_aed: input.market_rent_aed,
        gap_percentage: (gap_pct * 100.0).round() / 100.0,
        tier: tier as u8,
        tier_description: format!("{:?}", tier),
        statutory_bracket_max_pct: bracket_max_pct,
        notice_days: input.notice_days,
        notice_compliant,
        effective_permitted_increase_pct: effective_pct,
        statutory_new_rent_aed,
        max_annual_increase_amount_aed,
        legal_reference: legal_ref,
        reasoning,
    }
}
