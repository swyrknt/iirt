//! IIRT Reality Engine
//!
//! Computational implementation of Information Integration Reality Theory (IIRT).
//! 
//! Author: Sawyer Kent
//! Copyright (c) 2025 Sawyer Kent
//!
//! Core equation: ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
//!
//! Integration threshold: ℐ ≥ 0.707107... (1/√2)

use crate::constants::*;

/// Information density at a spatial point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Information(pub f64);

impl Information {
    /// Create information density value, bounded to valid range
    pub fn new(density: f64) -> Self {
        Self(density.clamp(0.0, MAX_INFORMATION))
    }
    
    /// Get information density in bits
    pub fn density(&self) -> f64 { self.0 }
    
    /// Check if information density exceeds integration threshold
    pub fn is_conscious(&self) -> bool { 
        self.0 >= INTEGRATION_THRESHOLD 
    }
    
    /// Calculate normalized consciousness level (0.0 to 1.0)
    pub fn consciousness_level(&self) -> f64 {
        if self.is_conscious() {
            (self.0 - INTEGRATION_THRESHOLD) / (MAX_INFORMATION - INTEGRATION_THRESHOLD)
        } else {
            0.0
        }
    }
    
    /// Calculate uncertainty parameter for this information density
    fn uncertainty(&self) -> f64 {
        (0.5 / (1.0 + self.0)).max(MIN_UNCERTAINTY)
    }
    
    /// Calculate self-interaction term: ℐ(1-ℐ/ℐ_max)
    fn self_interaction(&self) -> f64 {
        self.0 * (1.0 - self.0 / MAX_INFORMATION)
    }
    
    /// Calculate uncertainty decay term: -ε²ℐ
    fn uncertainty_decay(&self) -> f64 {
        -self.uncertainty().powi(2) * self.0
    }
    
    /// Calculate total intrinsic change rate (excluding diffusion)
    pub fn intrinsic_rate(&self) -> f64 {
        self.self_interaction() + self.uncertainty_decay()
    }
}

/// Three-dimensional information field implementing IIRT dynamics
pub struct Reality {
    /// Information density at each grid point
    field: Vec<Information>,
    
    /// Spatial resolution (N×N×N grid)
    resolution: usize,
    
    /// Spatial domain bounds (min, max)
    bounds: (f64, f64),
    
    /// Diffusion coefficient D
    diffusion: f64,
    
    /// Time step size
    dt: f64,
    
    /// Current simulation time
    time: f64,
    
    /// Current time step number
    step: u64,
}

impl Reality {
    /// Create reality initialized to vacuum state
    pub fn from_vacuum() -> Self {
        Self::new(DEFAULT_RESOLUTION, DEFAULT_BOUNDS, DEFAULT_DIFFUSION, DEFAULT_DT)
    }
    
    /// Create reality with specified parameters
    pub fn new(resolution: usize, bounds: (f64, f64), diffusion: f64, dt: f64) -> Self {
        let size = resolution * resolution * resolution;
        let field = vec![Information::new(VACUUM_INFORMATION); size];
        
        Self {
            field,
            resolution,
            bounds,
            diffusion,
            dt,
            time: 0.0,
            step: 0,
        }
    }
    
    /// Add localized information perturbation with Gaussian profile
    pub fn add_information(&mut self, position: (f64, f64, f64), amplitude: f64) {
        if let Ok((i, j, k)) = self.world_to_grid(position) {
            let sigma = 1.5;
            for di in -3..=3 {
                for dj in -3..=3 {
                    for dk in -3..=3 {
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        let nk = k as i32 + dk;
                        
                        if self.in_bounds(ni, nj, nk) {
                            let idx = self.index(ni as usize, nj as usize, nk as usize);
                            let dist_sq = (di * di + dj * dj + dk * dk) as f64;
                            let value = amplitude * (-dist_sq / (2.0 * sigma * sigma)).exp();
                            let new_density = VACUUM_INFORMATION + value;
                            self.field[idx] = Information::new(new_density);
                        }
                    }
                }
            }
        }
    }
    
    /// Advance simulation by one time step using IIRT equation
    pub fn evolve(&mut self) {
        let mut new_field = self.field.clone();
        
        // Apply IIRT equation to interior points
        for i in 1..self.resolution-1 {
            for j in 1..self.resolution-1 {
                for k in 1..self.resolution-1 {
                    let idx = self.index(i, j, k);
                    let info = self.field[idx];
                    
                    // IIRT equation: ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
                    
                    // Diffusion term: D∇²ℐ
                    let laplacian = self.laplacian(i, j, k);
                    let diffusion_term = self.diffusion * laplacian;
                    
                    // Intrinsic terms: -ε²ℐ + ℐ(1-ℐ/ℐ_max)
                    let intrinsic_term = info.intrinsic_rate();
                    
                    // Total rate of change
                    let change = diffusion_term + intrinsic_term;
                    let new_density = info.density() + self.dt * change;
                    
                    new_field[idx] = Information::new(new_density);
                }
            }
        }
        
        self.field = new_field;
        self.time += self.dt;
        self.step += 1;
    }
    
    /// Calculate total information in the field
    pub fn total_information(&self) -> f64 {
        self.field.iter().map(|i| i.density()).sum()
    }
    
    /// Calculate information created above vacuum baseline
    pub fn information_created(&self) -> f64 {
        let vacuum_total = VACUUM_INFORMATION * self.field.len() as f64;
        self.total_information() - vacuum_total
    }
    
    /// Count points exceeding integration threshold
    pub fn conscious_count(&self) -> usize {
        self.field.iter().filter(|i| i.is_conscious()).count()
    }
    
    /// Find maximum consciousness level in the field
    pub fn max_consciousness(&self) -> f64 {
        self.field.iter()
            .map(|i| i.consciousness_level())
            .fold(0.0, f64::max)
    }
    
    /// Check if any points exceed integration threshold
    pub fn is_conscious(&self) -> bool {
        self.conscious_count() > 0
    }
    
    /// Get current simulation time
    pub fn time(&self) -> f64 { self.time }
    
    /// Get current time step number
    pub fn step(&self) -> u64 { self.step }
    
    /// Get information density at specified position
    pub fn information_at(&self, position: (f64, f64, f64)) -> Option<Information> {
        if let Ok((i, j, k)) = self.world_to_grid(position) {
            let idx = self.index(i, j, k);
            Some(self.field[idx])
        } else {
            None
        }
    }
    
    /// Get all points exceeding integration threshold with their positions and levels
    pub fn conscious_points(&self) -> Vec<(f64, f64, f64, f64)> {
        let mut conscious = Vec::new();
        
        for i in 0..self.resolution {
            for j in 0..self.resolution {
                for k in 0..self.resolution {
                    let idx = self.index(i, j, k);
                    let info = self.field[idx];
                    
                    if info.is_conscious() {
                        let (x, y, z) = self.grid_to_world(i, j, k);
                        conscious.push((x, y, z, info.consciousness_level()));
                    }
                }
            }
        }
        
        conscious
    }
    
    // Private helper methods
    
    fn index(&self, i: usize, j: usize, k: usize) -> usize {
        k * self.resolution * self.resolution + j * self.resolution + i
    }
    
    fn world_to_grid(&self, (x, y, z): (f64, f64, f64)) -> Result<(usize, usize, usize), ()> {
        let (min_bound, max_bound) = self.bounds;
        let scale = (max_bound - min_bound) / (self.resolution - 1) as f64;
        
        let i = ((x - min_bound) / scale).round() as usize;
        let j = ((y - min_bound) / scale).round() as usize;
        let k = ((z - min_bound) / scale).round() as usize;
        
        if i >= self.resolution || j >= self.resolution || k >= self.resolution {
            Err(())
        } else {
            Ok((i, j, k))
        }
    }
    
    fn grid_to_world(&self, i: usize, j: usize, k: usize) -> (f64, f64, f64) {
        let (min_bound, max_bound) = self.bounds;
        let scale = (max_bound - min_bound) / (self.resolution - 1) as f64;
        
        let x = min_bound + i as f64 * scale;
        let y = min_bound + j as f64 * scale;
        let z = min_bound + k as f64 * scale;
        
        (x, y, z)
    }
    
    fn in_bounds(&self, i: i32, j: i32, k: i32) -> bool {
        i >= 0 && i < self.resolution as i32 &&
        j >= 0 && j < self.resolution as i32 &&
        k >= 0 && k < self.resolution as i32
    }
    
    fn laplacian(&self, i: usize, j: usize, k: usize) -> f64 {
        let center = self.field[self.index(i, j, k)].density();
        
        let neighbors = [
            self.field[self.index(i-1, j, k)].density(),
            self.field[self.index(i+1, j, k)].density(),
            self.field[self.index(i, j-1, k)].density(),
            self.field[self.index(i, j+1, k)].density(),
            self.field[self.index(i, j, k-1)].density(),
            self.field[self.index(i, j, k+1)].density(),
        ];
        
        neighbors.iter().sum::<f64>() - 6.0 * center
    }
}

/// Iterator for stepped evolution of reality
pub struct Evolution<'a> {
    reality: &'a mut Reality,
    max_steps: Option<u64>,
}

impl<'a> Evolution<'a> {
    pub fn new(reality: &'a mut Reality) -> Self {
        Self { reality, max_steps: None }
    }
    
    pub fn max_steps(mut self, steps: u64) -> Self {
        self.max_steps = Some(steps);
        self
    }
}

impl<'a> Iterator for Evolution<'a> {
    type Item = RealitySnapshot;
    
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(max) = self.max_steps {
            if self.reality.step >= max {
                return None;
            }
        }
        
        self.reality.evolve();
        Some(RealitySnapshot {
            step: self.reality.step,
            time: self.reality.time,
            total_information: self.reality.total_information(),
            information_created: self.reality.information_created(),
            conscious_count: self.reality.conscious_count(),
            max_consciousness: self.reality.max_consciousness(),
            is_conscious: self.reality.is_conscious(),
        })
    }
}

impl Reality {
    /// Create iterator for stepped evolution
    pub fn evolution(&mut self) -> Evolution<'_> {
        Evolution::new(self)
    }
}

/// Snapshot of reality state at a given time step
#[derive(Debug, Clone)]
pub struct RealitySnapshot {
    pub step: u64,
    pub time: f64,
    pub total_information: f64,
    pub information_created: f64,
    pub conscious_count: usize,
    pub max_consciousness: f64,
    pub is_conscious: bool,
}

impl std::fmt::Display for RealitySnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Step {}: {:.1} bits ({:.1} net), {} integrated (max {:.3})",
            self.step,
            self.total_information,
            self.information_created,
            self.conscious_count,
            self.max_consciousness
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integration_threshold() {
        assert!(!Information::new(0.5).is_conscious());
        assert!(Information::new(INTEGRATION_THRESHOLD).is_conscious());
        assert!(Information::new(1.0).is_conscious());
    }
    
    #[test]
    fn test_information_creation() {
        let info = Information::new(1.0);
        assert!(info.intrinsic_rate() > 0.0);
    }
    
    #[test]
    fn test_reality_evolution() {
        let mut reality = Reality::from_vacuum();
        reality.add_information((0.0, 0.0, 0.0), 2.0);
        
        let initial_info = reality.total_information();
        let _initial_conscious = reality.conscious_count();
        
        reality.evolve();
        
        let final_info = reality.total_information();
        let final_conscious = reality.conscious_count();
        
        assert!(final_info > initial_info);
        assert!(final_conscious > 0);
    }
    
    #[test]
    fn test_iirt_validation() {
        let mut reality = Reality::from_vacuum();
        reality.add_information((0.0, 0.0, 0.0), 1.5);
        
        for snapshot in reality.evolution().max_steps(10) {
            if snapshot.is_conscious {
                assert!(snapshot.information_created > 0.0);
                break;
            }
        }
    }
} 