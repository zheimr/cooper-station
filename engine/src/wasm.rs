use wasm_bindgen::prelude::*;

use crate::gravity::ZONE_DEFINITIONS;
use crate::station::StationConfig;
use crate::structure::material_library;

/// Get the default station configuration as JSON.
#[wasm_bindgen]
pub fn station_config_json() -> String {
    let station = StationConfig::new();
    serde_json::to_string(&station).unwrap_or_default()
}

/// Get all derived station parameters as JSON.
#[wasm_bindgen]
pub fn station_derived_json() -> String {
    let station = StationConfig::new();
    let derived = station.derive();
    serde_json::to_string(&derived).unwrap_or_default()
}

/// Angular velocity in rad/s.
#[wasm_bindgen]
pub fn station_omega() -> f64 {
    StationConfig::new().omega()
}

/// Rotation rate in RPM.
#[wasm_bindgen]
pub fn station_rpm() -> f64 {
    StationConfig::new().rpm()
}

/// Rotation period in seconds.
#[wasm_bindgen]
pub fn station_period() -> f64 {
    StationConfig::new().period()
}

/// Rim velocity in m/s.
#[wasm_bindgen]
pub fn rim_velocity() -> f64 {
    StationConfig::new().rim_velocity()
}

/// Gravity at a given radius (meters) in m/s².
#[wasm_bindgen]
pub fn gravity_at(radius: f64) -> f64 {
    StationConfig::new().gravity_at_radius(radius)
}

/// Gravity as fraction of Earth g at a given radius.
#[wasm_bindgen]
pub fn gravity_fraction_at(radius: f64) -> f64 {
    StationConfig::new().gravity_fraction_at_radius(radius)
}

/// Get all gravity zones as JSON array.
#[wasm_bindgen]
pub fn gravity_zones_json() -> String {
    let station = StationConfig::new();
    let zones = station.gravity_zones();
    serde_json::to_string(&zones).unwrap_or_default()
}

/// Coriolis acceleration for a given radial velocity (m/s).
#[wasm_bindgen]
pub fn coriolis_acceleration(radial_velocity: f64) -> f64 {
    StationConfig::new().coriolis_acceleration(radial_velocity)
}

/// Full Coriolis effect details as JSON.
#[wasm_bindgen]
pub fn coriolis_effect_json(radial_velocity: f64) -> String {
    let station = StationConfig::new();
    let effect = station.coriolis_effect(radial_velocity);
    serde_json::to_string(&effect).unwrap_or_default()
}

/// Atmospheric pressure at a given radius (Pascals). Temperature defaults to 293K.
#[wasm_bindgen]
pub fn pressure_at(radius: f64) -> f64 {
    StationConfig::new().pressure_at_radius(radius, 293.0)
}

/// Full atmospheric conditions at a given radius as JSON.
#[wasm_bindgen]
pub fn atmosphere_at_json(radius: f64) -> String {
    let station = StationConfig::new();
    let atm = station.atmosphere_at_radius(radius, 293.0);
    serde_json::to_string(&atm).unwrap_or_default()
}

/// ECLSS budget as JSON.
#[wasm_bindgen]
pub fn eclss_budget_json() -> String {
    let station = StationConfig::new();
    let budget = station.eclss_budget(293.0);
    serde_json::to_string(&budget).unwrap_or_default()
}

/// Hoop stress for a given material index (0-4) in Pascals.
#[wasm_bindgen]
pub fn hoop_stress(material_index: usize) -> f64 {
    let materials = material_library();
    if material_index >= materials.len() {
        return 0.0;
    }
    StationConfig::new().hoop_stress(&materials[material_index])
}

/// Structural analysis for a given material index as JSON.
#[wasm_bindgen]
pub fn structural_analysis_json(material_index: usize, safety_factor: f64) -> String {
    let materials = material_library();
    if material_index >= materials.len() {
        return "{}".to_string();
    }
    let station = StationConfig::new();
    let analysis = station.analyze_structure(&materials[material_index], safety_factor);
    serde_json::to_string(&analysis).unwrap_or_default()
}

/// Mass breakdown as JSON.
#[wasm_bindgen]
pub fn mass_breakdown_json() -> String {
    let station = StationConfig::new();
    let mass = station.mass_breakdown();
    serde_json::to_string(&mass).unwrap_or_default()
}

/// Drop trajectory simulation as JSON array of [time, radius, deflection].
#[wasm_bindgen]
pub fn drop_trajectory_json(drop_height: f64, initial_radius: f64, steps: usize) -> String {
    let station = StationConfig::new();
    let traj = station.drop_trajectory(drop_height, initial_radius, steps);
    serde_json::to_string(&traj).unwrap_or_default()
}

/// Radius at which a given number of gravity zones are defined.
#[wasm_bindgen]
pub fn zone_count() -> usize {
    ZONE_DEFINITIONS.len()
}

/// Tangential weight variation fraction for a given walking speed.
#[wasm_bindgen]
pub fn tangential_weight_variation(speed: f64) -> f64 {
    StationConfig::new().tangential_weight_variation(speed)
}

/// Custom station: gravity at radius for custom station parameters.
#[wasm_bindgen]
pub fn custom_gravity_at(length: f64, radius: f64, target_g: f64, query_radius: f64) -> f64 {
    let station = StationConfig::custom(length, radius, target_g);
    station.gravity_at_radius(query_radius)
}

/// Custom station: derived parameters as JSON.
#[wasm_bindgen]
pub fn custom_station_derived_json(length: f64, radius: f64, target_g: f64) -> String {
    let station = StationConfig::custom(length, radius, target_g);
    let derived = station.derive();
    serde_json::to_string(&derived).unwrap_or_default()
}
