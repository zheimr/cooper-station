use serde::{Deserialize, Serialize};

use crate::station::StationConfig;

/// Gravity zone designation within the station.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravityZone {
    /// Zone name
    pub name: String,
    /// Radial distance from central axis in meters
    pub radius: f64,
    /// Gravitational acceleration in m/s²
    pub gravity: f64,
    /// Gravity as fraction of Earth's g
    pub gravity_g: f64,
}

/// Predefined gravity zones for the station.
pub const ZONE_DEFINITIONS: &[(&str, f64)] = &[
    ("Residential Rim", 4000.0),
    ("Parks & Recreation", 3500.0),
    ("Light Commercial", 3000.0),
    ("Transit Level", 2000.0),
    ("Manufacturing", 1000.0),
    ("Assembly", 500.0),
    ("Zero-G Axis", 0.0),
];

impl StationConfig {
    /// Calculate gravity at a given radius from the central axis.
    ///
    /// g(r) = ω²r
    ///
    /// Returns acceleration in m/s².
    pub fn gravity_at_radius(&self, r: f64) -> f64 {
        assert!(
            r >= 0.0 && r <= self.radius,
            "Radius must be between 0 and station radius ({}m)",
            self.radius
        );
        self.omega().powi(2) * r
    }

    /// Calculate gravity as a fraction of Earth's g at a given radius.
    pub fn gravity_fraction_at_radius(&self, r: f64) -> f64 {
        self.gravity_at_radius(r) / 9.81
    }

    /// Get all predefined gravity zones with calculated values.
    pub fn gravity_zones(&self) -> Vec<GravityZone> {
        ZONE_DEFINITIONS
            .iter()
            .filter(|(_, r)| *r <= self.radius)
            .map(|(name, r)| {
                let g = self.gravity_at_radius(*r);
                GravityZone {
                    name: name.to_string(),
                    radius: *r,
                    gravity: g,
                    gravity_g: g / 9.81,
                }
            })
            .collect()
    }

    /// Calculate effective weight of an object at a given radius.
    ///
    /// W(r) = m × ω²r
    pub fn weight_at_radius(&self, mass_kg: f64, r: f64) -> f64 {
        mass_kg * self.gravity_at_radius(r)
    }

    /// Calculate the radius for a desired gravity level.
    ///
    /// r = g_desired / ω²
    pub fn radius_for_gravity(&self, desired_g: f64) -> Option<f64> {
        let omega = self.omega();
        let r = desired_g / omega.powi(2);
        if r <= self.radius {
            Some(r)
        } else {
            None // Requested gravity exceeds station capacity
        }
    }

    /// Calculate gravitational variation across a building of given height.
    ///
    /// At the rim, a building extending inward has slightly less gravity at its top.
    /// Returns (g_base, g_top, percentage_difference).
    pub fn gravity_across_building(&self, building_height: f64) -> (f64, f64, f64) {
        let g_base = self.gravity_at_radius(self.radius);
        let g_top = self.gravity_at_radius(self.radius - building_height);
        let pct_diff = ((g_base - g_top) / g_base) * 100.0;
        (g_base, g_top, pct_diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_at_rim() {
        let station = StationConfig::new();
        let g = station.gravity_at_radius(4000.0);
        assert!((g - 9.81).abs() < 0.01);
    }

    #[test]
    fn test_gravity_at_axis() {
        let station = StationConfig::new();
        let g = station.gravity_at_radius(0.0);
        assert!(g.abs() < 1e-10);
    }

    #[test]
    fn test_gravity_linear_scaling() {
        let station = StationConfig::new();
        let g_half = station.gravity_at_radius(2000.0);
        let g_full = station.gravity_at_radius(4000.0);
        assert!((g_half / g_full - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_radius_for_gravity() {
        let station = StationConfig::new();
        let r = station.radius_for_gravity(4.905).unwrap();
        // Half gravity should be at half radius
        assert!((r - 2000.0).abs() < 1.0);
    }

    #[test]
    fn test_building_gravity_gradient() {
        let station = StationConfig::new();
        let (g_base, g_top, pct) = station.gravity_across_building(50.0);
        // 50m building: ~1.25% gravity difference
        assert!((pct - 1.25).abs() < 0.1);
        assert!(g_base > g_top);
    }

    #[test]
    #[should_panic]
    fn test_gravity_beyond_radius() {
        let station = StationConfig::new();
        station.gravity_at_radius(5000.0); // Beyond station radius
    }

    #[test]
    fn test_gravity_zones_count() {
        let station = StationConfig::new();
        let zones = station.gravity_zones();
        assert_eq!(zones.len(), ZONE_DEFINITIONS.len());
    }
}
