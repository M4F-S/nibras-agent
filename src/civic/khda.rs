//! KHDA Private School Fees Framework Engine
//! Education Cost Index (ECI) & DSIB Inspection Rating Multipliers.
//! Canonical Master Specification §4.1.5.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SchoolRating {
    VeryWeak = 0,
    Weak = 1,
    Acceptable = 2,
    Good = 3,
    VeryGood = 4,
    Outstanding = 5,
}

impl SchoolRating {
    pub fn from_str_name(s: &str) -> Option<Self> {
        let norm = s.to_lowercase().replace([' ', '-', '_'], "");
        match norm.as_str() {
            "veryweak" => Some(SchoolRating::VeryWeak),
            "weak" => Some(SchoolRating::Weak),
            "acceptable" => Some(SchoolRating::Acceptable),
            "good" => Some(SchoolRating::Good),
            "verygood" => Some(SchoolRating::VeryGood),
            "outstanding" => Some(SchoolRating::Outstanding),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            SchoolRating::VeryWeak => "Very Weak",
            SchoolRating::Weak => "Weak",
            SchoolRating::Acceptable => "Acceptable",
            SchoolRating::Good => "Good",
            SchoolRating::VeryGood => "Very Good",
            SchoolRating::Outstanding => "Outstanding",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RatingTrend {
    Maintained,
    Improved,
    Dropped,
}

/// Input parameters for KHDA fee calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhdaFeeInput {
    pub current_fee_aed: f64,
    pub eci_percentage: f64,
    pub previous_rating: SchoolRating,
    pub current_rating: SchoolRating,
}

/// Comprehensive KHDA fee assessment output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhdaFeeAssessment {
    pub current_fee_aed: f64,
    pub eci_percentage: f64,
    pub previous_rating: SchoolRating,
    pub current_rating: SchoolRating,
    pub rating_trend: RatingTrend,
    pub multiplier: f64,
    pub allowed_increase_percentage: f64,
    pub max_increase_amount_aed: f64,
    pub permitted_new_fee_aed: f64,
    pub statutory_rule_summary: String,
    pub framework_reference: String,
}

/// Calculate KHDA inspection shift multiplier based on rating transition
pub fn calculate_khda_multiplier(prev: SchoolRating, curr: SchoolRating) -> (RatingTrend, f64, &'static str) {
    if curr < prev {
        // Dropped rating: 0% fee freeze
        (RatingTrend::Dropped, 0.0, "Rating dropped: Strict 0% fee freeze enforced under KHDA School Fees Framework.")
    } else if curr == prev {
        // Maintained rating: 1.0 * ECI
        (RatingTrend::Maintained, 1.0, "Rating maintained: Eligible for 1.0x ECI increase.")
    } else {
        // Improved rating: Tiered multiplier per §4.1.5
        let (mult, note) = match (prev, curr) {
            (SchoolRating::VeryWeak, SchoolRating::Weak) => (2.0, "Improved from Very Weak to Weak: Eligible for 2.0x ECI increase."),
            (SchoolRating::Weak, SchoolRating::Acceptable) => (2.0, "Improved from Weak to Acceptable: Eligible for 2.0x ECI increase."),
            (SchoolRating::Acceptable, SchoolRating::Good) => (2.0, "Improved from Acceptable to Good: Eligible for 2.0x ECI increase."),
            (SchoolRating::Good, SchoolRating::VeryGood) => (1.75, "Improved from Good to Very Good: Eligible for 1.75x ECI increase."),
            (SchoolRating::VeryGood, SchoolRating::Outstanding) => (1.5, "Improved from Very Good to Outstanding: Eligible for 1.5x ECI increase."),
            _ => {
                // Multi-tier jump defaults to highest applicable destination tier
                match curr {
                    SchoolRating::Outstanding => (1.5, "Multi-tier improvement to Outstanding: Eligible for 1.5x ECI increase."),
                    SchoolRating::VeryGood => (1.75, "Multi-tier improvement to Very Good: Eligible for 1.75x ECI increase."),
                    _ => (2.0, "Multi-tier improvement: Eligible for 2.0x ECI increase."),
                }
            }
        };
        (RatingTrend::Improved, mult, note)
    }
}

/// Evaluate KHDA school fee adjustment
pub fn evaluate_khda_fee(input: &KhdaFeeInput) -> KhdaFeeAssessment {
    let (trend, multiplier, rule_note) = calculate_khda_multiplier(input.previous_rating, input.current_rating);
    let allowed_increase_pct = (multiplier * input.eci_percentage * 100.0).round() / 100.0;
    let max_increase_amount = ((input.current_fee_aed * (allowed_increase_pct / 100.0)) * 100.0).round() / 100.0;
    let permitted_new_fee = ((input.current_fee_aed + max_increase_amount) * 100.0).round() / 100.0;

    KhdaFeeAssessment {
        current_fee_aed: input.current_fee_aed,
        eci_percentage: input.eci_percentage,
        previous_rating: input.previous_rating,
        current_rating: input.current_rating,
        rating_trend: trend,
        multiplier,
        allowed_increase_percentage: allowed_increase_pct,
        max_increase_amount_aed: max_increase_amount,
        permitted_new_fee_aed: permitted_new_fee,
        statutory_rule_summary: rule_note.to_string(),
        framework_reference: "KHDA School Fees Framework / Dubai Statistics Center ECI Guidelines".to_string(),
    }
}
