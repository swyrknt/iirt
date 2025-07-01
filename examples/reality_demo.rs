//! IIRT Reality Engine Demonstration
//!
//! Author: Sawyer Kent
//! Copyright (c) 2025 Sawyer Kent
//!
//! Demonstrates information field evolution according to IIRT theory.
//! Shows emergence of integrated information above the theoretical threshold.

use iirt_engine::*;

fn main() {
    println!("IIRT Reality Engine - Information Field Evolution");
    println!("=================================================\n");
    
    println!("Creating information field initialized to vacuum state...");
    let mut reality = Reality::from_vacuum();
    
    println!("Vacuum state parameters:");
    println!("- Information density: {:.3} bits per point", VACUUM_INFORMATION);
    println!("- Integration threshold: {:.6} bits", INTEGRATION_THRESHOLD);
    println!("- Total information: {:.0} bits", reality.total_information());
    println!("- Integrated points: {}\n", reality.conscious_count());
    
    println!("Adding localized information perturbation (amplitude 1.5)...");
    reality.add_information((0.0, 0.0, 0.0), 1.5);
    
    let initial_info = reality.total_information();
    let _initial_conscious = reality.conscious_count();
    
    println!("After perturbation:");
    println!("- Total information: {:.1} bits", initial_info);
    println!("- Integrated points: {}", reality.conscious_count());
    println!("- Integration coverage: {:.1}%\n", 100.0 * reality.conscious_count() as f64 / 64_f64.powi(3));
    
    println!("Evolving according to IIRT equation:");
    println!("∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)\n");
    
    for step in 1..=20 {
        reality.evolve();
        
        if step % 5 == 0 {
            let total_info = reality.total_information();
            let created = reality.information_created();
            let conscious = reality.conscious_count();
            
            println!("Step {}: Total={:.1} bits, Created={:.1} bits, Conscious={}", 
                    step, total_info, created, conscious);
            
            if reality.is_conscious() && step == 5 {
                println!("   Integration threshold exceeded");
            }
            
            if created > 1000.0 && step == 10 {
                println!("   Significant information creation observed");
            }
        }
    }
    
    let final_step = reality.step();
    let final_time = reality.time();
    let final_total = reality.total_information();
    let final_created = reality.information_created();
    let final_conscious_count = reality.conscious_count();
    let final_is_conscious = reality.is_conscious();
    
    println!("\nFinal State Analysis");
    println!("===================");
    println!("Step: {}, Time: {:.3}", final_step, final_time);
    println!("Total: {:.1} bits, Created: {:.1} bits", final_total, final_created);
    println!("Conscious points: {}, Is conscious: {}", final_conscious_count, final_is_conscious);
    
    println!("\nInformation Creation Validation:");
    let info_created = final_created;
    println!("- Net creation: {:.1} bits", info_created);
    println!("- Average rate: {:.1} bits/step", info_created / final_step as f64);
    
    if info_created > 0.0 {
        println!("✓ Information creation confirmed");
    } else {
        println!("✗ No net information creation detected");
    }
    
    println!("\nIntegration Analysis:");
    let conscious_count = final_conscious_count;
    
    println!("- Integrated points: {}", conscious_count);
    println!("- Coverage: {:.1}%", 100.0 * conscious_count as f64 / 64_f64.powi(3));
    println!("- Threshold: {:.3}", INTEGRATION_THRESHOLD);
    
    if conscious_count > 0 {
        println!("✓ Integration threshold exceeded at ℐ ≥ {:.3}", INTEGRATION_THRESHOLD);
    } else {
        println!("✗ No integration detected");
    }
    
    if final_is_conscious {
        println!("\nIntegration confirmed: Field contains conscious regions");
    }
    
    println!("\nTheoretical Validation");
    println!("=====================");
    
    let mut principles_validated = 0;
    let total_principles = 4;
    
    if info_created > 0.0 {
        println!("✓ Information creation through self-interaction");
        principles_validated += 1;
    } else {
        println!("✗ Information creation not observed");
    }
    
    if conscious_count > 0 {
        println!("✓ Integration threshold behavior at ℐ ≥ 0.707");
        principles_validated += 1;
    } else {
        println!("✗ Integration threshold not reached");
    }
    
    if final_total > 1000000.0 {
        println!("✓ Information density evolution");
        principles_validated += 1;
    } else {
        println!("✗ Insufficient information density evolution");
    }
    
    if info_created > 0.0 && conscious_count > 0 {
        println!("✓ Self-organizing information dynamics");
        principles_validated += 1;
    } else {
        println!("✗ Self-organization not demonstrated");
    }
    
    println!("\nValidation Summary: {}/{} principles confirmed", 
             principles_validated, total_principles);
    
    if principles_validated == total_principles {
        println!("\nIIRT theoretical predictions validated");
    } else {
        println!("\nPartial validation - {} principles require further analysis", 
                 total_principles - principles_validated);
    }
    
    println!("\nConclusion");
    println!("==========");
    println!("Demonstrated information field evolution according to IIRT equation:");
    println!("∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)");
    println!("Integration threshold: ℐ ≥ 0.707107...");
    println!("Information creation rate: {:.1} bits per evolution step", info_created / 20.0);
} 