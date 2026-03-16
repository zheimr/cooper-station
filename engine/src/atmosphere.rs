use serde::{Deserialize, Serialize};

use crate::station::StationConfig;

/// Gas constant for ideal gas law (J/(mol·K))
const R_GAS: f64 = 8.314;

/// Mean molar mass of station atmosphere (kg/mol)
/// 40% O₂ (32 g/mol) + 60% N₂ (28 g/mol) = 29.6 g/mol
const MEAN_MOLAR_MASS: f64 = 0.0296;

/// Atmospheric conditions at a given point in the station.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtmosphericConditions {
    /// Radius from central axis in meters
    pub radius: f64,
    /// Total pressure in Pascals
    pub pressure: f64,
    /// O₂ partial pressure in Pascals
    pub o2_pressure: f64,
    /// N₂ partial pressure in Pascals
    pub n2_pressure: f64,
    /// Air density in kg/m³
    pub density: f64,
    /// Temperature in Kelvin (assumed uniform for simplification)
    pub temperature: f64,
    /// Speed of sound in m/s
    pub speed_of_sound: f64,
}

/// ECLSS (Environmental Control and Life Support System) budget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EclssBudget {
    /// Daily O₂ consumption in kg
    pub daily_o2_consumption: f64,
    /// Daily CO₂ production in kg
    pub daily_co2_production: f64,
    /// Daily water consumption in m³
    pub daily_water_consumption: f64,
    /// Required biological O₂ generation area in m²
    pub bio_generation_area: f64,
    /// Total atmospheric mass in kg
    pub total_atmospheric_mass: f64,
}

impl StationConfig {
    /// Calculate atmospheric pressure at a given radius.
    ///
    /// In a rotating reference frame, the pressure increases toward the rim
    /// following a barometric-like formula:
    ///
    /// P(r) = P_axis × exp(ω²r²M / (2RT))
    ///
    /// We work backward from rim pressure to find axis pressure.
    pub fn pressure_at_radius(&self, r: f64, temperature_k: f64) -> f64 {
        let omega = self.omega();
        let exponent_rim =
            omega.powi(2) * self.radius.powi(2) * MEAN_MOLAR_MASS / (2.0 * R_GAS * temperature_k);
        let p_axis = self.atmosphere_pressure / exponent_rim.exp();

        let exponent_r =
            omega.powi(2) * r.powi(2) * MEAN_MOLAR_MASS / (2.0 * R_GAS * temperature_k);
        p_axis * exponent_r.exp()
    }

    /// Get full atmospheric conditions at a given radius.
    pub fn atmosphere_at_radius(&self, r: f64, temperature_k: f64) -> AtmosphericConditions {
        let pressure = self.pressure_at_radius(r, temperature_k);
        let o2_pressure = pressure * self.o2_fraction;
        let n2_pressure = pressure * self.n2_fraction;

        // Ideal gas law: ρ = PM/(RT)
        let density = pressure * MEAN_MOLAR_MASS / (R_GAS * temperature_k);

        // Speed of sound: c = sqrt(γRT/M) where γ ≈ 1.4 for diatomic gases
        let gamma = 1.4;
        let speed_of_sound = (gamma * R_GAS * temperature_k / MEAN_MOLAR_MASS).sqrt();

        AtmosphericConditions {
            radius: r,
            pressure,
            o2_pressure,
            n2_pressure,
            density,
            temperature: temperature_k,
            speed_of_sound,
        }
    }

    /// Calculate ECLSS budget for the station's population.
    pub fn eclss_budget(&self, temperature_k: f64) -> EclssBudget {
        let pop = self.population as f64;

        // Per-person daily values (from ISS research):
        let o2_per_person = 0.84; // kg/day
        let co2_per_person = 1.0; // kg/day
        let water_per_person = 0.353; // m³/day (total including agriculture)

        // Biological O₂ generation: tropical forest produces ~15 tonnes O₂/km²/day
        let total_o2 = o2_per_person * pop;
        let bio_area = total_o2 / 15_000.0 * 1e6; // Convert to m²

        // Total atmospheric mass: integrate density over cylinder volume
        // Simplified: use average density × volume
        let avg_conditions = self.atmosphere_at_radius(self.radius * 0.7, temperature_k);
        let volume = std::f64::consts::PI * self.radius.powi(2) * self.length;
        let atm_mass = avg_conditions.density * volume;

        EclssBudget {
            daily_o2_consumption: total_o2,
            daily_co2_production: co2_per_person * pop,
            daily_water_consumption: water_per_person * pop,
            bio_generation_area: bio_area,
            total_atmospheric_mass: atm_mass,
        }
    }

    /// Calculate pressure ratio between rim and axis.
    pub fn pressure_ratio(&self, temperature_k: f64) -> f64 {
        let omega = self.omega();
        let exponent =
            omega.powi(2) * self.radius.powi(2) * MEAN_MOLAR_MASS / (2.0 * R_GAS * temperature_k);
        exponent.exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pressure_at_rim() {
        let station = StationConfig::new();
        let p = station.pressure_at_radius(4000.0, 293.0);
        // Should be close to configured rim pressure
        assert!((p - 50_650.0).abs() < 100.0);
    }

    #[test]
    fn test_pressure_gradient() {
        let station = StationConfig::new();
        let p_axis = station.pressure_at_radius(0.0, 293.0);
        let p_rim = station.pressure_at_radius(4000.0, 293.0);
        // Axis pressure should be lower than rim
        assert!(p_axis < p_rim);
        // Ratio should be ~1.26
        let ratio = p_rim / p_axis;
        assert!((ratio - 1.26).abs() < 0.05);
    }

    #[test]
    fn test_atmosphere_density() {
        let station = StationConfig::new();
        let atm = station.atmosphere_at_radius(4000.0, 293.0);
        // At 0.5 atm, density should be roughly half of sea level (~1.225 kg/m³)
        assert!(atm.density > 0.4 && atm.density < 0.8);
    }

    #[test]
    fn test_speed_of_sound() {
        let station = StationConfig::new();
        let atm = station.atmosphere_at_radius(4000.0, 293.0);
        // Speed of sound should be ~340-350 m/s (similar to Earth at same temp)
        assert!(atm.speed_of_sound > 330.0 && atm.speed_of_sound < 360.0);
    }

    #[test]
    fn test_eclss_budget() {
        let station = StationConfig::new();
        let budget = station.eclss_budget(293.0);
        // 1M people × 0.84 kg/day = 840,000 kg/day O₂
        assert!((budget.daily_o2_consumption - 840_000.0).abs() < 1000.0);
    }

    #[test]
    fn test_pressure_ratio() {
        let station = StationConfig::new();
        let ratio = station.pressure_ratio(293.0);
        // Expected ~1.26
        assert!((ratio - 1.26).abs() < 0.05);
    }
}
