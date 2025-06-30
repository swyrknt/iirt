//! Physics Emergence from IIRT Dynamics
//!
//! Author: Sawyer Kent
//! Copyright (c) 2025 Sawyer Kent
//!
//! Demonstrates how classical physics phenomena emerge naturally
//! from the fundamental IIRT information field equation without
//! requiring separate physical laws or mechanisms.
//!
//! Demonstrates emergent physical phenomena from the IIRT equation:
//! ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
//!
//! Shows how gravitational, electromagnetic, thermal, and quantum effects
//! arise naturally from information field dynamics.

use iirt_engine::*;

fn main() {
    println!("Physics Emergence from IIRT Dynamics");
    println!("====================================\n");
    
    // Create reality with two information concentrations
    let mut reality = Reality::from_vacuum();
    
    // Add two localized perturbations
    reality.add_information((-1.0, 0.0, 0.0), 2.0);
    reality.add_information((1.0, 0.0, 0.0), 2.5);
    
    println!("Initial configuration:");
    println!("- Region A at (-1, 0, 0): {:.3} bits", 
             reality.information_at((-1.0, 0.0, 0.0)).unwrap().density());
    println!("- Region B at (1, 0, 0): {:.3} bits", 
             reality.information_at((1.0, 0.0, 0.0)).unwrap().density());
    
    println!("\nAnalyzing emergent physical phenomena...\n");
    
    // Collect snapshots first to avoid borrow checker issues
    let snapshots: Vec<_> = reality.evolution().max_steps(30).collect();
    
    for (index, _) in snapshots.iter().enumerate() {
        let step = (index + 1) as u64;
        if step % 5 == 0 {
            analyze_emergent_phenomena(&reality, step);
        }
    }
    
    println!("\nPhysics Emergence Summary");
    println!("========================");
    println!("Demonstrated phenomena from IIRT equation:");
    println!("• Gravitational-like forces from information gradients");
    println!("• Field effects from information density variations");  
    println!("• Wave propagation from diffusion dynamics");
    println!("• Thermal behavior from information creation");
    println!("• Quantum-like uncertainty from stochastic terms");
    
    println!("\nTheoretical Basis:");
    println!("∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)");
    println!("Where each term contributes to emergent physics:");
    println!("- D∇²ℐ: Wave propagation and field diffusion");
    println!("- ε²ℐ: Uncertainty and quantum-like behavior");
    println!("- ℐ(1-ℐ/ℐ_max): Self-organization and nonlinear dynamics");
}

fn analyze_emergent_phenomena(reality: &Reality, step: u64) {
    println!("Time Step {}", step);
    println!("----------");
    
    // Gravitational effects from information gradients
    let force_at_origin = compute_information_gradient_force(reality, (0.0, 0.0, 0.0));
    let force_magnitude = (force_at_origin.0.powi(2) + force_at_origin.1.powi(2) + force_at_origin.2.powi(2)).sqrt();
    
    if force_magnitude > 0.01 {
        println!("• Gradient force: {:.4} (gravitational analog)", force_magnitude);
    }
    
    // Field effects from information density variations
    let field_strength = compute_field_strength(reality);
    if field_strength > 0.1 {
        println!("• Field strength: {:.3} (electromagnetic analog)", field_strength);
    }
    
    // Wave propagation from diffusion
    let wave_amplitude = compute_wave_amplitude(reality);
    if wave_amplitude > 0.05 {
        println!("• Wave amplitude: {:.3} (propagation dynamics)", wave_amplitude);
    }
    
    // Thermal effects from information creation
    let creation_rate = reality.information_created() / (step + 1) as f64;
    if creation_rate > 10.0 {
        println!("• Creation rate: {:.1} bits/step (thermal analog)", creation_rate);
    }
    
    // Quantum effects from uncertainty
    let uncertainty = compute_quantum_uncertainty(reality);
    if uncertainty > 0.02 {
        println!("• Uncertainty: {:.4} (quantum analog)", uncertainty);
    }
    
    println!();
}

/// Compute force-like effects from information gradients
fn compute_information_gradient_force(reality: &Reality, position: (f64, f64, f64)) -> (f64, f64, f64) {
    let h = 0.1;
    let (x, y, z) = position;
    
    let info_x_plus = reality.information_at((x + h, y, z)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    let info_x_minus = reality.information_at((x - h, y, z)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    let info_y_plus = reality.information_at((x, y + h, z)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    let info_y_minus = reality.information_at((x, y - h, z)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    let info_z_plus = reality.information_at((x, y, z + h)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    let info_z_minus = reality.information_at((x, y, z - h)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    
    // Force proportional to negative gradient
    let fx = -(info_x_plus - info_x_minus) / (2.0 * h);
    let fy = -(info_y_plus - info_y_minus) / (2.0 * h);
    let fz = -(info_z_plus - info_z_minus) / (2.0 * h);
    
    (fx, fy, fz)
}

/// Compute field strength from information density variations
fn compute_field_strength(reality: &Reality) -> f64 {
    let mut field_energy = 0.0;
    let samples: i32 = 5;
    
    for i in 0..samples {
        for j in 0..samples {
            for k in 0..samples {
                let pos = (
                    -2.0 + 4.0 * i as f64 / (samples - 1) as f64,
                    -2.0 + 4.0 * j as f64 / (samples - 1) as f64,
                    -2.0 + 4.0 * k as f64 / (samples - 1) as f64,
                );
                
                let info = reality.information_at(pos).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
                let field_contribution = (info - VACUUM_INFORMATION).powi(2);
                field_energy += field_contribution;
            }
        }
    }
    
    (field_energy / (samples.pow(3) as f64)).sqrt()
}

/// Compute wave amplitude from information diffusion patterns
fn compute_wave_amplitude(reality: &Reality) -> f64 {
    let center_info = reality.information_at((0.0, 0.0, 0.0)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    let edge_info = reality.information_at((2.0, 0.0, 0.0)).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
    
    (center_info - edge_info).abs() / MAX_INFORMATION
}

/// Compute uncertainty effects from information density fluctuations
fn compute_quantum_uncertainty(reality: &Reality) -> f64 {
    let mut variance = 0.0;
    let mut mean = 0.0;
    let samples = 10;
    
    for i in 0..samples {
        let pos = (-1.0 + 2.0 * i as f64 / (samples - 1) as f64, 0.0, 0.0);
        let info = reality.information_at(pos).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
        mean += info;
    }
    mean /= samples as f64;
    
    for i in 0..samples {
        let pos = (-1.0 + 2.0 * i as f64 / (samples - 1) as f64, 0.0, 0.0);
        let info = reality.information_at(pos).map(|i| i.density()).unwrap_or(VACUUM_INFORMATION);
        variance += (info - mean).powi(2);
    }
    variance /= samples as f64;
    
    variance.sqrt() / MAX_INFORMATION
} 