//! RTA Smart Mobility Engine
//! Salik Variable Pricing, Vehicle Registration Renewal, and Public Parking Zones.
//! Canonical Master Specification §4.1.4.

use serde::{Deserialize, Serialize};

pub const SALIK_PEAK_RATE_AED: f64 = 6.0;
pub const SALIK_OFFPEAK_RATE_AED: f64 = 4.0;
pub const SALIK_FREE_RATE_AED: f64 = 0.0;

pub const VEHICLE_INSPECTION_FEE_AED: f64 = 170.0;
pub const VEHICLE_RENEWAL_FEE_AED: f64 = 350.0;
pub const VEHICLE_KNOWLEDGE_INNOVATION_FEE_AED: f64 = 20.0;
pub const VEHICLE_STATUTORY_FIXED_TOTAL_AED: f64 =
    VEHICLE_INSPECTION_FEE_AED + VEHICLE_RENEWAL_FEE_AED + VEHICLE_KNOWLEDGE_INNOVATION_FEE_AED; // AED 540.00

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParkingZone {
    ZoneA, // Commercial roadside: AED 4.00 / hr
    ZoneB, // Commercial parking lot: AED 3.00 / hr
    ZoneC, // Non-commercial roadside: AED 2.00 / hr
    ZoneD, // Non-commercial parking lot: AED 2.00 / hr
}

impl ParkingZone {
    pub fn hourly_rate_aed(&self) -> f64 {
        match self {
            ParkingZone::ZoneA => 4.0,
            ParkingZone::ZoneB => 3.0,
            ParkingZone::ZoneC => 2.0,
            ParkingZone::ZoneD => 2.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ParkingZone::ZoneA => "Zone A (Commercial Roadside Parking) - AED 4.00/hr",
            ParkingZone::ZoneB => "Zone B (Commercial Parking Lots) - AED 3.00/hr",
            ParkingZone::ZoneC => "Zone C (Non-Commercial Roadside Parking) - AED 2.00/hr",
            ParkingZone::ZoneD => "Zone D (Non-Commercial Parking Lots) - AED 2.00/hr",
        }
    }

    pub fn from_str_name(name: &str) -> Option<Self> {
        match name.to_uppercase().trim() {
            "A" | "ZONE A" | "ZONE_A" => Some(ParkingZone::ZoneA),
            "B" | "ZONE B" | "ZONE_B" => Some(ParkingZone::ZoneB),
            "C" | "ZONE C" | "ZONE_C" => Some(ParkingZone::ZoneC),
            "D" | "ZONE D" | "ZONE_D" => Some(ParkingZone::ZoneD),
            _ => None,
        }
    }
}

/// Salik toll calculation breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalikAssessment {
    pub total_crossings: u32,
    pub peak_crossings: u32,
    pub peak_cost_aed: f64,
    pub offpeak_crossings: u32,
    pub offpeak_cost_aed: f64,
    pub free_crossings: u32,
    pub free_cost_aed: f64,
    pub total_toll_aed: f64,
}

/// Calculate dynamic Salik toll amounts
pub fn calculate_salik(peak_crossings: u32, offpeak_crossings: u32, free_crossings: u32) -> SalikAssessment {
    let peak_cost = (peak_crossings as f64) * SALIK_PEAK_RATE_AED;
    let offpeak_cost = (offpeak_crossings as f64) * SALIK_OFFPEAK_RATE_AED;
    let free_cost = (free_crossings as f64) * SALIK_FREE_RATE_AED;
    let total_crossings = peak_crossings + offpeak_crossings + free_crossings;
    let total_toll = peak_cost + offpeak_cost + free_cost;

    SalikAssessment {
        total_crossings,
        peak_crossings,
        peak_cost_aed: (peak_cost * 100.0).round() / 100.0,
        offpeak_crossings,
        offpeak_cost_aed: (offpeak_cost * 100.0).round() / 100.0,
        free_crossings,
        free_cost_aed: (free_cost * 100.0).round() / 100.0,
        total_toll_aed: (total_toll * 100.0).round() / 100.0,
    }
}

/// Vehicle renewal checklist input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleRenewalInput {
    pub vehicle_age_years: u32,
    pub inspection_passed: bool,
    pub inspection_days_ago: u32,
    pub has_13_month_insurance: bool,
    pub outstanding_fines_aed: f64,
}

/// Vehicle renewal checklist assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleRenewalAssessment {
    pub ready_for_renewal: bool,
    pub inspection_required: bool,
    pub inspection_valid: bool,
    pub insurance_valid: bool,
    pub fines_cleared: bool,
    pub inspection_fee_aed: f64,
    pub registration_fee_aed: f64,
    pub knowledge_innovation_fee_aed: f64,
    pub outstanding_fines_aed: f64,
    pub total_payable_aed: f64,
    pub pending_actions: Vec<String>,
}

/// Evaluate vehicle registration renewal readiness and costs
pub fn evaluate_vehicle_renewal(input: &VehicleRenewalInput) -> VehicleRenewalAssessment {
    // New vehicles (< 3 years) are exempt from annual technical inspection
    let inspection_required = input.vehicle_age_years >= 3;
    let inspection_valid = if inspection_required {
        input.inspection_passed && input.inspection_days_ago <= 30
    } else {
        true
    };

    let insurance_valid = input.has_13_month_insurance;
    let fines_cleared = input.outstanding_fines_aed <= 0.0;

    let mut pending_actions = Vec::new();

    if inspection_required {
        if !input.inspection_passed {
            pending_actions.push("Pass RTA Technical Inspection at an authorized testing center (AED 170).".to_string());
        } else if input.inspection_days_ago > 30 {
            pending_actions.push("Re-inspect vehicle: previous inspection certificate exceeded 30-day statutory validity.".to_string());
        }
    }

    if !insurance_valid {
        pending_actions.push("Secure 13-month comprehensive or third-party vehicle insurance certificate.".to_string());
    }

    if !fines_cleared {
        pending_actions.push(format!("Settle outstanding traffic fines of AED {:.2} via Dubai Police or RTA portal.", input.outstanding_fines_aed));
    }

    let ready_for_renewal = inspection_valid && insurance_valid && fines_cleared;

    let inspection_fee = if inspection_required { VEHICLE_INSPECTION_FEE_AED } else { 0.0 };
    let registration_fee = VEHICLE_RENEWAL_FEE_AED;
    let knowledge_fee = VEHICLE_KNOWLEDGE_INNOVATION_FEE_AED;
    let total_payable = inspection_fee + registration_fee + knowledge_fee + input.outstanding_fines_aed.max(0.0);

    VehicleRenewalAssessment {
        ready_for_renewal,
        inspection_required,
        inspection_valid,
        insurance_valid,
        fines_cleared,
        inspection_fee_aed: inspection_fee,
        registration_fee_aed: registration_fee,
        knowledge_innovation_fee_aed: knowledge_fee,
        outstanding_fines_aed: input.outstanding_fines_aed.max(0.0),
        total_payable_aed: (total_payable * 100.0).round() / 100.0,
        pending_actions,
    }
}

/// Parking fee calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParkingAssessment {
    pub zone: ParkingZone,
    pub hourly_rate_aed: f64,
    pub duration_hours: f64,
    pub total_fee_aed: f64,
    pub zone_description: String,
}

/// Calculate parking fee by zone and duration
pub fn calculate_parking(zone: ParkingZone, hours: f64) -> ParkingAssessment {
    let rate = zone.hourly_rate_aed();
    let total = (rate * hours.max(0.0) * 100.0).round() / 100.0;

    ParkingAssessment {
        zone,
        hourly_rate_aed: rate,
        duration_hours: hours,
        total_fee_aed: total,
        zone_description: zone.description().to_string(),
    }
}
