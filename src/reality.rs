//! IIRT Reality Engine
//!
//! Pure implementation of Information Integration Reality Theory (IIRT).
//! 
//! Core equation: ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
//! Threshold: ℐ_crit = 1/√2

use crate::constants::*;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Information density at a spatial point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Information(pub f64);

impl Information {
    /// Create information density, clamped to valid range
    pub fn new(density: f64) -> Self {
        Self(density.clamp(0.0, MAX_INFORMATION))
    }
    
    /// Get density in bits
    pub fn density(&self) -> f64 { self.0 }
    
    /// Check if exceeds consciousness threshold
    pub fn is_conscious(&self) -> bool { 
        self.0 >= INTEGRATION_THRESHOLD 
    }
    
    /// Uncertainty: ε(ℐ) = max(0.5/(1+ℐ), ε_min)
    fn uncertainty(&self) -> f64 {
        (0.5 / (1.0 + self.0)).max(MIN_UNCERTAINTY)
    }
    
    /// Self-creation: ℐ(1-ℐ/ℐ_max)
    fn self_creation(&self) -> f64 {
        self.0 * (1.0 - self.0 / MAX_INFORMATION)
    }
    
    /// Uncertainty decay: -ε²ℐ
    fn uncertainty_decay(&self) -> f64 {
        -self.uncertainty().powi(2) * self.0
    }
    
    /// Total intrinsic rate: -ε²ℐ + ℐ(1-ℐ/ℐ_max)
    pub fn intrinsic_rate(&self) -> f64 {
        self.self_creation() + self.uncertainty_decay()
    }
}

/// 3D Information field implementing IIRT dynamics
pub struct Reality {
    field: Vec<Information>,
    resolution: usize,
    bounds: (f64, f64),
    diffusion: f64,
    dt: f64,
    time: f64,
    step: u64,
    cosmic_age: f64,
}

impl Reality {
    /// Create new reality field
    pub fn new(resolution: usize, bounds: (f64, f64), diffusion: f64, dt: f64) -> Self {
        Self::new_at_cosmic_age(resolution, bounds, diffusion, dt, CURRENT_COSMIC_AGE_GYR)
    }
    
    /// Create reality at specific cosmic age
    pub fn new_at_cosmic_age(resolution: usize, bounds: (f64, f64), diffusion: f64, dt: f64, cosmic_age: f64) -> Self {
        let size = resolution * resolution * resolution;
        let vacuum = vacuum_at_cosmic_time(cosmic_age);
        let field = vec![Information::new(vacuum); size];
        
        Self {
            field,
            resolution,
            bounds,
            diffusion,
            dt,
            time: 0.0,
            step: 0,
            cosmic_age,
        }
    }
    
    /// Create vacuum reality (current cosmic age)
    pub fn from_vacuum() -> Self {
        Self::new(DEFAULT_RESOLUTION, DEFAULT_BOUNDS, DEFAULT_DIFFUSION, DEFAULT_DT)
    }
    
    /// Create primordial reality (t=0, vacuum at threshold)
    pub fn from_primordial_vacuum() -> Self {
        Self::new_at_cosmic_age(DEFAULT_RESOLUTION, DEFAULT_BOUNDS, DEFAULT_DIFFUSION, DEFAULT_DT, 0.0)
    }
    
    /// Add information at position
    pub fn add_information(&mut self, position: (f64, f64, f64), amplitude: f64) {
        if let Ok(idx) = self.position_to_index(position) {
            let current = self.field[idx].density();
            self.field[idx] = Information::new(current + amplitude);
        }
    }
    
    /// Evolve one time step: ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
    pub fn evolve(&mut self) {
        let mut new_field = self.field.clone();
        
        #[cfg(feature = "parallel")]
        {
            // Parallel version using rayon
            let resolution = self.resolution;
            let diffusion = self.diffusion;
            let dt = self.dt;
            let field = &self.field;
            
            let indices: Vec<_> = (1..resolution-1)
                .flat_map(|i| (1..resolution-1)
                    .flat_map(move |j| (1..resolution-1)
                        .map(move |k| (i, j, k))))
                .collect();
            
            let updates: Vec<_> = indices.par_iter().map(|&(i, j, k)| {
                let idx = k * resolution * resolution + j * resolution + i;
                let info = field[idx];
                
                // Calculate laplacian
                let center = field[idx].density();
                let neighbors = [
                    field[(k * resolution * resolution + j * resolution + (i-1))].density(),
                    field[(k * resolution * resolution + j * resolution + (i+1))].density(),
                    field[(k * resolution * resolution + (j-1) * resolution + i)].density(),
                    field[(k * resolution * resolution + (j+1) * resolution + i)].density(),
                    field[((k-1) * resolution * resolution + j * resolution + i)].density(),
                    field[((k+1) * resolution * resolution + j * resolution + i)].density(),
                ];
                let laplacian = neighbors.iter().sum::<f64>() - 6.0 * center;
                
                // IIRT equation
                let diffusion_term = diffusion * laplacian;
                let intrinsic_term = info.intrinsic_rate();
                let change = diffusion_term + intrinsic_term;
                
                (idx, Information::new(info.density() + dt * change))
            }).collect();
            
            for (idx, new_info) in updates {
                new_field[idx] = new_info;
            }
        }
        
        #[cfg(not(feature = "parallel"))]
        {
            // Sequential version
            for i in 1..self.resolution-1 {
                for j in 1..self.resolution-1 {
                    for k in 1..self.resolution-1 {
                        let idx = self.index(i, j, k);
                        let info = self.field[idx];
                        
                        // IIRT equation
                        let laplacian = self.laplacian(i, j, k);
                        let diffusion_term = self.diffusion * laplacian;
                        let intrinsic_term = info.intrinsic_rate();
                        let change = diffusion_term + intrinsic_term;
                        
                        new_field[idx] = Information::new(info.density() + self.dt * change);
                    }
                }
            }
        }
        
        self.field = new_field;
        self.time += self.dt;
        self.step += 1;
    }
    
    /// Get information at position
    pub fn information_at(&self, position: (f64, f64, f64)) -> Option<Information> {
        self.position_to_index(position).ok().map(|idx| self.field[idx])
    }
    
    /// Total information in field
    pub fn total_information(&self) -> f64 {
        #[cfg(feature = "parallel")]
        {
            self.field.par_iter().map(|i| i.density()).sum()
        }
        #[cfg(not(feature = "parallel"))]
        {
            self.field.iter().map(|i| i.density()).sum()
        }
    }
    
    /// Count conscious points
    pub fn conscious_count(&self) -> usize {
        #[cfg(feature = "parallel")]
        {
            self.field.par_iter().filter(|i| i.is_conscious()).count()
        }
        #[cfg(not(feature = "parallel"))]
        {
            self.field.iter().filter(|i| i.is_conscious()).count()
        }
    }
    
    /// Check if any point is conscious
    pub fn is_conscious(&self) -> bool {
        self.conscious_count() > 0
    }
    
    /// Current vacuum density
    pub fn vacuum_density(&self) -> f64 {
        vacuum_at_cosmic_time(self.cosmic_age)
    }
    
    /// Information created above vacuum
    pub fn information_created(&self) -> f64 {
        let vacuum_total = self.vacuum_density() * self.field.len() as f64;
        self.total_information() - vacuum_total
    }
    
    /// Get current time
    pub fn time(&self) -> f64 { self.time }
    
    /// Get step count  
    pub fn step(&self) -> u64 { self.step }
    
    /// Get cosmic age
    pub fn cosmic_age(&self) -> f64 { self.cosmic_age }
    
    // Private helpers
    
    fn index(&self, i: usize, j: usize, k: usize) -> usize {
        k * self.resolution * self.resolution + j * self.resolution + i
    }
    
    fn position_to_index(&self, (x, y, z): (f64, f64, f64)) -> Result<usize, ()> {
        let (min_bound, max_bound) = self.bounds;
        let scale = (max_bound - min_bound) / (self.resolution - 1) as f64;
        
        let i = ((x - min_bound) / scale).round() as usize;
        let j = ((y - min_bound) / scale).round() as usize;
        let k = ((z - min_bound) / scale).round() as usize;
        
        if i >= self.resolution || j >= self.resolution || k >= self.resolution {
            Err(())
        } else {
            Ok(self.index(i, j, k))
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_consciousness_threshold() {
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
    fn test_iirt_equation() {
        let mut reality = Reality::from_vacuum();
        reality.add_information((0.0, 0.0, 0.0), 2.0);
        
        let initial = reality.total_information();
        reality.evolve();
        let final_info = reality.total_information();
        
        assert!(final_info > initial);
        assert!(reality.conscious_count() > 0);
    }
}