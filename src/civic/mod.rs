//! Nibras Civic Navigator - The 6 Sovereign Statutory Engines
//! Canonical Master Specification §4.1:
//! 1. GDRFA 10-Year Golden Visa Eligibility Engine
//! 2. DLD Ejari & Decree No. 43 of 2013 Rental Increase Bracket Engine
//! 3. DEWA Residential Utility Slab and Tariff Engine
//! 4. RTA Smart Mobility Engine (Salik & Vehicle Registration)
//! 5. KHDA Private School Fees Framework Engine
//! 6. UAE Pass PKI & Digital Signature Engine

pub mod dewa;
pub mod ejari;
pub mod golden_visa;
pub mod khda;
pub mod rta;
pub mod uae_pass;

pub use dewa::{
    calculate_dewa_bill, calculate_electricity_slabs, calculate_housing_fee, calculate_water_slabs,
    DewaBillAssessment, DewaCalculationInput, PropertyType, SlabTierBreakdown,
};

pub use ejari::{
    calculate_rental_gap, determine_rental_tier, evaluate_rental_increase,
    RentalEvaluationInput, RentalIncreaseAssessment, RentalTier,
    STATUTORY_NOTICE_DAYS_MANDATE,
};

pub use golden_visa::{
    assess_family_member, evaluate_real_estate_investor, evaluate_skilled_professional,
    FamilyMember, FamilyMemberAssessment, FamilyRelation, GoldenVisaAssessment,
    GoldenVisaCategory, MohreSkillLevel, RealEstateInvestorInput, SkilledProfessionalInput,
    MIN_REAL_ESTATE_VALUATION_AED, MIN_SKILLED_BASIC_SALARY_AED,
};

pub use khda::{
    calculate_khda_multiplier, evaluate_khda_fee, KhdaFeeAssessment, KhdaFeeInput,
    RatingTrend, SchoolRating,
};

pub use rta::{
    calculate_parking, calculate_salik, evaluate_vehicle_renewal, ParkingAssessment,
    ParkingZone, SalikAssessment, VehicleRenewalAssessment, VehicleRenewalInput,
    SALIK_OFFPEAK_RATE_AED, SALIK_PEAK_RATE_AED, VEHICLE_STATUTORY_FIXED_TOTAL_AED,
};

pub use uae_pass::{
    create_signed_envelope, hash_document, verify_envelope, IdentityAssuranceLevel,
    UaePassSignedEnvelope, UaePassValidationResult,
};
