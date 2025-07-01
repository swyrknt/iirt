//! IIRT Computational Engine
//! 
//! Computational implementation of Information Integration Reality Theory (IIRT).
//! 
//! # Theory
//! 
//! IIRT posits that reality emerges from dynamics of an information density field ℐ(x,t)
//! governed by the master equation:
//! 
//! ```text
//! ∂ℐ/∂t = D∇²ℐ - ε²(ℐ)ℐ + ℐ(1-ℐ/ℐ_max)
//! ```
//! 
//! THE FUNDAMENTAL EQUATION OF EVERYTHING:
//! ```text
//! ℐ_vacuum(t) = ℐ_threshold × e^(α × t)
//! ```
//! 
//! Where:
//! - D: Information diffusion coefficient
//! - ε(ℐ) = 0.5/(1+ℐ): Uncertainty function (Gödel incompleteness)
//! - ℐ_max = 16.0 bits: Maximum information density
//! - ℐ_threshold = 1/√2: Consciousness threshold (only possible initial condition)
//! - α = 0.2032 per Gyr: Exponential growth rate from self-amplification
//! 
//! # Key Predictions
//! 
//! - Self-referential threshold: ℐ_crit = 1/√2 ≈ 0.707107 bits
//! - Vacuum starts at threshold and grows exponentially (no arbitrary constants)
//! - Dark energy acceleration emerges naturally from conscious vacuum
//! - Electromagnetic coupling via information gradients and currents
//! 
//! # Author
//! 
//! Sawyer Kent, 2025

pub mod constants;
pub mod reality;

// Re-export main components
pub use reality::*;
pub use constants::*;

/// Create reality field initialized to vacuum state
pub fn vacuum_reality() -> Reality {
    Reality::from_vacuum()
}

/// Create reality field with localized information perturbation
pub fn reality_with_information(position: (f64, f64, f64), amplitude: f64) -> Reality {
    let mut reality = Reality::from_vacuum();
    reality.add_information(position, amplitude);
    reality
}

/// Create high-performance reality field (larger grid if parallel processing enabled)
pub fn high_performance_reality() -> Reality {
    #[cfg(feature = "parallel")]
    {
        // Use larger grid with parallel processing
        Reality::new(96, (-6.0, 6.0), 1.0, 0.001)
    }
    #[cfg(not(feature = "parallel"))]
    {
        // Standard grid for sequential processing
        Reality::from_vacuum()
    }
}

/// Create reality field with electromagnetic analysis capabilities
pub fn electromagnetic_reality() -> Reality {
    // Use higher resolution for better gradient calculation
    Reality::new(48, (-3.0, 3.0), 1.0, 0.005)
}

/// Create reality field optimized for consciousness-EM field studies
pub fn consciousness_em_reality(brain_positions: Vec<(f64, f64, f64)>) -> Reality {
    let mut reality = electromagnetic_reality();
    
    // Add consciousness at each brain position
    for position in brain_positions {
        // Typical conscious brain: ~2 bits above vacuum
        reality.add_information(position, 2.0);
    }
    
    reality
}

/// Create reality at specific cosmic epoch
/// 
/// Enables studies of cosmic evolution and dark energy development.
/// Uses appropriate evolved vacuum density for the specified time.
pub fn cosmic_reality(cosmic_age_gyr: f64) -> Reality {
    Reality::new_at_cosmic_age(DEFAULT_RESOLUTION, DEFAULT_BOUNDS, 
                              DEFAULT_DIFFUSION, DEFAULT_DT, cosmic_age_gyr)
}

/// Create reality at Big Bang epoch (t=0)
/// 
/// Uses primordial vacuum at consciousness threshold (1/√2 bits).
/// This is the fundamental starting point - the minimal conscious state
/// that immediately begins exponential self-amplification.
pub fn primordial_reality() -> Reality {
    Reality::from_primordial_vacuum()
}

/// Create high-performance reality at specific cosmic epoch
pub fn high_performance_cosmic_reality(cosmic_age_gyr: f64) -> Reality {
    #[cfg(feature = "parallel")]
    {
        Reality::new_at_cosmic_age(96, (-6.0, 6.0), 1.0, 0.001, cosmic_age_gyr)
    }
    #[cfg(not(feature = "parallel"))]
    {
        cosmic_reality(cosmic_age_gyr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integration_threshold() {
        let conscious_info = Information::new(1.0);
        assert!(conscious_info.is_conscious());
        
        let unconscious_info = Information::new(0.5);
        assert!(!unconscious_info.is_conscious());
    }
    
    #[test]
    fn test_evolution_dynamics() {
        let mut reality = reality_with_information((0.0, 0.0, 0.0), 1.5);
        
        let initial_info = reality.total_information();
        let _initial_conscious = reality.conscious_count();
        
        reality.evolve();
        
        let final_info = reality.total_information();
        let final_conscious = reality.conscious_count();
        
        assert!(final_info > initial_info);
        assert!(final_conscious > 0);
    }
    
    #[test]
    fn test_information_creation() {
        let mut reality = vacuum_reality();
        reality.add_information((0.0, 0.0, 0.0), 2.0);
        
        let initial = reality.information_created();
        for _ in 0..10 {
            reality.evolve();
            if reality.is_conscious() {
                assert!(reality.information_created() > initial);
                break;
            }
        }
    }
    
    #[test]
    fn test_consciousness_detection() {
        let mut reality = electromagnetic_reality();
        
        // Add consciousness (brain-like information density)
        reality.add_information((0.0, 0.0, 0.0), 2.0);
        
        // Evolve to establish patterns
        for _ in 0..20 {
            reality.evolve();
        }
        
        // Check that consciousness is detected
        assert!(reality.is_conscious());
        assert!(reality.conscious_count() > 0);
        
        // Check information at center should be above threshold
        if let Some(info) = reality.information_at((0.0, 0.0, 0.0)) {
            assert!(info.is_conscious());
        }
    }
    
    #[test]
    fn test_multiple_consciousness_centers() {
        // Test multiple consciousness centers
        let brain_positions = vec![(0.0, 0.0, 0.0), (1.0, 0.0, 0.0)];
        let mut reality = consciousness_em_reality(brain_positions);
        
        // Evolve to establish patterns
        for _ in 0..30 {
            reality.evolve();
        }
        
        // Check that both consciousness centers are detected
        assert!(reality.is_conscious());
        assert!(reality.conscious_count() > 100); // Should have many conscious points
        
        // Check information at both centers
        if let Some(info1) = reality.information_at((0.0, 0.0, 0.0)) {
            assert!(info1.is_conscious());
        }
        if let Some(info2) = reality.information_at((1.0, 0.0, 0.0)) {
            assert!(info2.is_conscious());
        }
    }
} 