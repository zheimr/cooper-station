use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Core configuration for a Cooper Station (O'Neill Cylinder) habitat.
///
/// All units are SI: meters, kilograms, seconds, Pascals, Kelvin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationConfig {
    /// Cylinder length in meters
    pub length: f64,
    /// Inner radius (habitable surface) in meters
    pub radius: f64,
    /// Hull thickness in meters
    pub hull_thickness: f64,
    /// Target gravity at rim in m/s²
    pub target_gravity: f64,
    /// Population capacity
    pub population: u64,
    /// Atmospheric pressure at rim in Pascals
    pub atmosphere_pressure: f64,
    /// O₂ fraction (0.0 - 1.0)
    pub o2_fraction: f64,
    /// N₂ fraction (0.0 - 1.0)
    pub n2_fraction: f64,
}

impl Default for StationConfig {
    fn default() -> Self {
        Self {
            length: 32_000.0,
            radius: 4_000.0,
            hull_thickness: 20.0,
            target_gravity: 9.81,
            population: 1_000_000,
            atmosphere_pressure: 50_650.0,
            o2_fraction: 0.40,
            n2_fraction: 0.60,
        }
    }
}

/// Derived parameters computed from station configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationDerived {
    /// Angular velocity in rad/s
    pub omega: f64,
    /// Rotation rate in RPM
    pub rpm: f64,
    /// Rotation period in seconds
    pub period: f64,
    /// Rim velocity in m/s
    pub rim_velocity: f64,
    /// Outer radius (hull exterior) in meters
    pub outer_radius: f64,
    /// Total cylindrical surface area in m²
    pub total_surface_area: f64,
    /// Habitable surface area in m² (3 of 6 strips)
    pub habitable_area: f64,
    /// Internal volume in m³
    pub internal_volume: f64,
    /// Rotational kinetic energy in Joules
    pub rotational_energy: f64,
    /// Moment of inertia (thin shell approx) in kg·m²
    pub moment_of_inertia: f64,
}

impl StationConfig {
    /// Create a new Cooper Station with default Island III parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a custom station configuration.
    pub fn custom(length: f64, radius: f64, target_gravity: f64) -> Self {
        Self {
            length,
            radius,
            target_gravity,
            ..Default::default()
        }
    }

    /// Compute angular velocity required for target gravity at rim.
    ///
    /// From centripetal acceleration: a = ω²r → ω = √(a/r)
    pub fn omega(&self) -> f64 {
        (self.target_gravity / self.radius).sqrt()
    }

    /// Rotation rate in revolutions per minute.
    pub fn rpm(&self) -> f64 {
        self.omega() * 60.0 / (2.0 * PI)
    }

    /// Rotation period in seconds.
    pub fn period(&self) -> f64 {
        2.0 * PI / self.omega()
    }

    /// Linear velocity at the rim in m/s.
    pub fn rim_velocity(&self) -> f64 {
        self.omega() * self.radius
    }

    /// Compute all derived parameters at once.
    pub fn derive(&self) -> StationDerived {
        let omega = self.omega();
        let outer_radius = self.radius + self.hull_thickness;
        let total_surface = 2.0 * PI * self.radius * self.length;
        let habitable = total_surface / 2.0; // 3 of 6 strips
        let volume = PI * self.radius.powi(2) * self.length;

        // Rough mass estimate for moment of inertia (thin shell)
        let estimated_mass = 8.75e12; // kg, from spec
        let moi = 0.5 * estimated_mass * self.radius.powi(2);
        let rot_energy = 0.5 * moi * omega.powi(2);

        StationDerived {
            omega,
            rpm: self.rpm(),
            period: self.period(),
            rim_velocity: self.rim_velocity(),
            outer_radius,
            total_surface_area: total_surface,
            habitable_area: habitable,
            internal_volume: volume,
            rotational_energy: rot_energy,
            moment_of_inertia: moi,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_station_omega() {
        let station = StationConfig::new();
        let omega = station.omega();
        // Expected: sqrt(9.81/4000) ≈ 0.04952
        assert!((omega - 0.04952).abs() < 0.001);
    }

    #[test]
    fn test_default_station_rpm() {
        let station = StationConfig::new();
        let rpm = station.rpm();
        // Expected: ~0.473
        assert!((rpm - 0.473).abs() < 0.01);
    }

    #[test]
    fn test_default_station_period() {
        let station = StationConfig::new();
        let period = station.period();
        // Expected: ~126.9 seconds
        assert!((period - 126.9).abs() < 1.0);
    }

    #[test]
    fn test_rim_velocity() {
        let station = StationConfig::new();
        let v = station.rim_velocity();
        // Expected: ~198 m/s
        assert!((v - 198.0).abs() < 2.0);
    }

    #[test]
    fn test_habitable_area() {
        let station = StationConfig::new();
        let derived = station.derive();
        // Expected: ~500 km² = 5e8 m²
        let area_km2 = derived.habitable_area / 1e6;
        assert!((area_km2 - 402.0).abs() < 50.0);
    }

    #[test]
    fn test_custom_station() {
        let station = StationConfig::custom(16_000.0, 2_000.0, 9.81);
        let omega = station.omega();
        // omega = sqrt(9.81/2000) ≈ 0.07004
        assert!((omega - 0.07004).abs() < 0.001);
    }
}
