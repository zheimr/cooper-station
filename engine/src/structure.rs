use serde::{Deserialize, Serialize};

use crate::station::StationConfig;

/// Material properties for structural analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    /// Material name
    pub name: String,
    /// Density in kg/m³
    pub density: f64,
    /// Yield strength in Pascals
    pub yield_strength: f64,
    /// Ultimate tensile strength in Pascals
    pub ultimate_strength: f64,
    /// Technology Readiness Level (1-9)
    pub trl: u8,
}

/// Common materials for space habitat construction.
pub fn material_library() -> Vec<Material> {
    vec![
        Material {
            name: "Steel (A36)".into(),
            density: 7800.0,
            yield_strength: 250e6,
            ultimate_strength: 400e6,
            trl: 9,
        },
        Material {
            name: "Steel (High-Strength)".into(),
            density: 7800.0,
            yield_strength: 690e6,
            ultimate_strength: 760e6,
            trl: 9,
        },
        Material {
            name: "Ti-6Al-4V".into(),
            density: 4430.0,
            yield_strength: 880e6,
            ultimate_strength: 950e6,
            trl: 8,
        },
        Material {
            name: "CFRP Composite".into(),
            density: 1600.0,
            yield_strength: 1500e6,
            ultimate_strength: 2500e6,
            trl: 6,
        },
        Material {
            name: "CNT Composite (theoretical)".into(),
            density: 1400.0,
            yield_strength: 10_000e6,
            ultimate_strength: 60_000e6,
            trl: 3,
        },
    ]
}

/// Structural analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralAnalysis {
    /// Hoop stress in Pascals
    pub hoop_stress: f64,
    /// Safety factor (yield_strength / hoop_stress)
    pub safety_factor: f64,
    /// Required hull thickness for given safety factor, in meters
    pub required_thickness: f64,
    /// Hull mass for the required thickness, in kg
    pub hull_mass: f64,
    /// Whether the material is structurally adequate
    pub adequate: bool,
}

/// Mass breakdown of the station.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassBreakdown {
    /// Structural shell mass in kg
    pub structural_shell: f64,
    /// Radiation shielding mass in kg
    pub radiation_shielding: f64,
    /// Interior soil/substrate mass in kg
    pub interior_substrate: f64,
    /// Atmospheric mass in kg
    pub atmosphere: f64,
    /// Water reserves mass in kg
    pub water_reserves: f64,
    /// Total station mass in kg
    pub total: f64,
}

impl StationConfig {
    /// Calculate hoop stress in the hull due to rotation.
    ///
    /// For a thin-walled pressure vessel under centrifugal loading:
    /// σ = ρ × v² (simplified hoop stress)
    /// where v = ωr (rim velocity)
    ///
    /// More precisely for internal pressure + centrifugal:
    /// σ_hoop = (p × r) / t + ρ_hull × ω² × r² / 2
    pub fn hoop_stress(&self, material: &Material) -> f64 {
        let v = self.rim_velocity();
        material.density * v.powi(2)
    }

    /// Perform structural analysis for a given material.
    pub fn analyze_structure(
        &self,
        material: &Material,
        target_safety_factor: f64,
    ) -> StructuralAnalysis {
        let stress = self.hoop_stress(material);
        let safety_factor = material.yield_strength / stress;
        let adequate = safety_factor >= target_safety_factor;

        // Required thickness: from hoop stress formula σ = pr/t
        // Rearranged: t = (ρ × v² × r) / σ_yield × SF
        let required_thickness =
            (material.density * self.rim_velocity().powi(2) * self.radius)
                / (material.yield_strength / target_safety_factor);

        // Hull mass: cylinder shell M = ρ × 2πr × L × t
        let hull_mass = material.density
            * 2.0
            * std::f64::consts::PI
            * self.radius
            * self.length
            * required_thickness;

        StructuralAnalysis {
            hoop_stress: stress,
            safety_factor,
            required_thickness,
            hull_mass,
            adequate,
        }
    }

    /// Estimate total station mass breakdown.
    pub fn mass_breakdown(&self) -> MassBreakdown {
        let surface_area = 2.0 * std::f64::consts::PI * self.radius * self.length;

        // Structural shell: ~20m thick steel equivalent
        let structural_shell = 7800.0 * surface_area * 8.0; // 8m structural layer

        // Radiation shielding: ~6m of regolith (density ~1500 kg/m³)
        let radiation_shielding = 1500.0 * surface_area * 6.0;

        // Interior substrate: ~2m soil (density ~1300 kg/m³), only on 3 habitable strips
        let interior_substrate = 1300.0 * (surface_area / 2.0) * 2.0;

        // Atmosphere: rough estimate from volume and density
        let volume = std::f64::consts::PI * self.radius.powi(2) * self.length;
        let atmosphere = 0.6 * volume; // ~0.6 kg/m³ average density at half-pressure

        // Water: reserves + ice shielding
        let water_reserves = 5e11; // 500 million tonnes

        let total =
            structural_shell + radiation_shielding + interior_substrate + atmosphere + water_reserves;

        MassBreakdown {
            structural_shell,
            radiation_shielding,
            interior_substrate,
            atmosphere,
            water_reserves,
            total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hoop_stress_steel() {
        let station = StationConfig::new();
        let steel = &material_library()[0]; // A36 steel
        let stress = station.hoop_stress(steel);
        // Expected: 7800 * 198² ≈ 305 MPa
        let stress_mpa = stress / 1e6;
        assert!(stress_mpa > 250.0 && stress_mpa < 350.0);
    }

    #[test]
    fn test_steel_analysis() {
        let station = StationConfig::new();
        let steel = &material_library()[1]; // High-strength steel
        let analysis = station.analyze_structure(steel, 2.0);
        assert!(analysis.safety_factor > 1.0);
    }

    #[test]
    fn test_cfrp_superior() {
        let station = StationConfig::new();
        let steel = &material_library()[0];
        let cfrp = &material_library()[3];

        let steel_sf = station.analyze_structure(steel, 2.0).safety_factor;
        let cfrp_sf = station.analyze_structure(cfrp, 2.0).safety_factor;

        // CFRP should have much better safety factor
        assert!(cfrp_sf > steel_sf);
    }

    #[test]
    fn test_mass_breakdown() {
        let station = StationConfig::new();
        let mass = station.mass_breakdown();
        // Total should be in the billions of tonnes range
        assert!(mass.total > 1e12); // > 1 trillion kg
    }

    #[test]
    fn test_material_library() {
        let materials = material_library();
        assert_eq!(materials.len(), 5);
        // CNT should have highest strength
        let cnt = &materials[4];
        assert!(cnt.yield_strength > 5_000e6);
    }
}
