use serde::{Deserialize, Serialize};

use crate::station::StationConfig;

/// Result of a Coriolis effect calculation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoriolisEffect {
    /// Radial velocity of the moving object in m/s
    pub radial_velocity: f64,
    /// Coriolis acceleration magnitude in m/s²
    pub acceleration: f64,
    /// Lateral deflection over a given distance in meters
    pub deflection_per_10m: f64,
    /// Whether the effect is noticeable to humans
    pub perception: CoriolisPerception,
}

/// Human perception level of Coriolis effects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoriolisPerception {
    /// Below human detection threshold
    Imperceptible,
    /// Detectable but not disorienting
    Noticeable,
    /// Clearly felt, may cause brief disorientation
    Significant,
    /// Strong effect, requires compensation in design
    Dramatic,
}

impl StationConfig {
    /// Calculate Coriolis acceleration for radial motion.
    ///
    /// a_coriolis = 2ωv (magnitude, for motion perpendicular to rotation axis)
    ///
    /// # Arguments
    /// * `radial_velocity` - Speed of radial motion in m/s (toward/away from axis)
    pub fn coriolis_acceleration(&self, radial_velocity: f64) -> f64 {
        2.0 * self.omega() * radial_velocity.abs()
    }

    /// Calculate full Coriolis effect details for a given radial velocity.
    pub fn coriolis_effect(&self, radial_velocity: f64) -> CoriolisEffect {
        let accel = self.coriolis_acceleration(radial_velocity);

        // Deflection over 10m of radial travel:
        // time = distance / velocity
        // deflection = 0.5 * a * t²
        let travel_dist = 10.0;
        let time = travel_dist / radial_velocity.abs().max(0.001);
        let deflection = 0.5 * accel * time.powi(2);

        let perception = if accel < 0.05 {
            CoriolisPerception::Imperceptible
        } else if accel < 0.2 {
            CoriolisPerception::Noticeable
        } else if accel < 0.5 {
            CoriolisPerception::Significant
        } else {
            CoriolisPerception::Dramatic
        };

        CoriolisEffect {
            radial_velocity,
            acceleration: accel,
            deflection_per_10m: deflection,
            perception,
        }
    }

    /// Calculate spinward/anti-spinward weight variation at a given tangential speed.
    ///
    /// Walking spinward: effective g decreases (feel lighter)
    /// Walking anti-spinward: effective g increases (feel heavier)
    ///
    /// Returns (delta_g_fraction) — the fractional change in apparent gravity.
    /// Positive means heavier (anti-spinward), negative means lighter (spinward).
    pub fn tangential_weight_variation(&self, tangential_speed: f64) -> f64 {
        // Effective gravity when moving tangentially:
        // g_eff = ω² r ± 2ωv (simplified for small v compared to rim speed)
        // Δg/g = 2v/(ωr) = 2v/v_rim
        2.0 * tangential_speed / self.rim_velocity()
    }

    /// Simulate a dropped object's trajectory in the rotating frame.
    ///
    /// Returns a Vec of (time, radial_pos, tangential_deflection) tuples.
    /// Radial position is distance from axis, tangential deflection is Coriolis-induced.
    pub fn drop_trajectory(
        &self,
        drop_height: f64,
        initial_radius: f64,
        time_steps: usize,
    ) -> Vec<(f64, f64, f64)> {
        let omega = self.omega();
        let g_local = omega.powi(2) * initial_radius;

        // Time to fall: t = sqrt(2h/g)
        let fall_time = (2.0 * drop_height / g_local).sqrt();
        let dt = fall_time / time_steps as f64;

        let mut trajectory = Vec::with_capacity(time_steps + 1);
        let mut r = initial_radius;
        let mut vr: f64 = 0.0; // radial velocity
        let mut tangential_offset = 0.0;

        for i in 0..=time_steps {
            let t = i as f64 * dt;
            trajectory.push((t, r, tangential_offset));

            // Radial acceleration (gravity at current r)
            let g_r = omega.powi(2) * r;
            vr += g_r * dt; // falling toward rim (increasing r)

            // Coriolis deflection
            let a_coriolis = 2.0 * omega * vr;
            tangential_offset += a_coriolis * dt * dt * 0.5;

            r += vr * dt;
            if r >= self.radius {
                break;
            }
        }

        trajectory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coriolis_walking() {
        let station = StationConfig::new();
        let effect = station.coriolis_effect(1.5);
        // Expected: ~0.149 m/s²
        assert!((effect.acceleration - 0.149).abs() < 0.01);
        assert_eq!(effect.perception, CoriolisPerception::Noticeable);
    }

    #[test]
    fn test_coriolis_elevator() {
        let station = StationConfig::new();
        let effect = station.coriolis_effect(10.0);
        // Expected: ~0.99 m/s²
        assert!((effect.acceleration - 0.99).abs() < 0.02);
        assert_eq!(effect.perception, CoriolisPerception::Dramatic);
    }

    #[test]
    fn test_tangential_weight_variation() {
        let station = StationConfig::new();
        let delta = station.tangential_weight_variation(1.5);
        // Expected: ~1.5% variation
        assert!((delta - 0.015).abs() < 0.005);
    }

    #[test]
    fn test_drop_trajectory_length() {
        let station = StationConfig::new();
        let traj = station.drop_trajectory(2.0, 3990.0, 100);
        assert!(!traj.is_empty());
        assert!(traj.len() <= 101);
    }

    #[test]
    fn test_drop_trajectory_coriolis_deflection() {
        let station = StationConfig::new();
        let traj = station.drop_trajectory(2.0, 3990.0, 100);
        // Last point should have non-zero tangential deflection
        let last = traj.last().unwrap();
        assert!(last.2.abs() > 0.0, "Dropped object should deflect tangentially");
    }
}
