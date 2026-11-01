//! GDRFA 10-Year Golden Visa Eligibility Engine
//! Implements Federal Decree-Law No. 29 of 2021 & Cabinet Resolution No. 65 of 2022.
//! Canonical Master Specification §4.1.1.

use serde::{Deserialize, Serialize};

pub const MIN_REAL_ESTATE_VALUATION_AED: f64 = 2_000_000.0;
pub const MIN_SKILLED_BASIC_SALARY_AED: f64 = 30_000.0;
pub const MAX_SPONSORED_SON_AGE_DEFAULT: u32 = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoldenVisaCategory {
    RealEstateInvestor,
    SkilledProfessional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MohreSkillLevel {
    Level1Managers,     // Level 1: Legislators, managers, business executives
    Level2Professionals, // Level 2: Professionals in scientific, technical, human fields
    Level3Technicians,  // Level 3: Technicians & associate professionals (ineligible)
    Level4Other,        // Other levels (ineligible)
}

impl MohreSkillLevel {
    pub fn from_u32(val: u32) -> Self {
        match val {
            1 => MohreSkillLevel::Level1Managers,
            2 => MohreSkillLevel::Level2Professionals,
            3 => MohreSkillLevel::Level3Technicians,
            _ => MohreSkillLevel::Level4Other,
        }
    }

    pub fn is_eligible(&self) -> bool {
        matches!(self, MohreSkillLevel::Level1Managers | MohreSkillLevel::Level2Professionals)
    }

    pub fn description(&self) -> &'static str {
        match self {
            MohreSkillLevel::Level1Managers => "MoHRE Level 1: Legislators, Managers & Executives",
            MohreSkillLevel::Level2Professionals => "MoHRE Level 2: Professionals & Scientists",
            MohreSkillLevel::Level3Technicians => "MoHRE Level 3: Technicians & Associate Professionals",
            MohreSkillLevel::Level4Other => "MoHRE Level 4+: Other Occupational Classifications",
        }
    }
}

/// Real Estate Investor input parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealEstateInvestorInput {
    pub property_valuation_aed: f64,
    pub equity_paid_aed: f64,
    pub is_mortgaged: bool,
    pub has_bank_noc: bool,
    pub is_off_plan: bool,
    pub has_oqood_registration: bool,
}

/// Skilled Professional input parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkilledProfessionalInput {
    pub mohre_level: MohreSkillLevel,
    pub monthly_basic_salary_aed: f64,
    pub degree_attested_mofa_moe: bool,
    pub active_employment_contract: bool,
}

/// Family member profile for sponsorship verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub relation: FamilyRelation,
    pub age: u32,
    pub is_married: bool,
    pub is_student: bool,
    pub is_person_of_determination: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyRelation {
    Spouse,
    Daughter,
    Son,
    Parent,
    DomesticWorker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCheck {
    pub rule_name: String,
    pub requirement: String,
    pub provided: String,
    pub passed: bool,
    pub statutory_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMemberAssessment {
    pub relation: FamilyRelation,
    pub eligible: bool,
    pub visa_duration_years: u32,
    pub condition_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenVisaAssessment {
    pub category: GoldenVisaCategory,
    pub is_eligible: bool,
    pub statutory_reference: String,
    pub checks: Vec<RuleCheck>,
    pub family_sponsorship_notes: Vec<String>,
    pub summary_statement: String,
}

/// Evaluate Real Estate Investor Golden Visa
pub fn evaluate_real_estate_investor(input: &RealEstateInvestorInput) -> GoldenVisaAssessment {
    let mut checks = Vec::new();

    // 1. Minimum property valuation (AED 2,000,000)
    let val_passed = input.property_valuation_aed >= MIN_REAL_ESTATE_VALUATION_AED;
    checks.push(RuleCheck {
        rule_name: "Minimum Property Valuation".to_string(),
        requirement: format!(">= AED {:.2}", MIN_REAL_ESTATE_VALUATION_AED),
        provided: format!("AED {:.2}", input.property_valuation_aed),
        passed: val_passed,
        statutory_reference: "Cabinet Resolution No. 65 of 2022, Art. 17(1)".to_string(),
    });

    // 2. Minimum Paid Equity (AED 2,000,000)
    let equity_passed = input.equity_paid_aed >= MIN_REAL_ESTATE_VALUATION_AED;
    checks.push(RuleCheck {
        rule_name: "Net Paid Investor Equity".to_string(),
        requirement: format!(">= AED {:.2}", MIN_REAL_ESTATE_VALUATION_AED),
        provided: format!("AED {:.2}", input.equity_paid_aed),
        passed: equity_passed,
        statutory_reference: "DLD Golden Visa Regulations / GDRFA Real Estate Rules".to_string(),
    });

    // 3. Mortgaged Property bank letter verification
    if input.is_mortgaged {
        let bank_passed = input.has_bank_noc && equity_passed;
        checks.push(RuleCheck {
            rule_name: "Bank Mortgage NOC / Equity Certification".to_string(),
            requirement: "Official bank NOC confirming paid capital equity >= AED 2,000,000.00".to_string(),
            provided: if input.has_bank_noc { "Bank NOC provided".to_string() } else { "Missing Bank NOC".to_string() },
            passed: bank_passed,
            statutory_reference: "DLD Real Estate Investment Trustee Notice 2024".to_string(),
        });
    }

    // 4. Off-Plan DLD Oqood registration
    if input.is_off_plan {
        let oqood_passed = input.has_oqood_registration && equity_passed;
        checks.push(RuleCheck {
            rule_name: "Off-Plan DLD Oqood Contract".to_string(),
            requirement: "Approved DLD Oqood contract showing realized payments >= AED 2,000,000.00".to_string(),
            provided: if input.has_oqood_registration { "Valid Oqood contract registered".to_string() } else { "No Oqood registration".to_string() },
            passed: oqood_passed,
            statutory_reference: "DLD Off-Plan Golden Visa Resolution 2024".to_string(),
        });
    }

    let is_eligible = checks.iter().all(|c| c.passed);

    let summary_statement = if is_eligible {
        format!(
            "QUALIFIED: Applicant satisfies all GDRFA 10-Year Real Estate Golden Visa statutory criteria with AED {:.2} equity.",
            input.equity_paid_aed
        )
    } else {
        let failed_rules: Vec<String> = checks.iter().filter(|c| !c.passed).map(|c| c.rule_name.clone()).collect();
        format!(
            "DISQUALIFIED: Real Estate Golden Visa requirements unmet for: {}.",
            failed_rules.join(", ")
        )
    };

    let family_notes = vec![
        "Spouse: Uncapped duration 10-year residency visa.".to_string(),
        "Daughters: Uncapped age limit (must be unmarried).".to_string(),
        "Sons: Sponsored up to age 25 without work restriction (uncapped if Person of Determination).".to_string(),
        "Parents: 1-Year renewable residence visa sponsorship.".to_string(),
        "Domestic helpers: Uncapped domestic staff sponsorship.".to_string(),
    ];

    GoldenVisaAssessment {
        category: GoldenVisaCategory::RealEstateInvestor,
        is_eligible,
        statutory_reference: "Cabinet Resolution No. 65 of 2022 / Federal Decree-Law No. 29 of 2021".to_string(),
        checks,
        family_sponsorship_notes: family_notes,
        summary_statement,
    }
}

/// Evaluate Skilled Professional Golden Visa
pub fn evaluate_skilled_professional(input: &SkilledProfessionalInput) -> GoldenVisaAssessment {
    let mut checks = Vec::new();

    // 1. Active employment contract
    checks.push(RuleCheck {
        rule_name: "Active UAE Employment Contract".to_string(),
        requirement: "Valid employment contract in mainland UAE or licensed Free Zone".to_string(),
        provided: if input.active_employment_contract { "Active contract confirmed".to_string() } else { "No active contract".to_string() },
        passed: input.active_employment_contract,
        statutory_reference: "Cabinet Resolution No. 65 of 2022, Art. 18(1)".to_string(),
    });

    // 2. MoHRE Occupational Skill Level 1 or 2
    let level_passed = input.mohre_level.is_eligible();
    checks.push(RuleCheck {
        rule_name: "MoHRE Occupational Classification".to_string(),
        requirement: "Occupational Level 1 (Directors/Managers) or Level 2 (Professionals)".to_string(),
        provided: input.mohre_level.description().to_string(),
        passed: level_passed,
        statutory_reference: "MoHRE Standard Occupational Classification (ISCO-08)".to_string(),
    });

    // 3. Minimum Monthly Basic Salary (AED 30,000.00 excluding allowances)
    let salary_passed = input.monthly_basic_salary_aed >= MIN_SKILLED_BASIC_SALARY_AED;
    checks.push(RuleCheck {
        rule_name: "Minimum Monthly Basic Salary".to_string(),
        requirement: format!(">= AED {:.2} (basic salary excluding allowances)", MIN_SKILLED_BASIC_SALARY_AED),
        provided: format!("AED {:.2}", input.monthly_basic_salary_aed),
        passed: salary_passed,
        statutory_reference: "GDRFA Executive Regulation / Cabinet Resolution No. 65 of 2022".to_string(),
    });

    // 4. Attested Bachelor's degree authenticated by MoFA and MoE
    checks.push(RuleCheck {
        rule_name: "Attested University Degree".to_string(),
        requirement: "Bachelor's degree or higher authenticated by MoFA with MoE Equivalency".to_string(),
        provided: if input.degree_attested_mofa_moe { "Attested degree authenticated".to_string() } else { "Degree unverified or lacking MoE equivalency".to_string() },
        passed: input.degree_attested_mofa_moe,
        statutory_reference: "MoE University Degree Authentication Mandate".to_string(),
    });

    let is_eligible = checks.iter().all(|c| c.passed);

    let summary_statement = if is_eligible {
        format!(
            "QUALIFIED: Applicant satisfies all GDRFA 10-Year Skilled Professional Golden Visa statutory requirements (Salary: AED {:.2}, Level: {:?}).",
            input.monthly_basic_salary_aed, input.mohre_level
        )
    } else {
        let failed: Vec<String> = checks.iter().filter(|c| !c.passed).map(|c| c.rule_name.clone()).collect();
        format!(
            "DISQUALIFIED: Skilled Professional Golden Visa requirements unmet for: {}.",
            failed.join(", ")
        )
    };

    let family_notes = vec![
        "Spouse: Uncapped duration 10-year residency visa.".to_string(),
        "Daughters: Uncapped age limit (must be unmarried).".to_string(),
        "Sons: Sponsored up to age 25 (uncapped if Person of Determination or documented university student).".to_string(),
        "Parents: 1-Year renewable residence visa sponsorship.".to_string(),
        "Stay outside UAE: Can remain outside UAE for > 6 months without visa invalidation.".to_string(),
    ];

    GoldenVisaAssessment {
        category: GoldenVisaCategory::SkilledProfessional,
        is_eligible,
        statutory_reference: "Cabinet Resolution No. 65 of 2022 / Federal Decree-Law No. 29 of 2021".to_string(),
        checks,
        family_sponsorship_notes: family_notes,
        summary_statement,
    }
}

/// Assess family member sponsorship under Golden Visa umbrella
pub fn assess_family_member(member: &FamilyMember) -> FamilyMemberAssessment {
    match member.relation {
        FamilyRelation::Spouse => FamilyMemberAssessment {
            relation: member.relation,
            eligible: true,
            visa_duration_years: 10,
            condition_notes: "Full 10-year residency without work restriction.".to_string(),
        },
        FamilyRelation::Daughter => {
            if member.is_married {
                FamilyMemberAssessment {
                    relation: member.relation,
                    eligible: false,
                    visa_duration_years: 0,
                    condition_notes: "Married daughters cannot be sponsored under primary applicant; must be sponsored by spouse or independent visa.".to_string(),
                }
            } else {
                FamilyMemberAssessment {
                    relation: member.relation,
                    eligible: true,
                    visa_duration_years: 10,
                    condition_notes: "Uncapped age limit for unmarried daughters.".to_string(),
                }
            }
        }
        FamilyRelation::Son => {
            if member.age <= MAX_SPONSORED_SON_AGE_DEFAULT || member.is_person_of_determination || member.is_student {
                FamilyMemberAssessment {
                    relation: member.relation,
                    eligible: true,
                    visa_duration_years: 10,
                    condition_notes: if member.is_person_of_determination {
                        "Eligible: Uncapped age exception for Person of Determination.".to_string()
                    } else if member.is_student && member.age > MAX_SPONSORED_SON_AGE_DEFAULT {
                        "Eligible: Documented university student continuation exception.".to_string()
                    } else {
                        format!("Eligible: Son under statutory age limit (age {} <= 25).", member.age)
                    },
                }
            } else {
                FamilyMemberAssessment {
                    relation: member.relation,
                    eligible: false,
                    visa_duration_years: 0,
                    condition_notes: format!("Ineligible: Son exceeds age 25 (age {}) and lacks student or determination exemption.", member.age),
                }
            }
        }
        FamilyRelation::Parent => FamilyMemberAssessment {
            relation: member.relation,
            eligible: true,
            visa_duration_years: 1,
            condition_notes: "1-Year renewable residence visa sponsorship under GDRFA humanitarian rules.".to_string(),
        },
        FamilyRelation::DomesticWorker => FamilyMemberAssessment {
            relation: member.relation,
            eligible: true,
            visa_duration_years: 2,
            condition_notes: "Uncapped domestic staff sponsorship under standard MoHRE domestic worker contract.".to_string(),
        },
    }
}
