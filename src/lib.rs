//! IIRT Computational Engine
//! 
//! Implementation of Information Integration Reality Theory (IIRT).
//! 
//! Author: Sawyer Kent
//! Copyright (c) 2025 Sawyer Kent
//!
//! Core equation: ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
//!
//! Integration threshold: ℐ ≥ 0.707107... (1/√2)

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
        
        for snapshot in reality.evolution().max_steps(5) {
            if snapshot.is_conscious {
                assert!(snapshot.information_created > 0.0);
                break;
            }
        }
    }
} 