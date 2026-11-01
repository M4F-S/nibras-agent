//! DEWA Residential Utility Slab and Tariff Engine
//! Expatriate Residential Progressive Multi-Slab Tariff Calculator.
//! Canonical Master Specification §4.1.3.

use serde::{Deserialize, Serialize};

pub const DEPOSIT_APARTMENT_AED: f64 = 2_000.0;
pub const DEPOSIT_VILLA_AED: f64 = 4_000.0;

pub const FUEL_SURCHARGE_ELECTRICITY_FILS_PER_KWH: f64 = 5.0; // 0.05 AED / kWh
pub const FUEL_SURCHARGE_WATER_FILS_PER_IG: f64 = 0.6;        // 0.006 AED / IG

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropertyType {
    Apartment,
    Villa,
}

impl PropertyType {
    pub fn security_deposit_aed(&self) -> f64 {
        match self {
            PropertyType::Apartment => DEPOSIT_APARTMENT_AED,
            PropertyType::Villa => DEPOSIT_VILLA_AED,
        }
    }
}

/// Breakdown of a single consumption slab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlabTierBreakdown {
    pub slab_name: String,
    pub units_consumed: f64,
    pub rate_fils_per_unit: f64,
    pub amount_aed: f64,
}

/// DEWA calculation input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DewaCalculationInput {
    pub electricity_kwh: f64,
    pub water_ig: f64,
    pub annual_ejari_aed: f64,
    pub property_type: Option<PropertyType>,
}

/// Complete DEWA monthly bill assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DewaBillAssessment {
    pub electricity_kwh: f64,
    pub electricity_slabs: Vec<SlabTierBreakdown>,
    pub electricity_tariff_subtotal_aed: f64,
    pub electricity_fuel_surcharge_aed: f64,
    pub electricity_total_aed: f64,

    pub water_ig: f64,
    pub water_slabs: Vec<SlabTierBreakdown>,
    pub water_tariff_subtotal_aed: f64,
    pub water_fuel_surcharge_aed: f64,
    pub water_total_aed: f64,

    pub annual_ejari_aed: f64,
    pub monthly_housing_fee_aed: f64,

    pub total_monthly_bill_aed: f64,
    pub security_deposit_aed: Option<f64>,
}

/// Calculate progressive electricity tariff for expatriate residential customers
/// - Green:  1 - 2,000 kWh @ 23 fils (0.23 AED)
/// - Yellow: 2,001 - 4,000 kWh @ 28 fils (0.28 AED)
/// - Orange: 4,001 - 6,000 kWh @ 32 fils (0.32 AED)
/// - Red:    > 6,000 kWh @ 38 fils (0.38 AED)
pub fn calculate_electricity_slabs(kwh: f64) -> (Vec<SlabTierBreakdown>, f64) {
    let mut slabs = Vec::new();
    let mut remaining = kwh.max(0.0);
    let mut subtotal = 0.0;

    // Green slab (0 - 2000)
    let green_units = remaining.min(2000.0);
    if green_units > 0.0 || kwh == 0.0 {
        let green_cost = green_units * 0.23;
        slabs.push(SlabTierBreakdown {
            slab_name: "Green Slab (1 - 2,000 kWh)".to_string(),
            units_consumed: green_units,
            rate_fils_per_unit: 23.0,
            amount_aed: (green_cost * 100.0).round() / 100.0,
        });
        subtotal += green_cost;
        remaining -= green_units;
    }

    // Yellow slab (2001 - 4000)
    if remaining > 0.0 {
        let yellow_units = remaining.min(2000.0);
        let yellow_cost = yellow_units * 0.28;
        slabs.push(SlabTierBreakdown {
            slab_name: "Yellow Slab (2,001 - 4,000 kWh)".to_string(),
            units_consumed: yellow_units,
            rate_fils_per_unit: 28.0,
            amount_aed: (yellow_cost * 100.0).round() / 100.0,
        });
        subtotal += yellow_cost;
        remaining -= yellow_units;
    }

    // Orange slab (4001 - 6000)
    if remaining > 0.0 {
        let orange_units = remaining.min(2000.0);
        let orange_cost = orange_units * 0.32;
        slabs.push(SlabTierBreakdown {
            slab_name: "Orange Slab (4,001 - 6,000 kWh)".to_string(),
            units_consumed: orange_units,
            rate_fils_per_unit: 32.0,
            amount_aed: (orange_cost * 100.0).round() / 100.0,
        });
        subtotal += orange_cost;
        remaining -= orange_units;
    }

    // Red slab (> 6000)
    if remaining > 0.0 {
        let red_units = remaining;
        let red_cost = red_units * 0.38;
        slabs.push(SlabTierBreakdown {
            slab_name: "Red Slab (> 6,000 kWh)".to_string(),
            units_consumed: red_units,
            rate_fils_per_unit: 38.0,
            amount_aed: (red_cost * 100.0).round() / 100.0,
        });
        subtotal += red_cost;
    }

    (slabs, (subtotal * 100.0).round() / 100.0)
}

/// Calculate progressive water tariff for expatriate residential customers (Imperial Gallons)
/// - Green:  1 - 6,000 IG @ 3.5 fils (0.035 AED)
/// - Yellow: 6,001 - 12,000 IG @ 4.0 fils (0.040 AED)
/// - Red:    > 12,000 IG @ 4.6 fils (0.046 AED)
pub fn calculate_water_slabs(ig: f64) -> (Vec<SlabTierBreakdown>, f64) {
    let mut slabs = Vec::new();
    let mut remaining = ig.max(0.0);
    let mut subtotal = 0.0;

    // Green slab (0 - 6000)
    let green_units = remaining.min(6000.0);
    if green_units > 0.0 || ig == 0.0 {
        let green_cost = green_units * 0.035;
        slabs.push(SlabTierBreakdown {
            slab_name: "Green Slab (1 - 6,000 IG)".to_string(),
            units_consumed: green_units,
            rate_fils_per_unit: 3.5,
            amount_aed: (green_cost * 100.0).round() / 100.0,
        });
        subtotal += green_cost;
        remaining -= green_units;
    }

    // Yellow slab (6001 - 12000)
    if remaining > 0.0 {
        let yellow_units = remaining.min(6000.0);
        let yellow_cost = yellow_units * 0.040;
        slabs.push(SlabTierBreakdown {
            slab_name: "Yellow Slab (6,001 - 12,000 IG)".to_string(),
            units_consumed: yellow_units,
            rate_fils_per_unit: 4.0,
            amount_aed: (yellow_cost * 100.0).round() / 100.0,
        });
        subtotal += yellow_cost;
        remaining -= yellow_units;
    }

    // Red slab (> 12000)
    if remaining > 0.0 {
        let red_units = remaining;
        let red_cost = red_units * 0.046;
        slabs.push(SlabTierBreakdown {
            slab_name: "Red Slab (> 12,000 IG)".to_string(),
            units_consumed: red_units,
            rate_fils_per_unit: 4.6,
            amount_aed: (red_cost * 100.0).round() / 100.0,
        });
        subtotal += red_cost;
    }

    (slabs, (subtotal * 100.0).round() / 100.0)
}

/// Calculate monthly housing fee: (Annual Ejari * 5%) / 12
pub fn calculate_housing_fee(annual_ejari: f64) -> f64 {
    ((annual_ejari * 0.05 / 12.0) * 100.0).round() / 100.0
}

/// Full residential utility evaluation
pub fn calculate_dewa_bill(input: &DewaCalculationInput) -> DewaBillAssessment {
    let (electricity_slabs, ele_subtotal) = calculate_electricity_slabs(input.electricity_kwh);
    let ele_fuel = ((input.electricity_kwh * (FUEL_SURCHARGE_ELECTRICITY_FILS_PER_KWH / 100.0)) * 100.0).round() / 100.0;
    let ele_total = ((ele_subtotal + ele_fuel) * 100.0).round() / 100.0;

    let (water_slabs, water_subtotal) = calculate_water_slabs(input.water_ig);
    let water_fuel = ((input.water_ig * (FUEL_SURCHARGE_WATER_FILS_PER_IG / 100.0)) * 100.0).round() / 100.0;
    let water_total = ((water_subtotal + water_fuel) * 100.0).round() / 100.0;

    let housing_fee = calculate_housing_fee(input.annual_ejari_aed);
    let total_monthly_bill = ((ele_total + water_total + housing_fee) * 100.0).round() / 100.0;

    let security_deposit = input.property_type.map(|p| p.security_deposit_aed());

    DewaBillAssessment {
        electricity_kwh: input.electricity_kwh,
        electricity_slabs,
        electricity_tariff_subtotal_aed: ele_subtotal,
        electricity_fuel_surcharge_aed: ele_fuel,
        electricity_total_aed: ele_total,

        water_ig: input.water_ig,
        water_slabs,
        water_tariff_subtotal_aed: water_subtotal,
        water_fuel_surcharge_aed: water_fuel,
        water_total_aed: water_total,

        annual_ejari_aed: input.annual_ejari_aed,
        monthly_housing_fee_aed: housing_fee,

        total_monthly_bill_aed: total_monthly_bill,
        security_deposit_aed: security_deposit,
    }
}
