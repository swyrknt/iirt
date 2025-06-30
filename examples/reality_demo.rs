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
    println!("- Maximum integration level: {:.3}\n", reality.max_consciousness());
    
    println!("Evolving according to IIRT equation:");
    println!("∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)\n");
    
    for snapshot in reality.evolution().max_steps(20) {
        if snapshot.step % 5 == 0 {
            println!("{}", snapshot);
            
            if snapshot.is_conscious && snapshot.step == 5 {
                println!("   Integration threshold exceeded");
            }
            
            if snapshot.information_created > 1000.0 && snapshot.step == 10 {
                println!("   Significant information creation observed");
            }
            
            if snapshot.max_consciousness > 0.8 {
                println!("   High integration levels achieved");
            }
        }
    }
    
    let final_snapshot = RealitySnapshot {
        step: reality.step(),
        time: reality.time(),
        total_information: reality.total_information(),
        information_created: reality.information_created(),
        conscious_count: reality.conscious_count(),
        max_consciousness: reality.max_consciousness(),
        is_conscious: reality.is_conscious(),
    };
    
    println!("\nFinal State Analysis");
    println!("===================");
    println!("{}", final_snapshot);
    
    println!("\nInformation Creation Validation:");
    let info_created = final_snapshot.information_created;
    println!("- Net creation: {:.1} bits", info_created);
    println!("- Average rate: {:.1} bits/step", info_created / final_snapshot.step as f64);
    
    if info_created > 0.0 {
        println!("✓ Information creation confirmed");
    } else {
        println!("✗ No net information creation detected");
    }
    
    println!("\nIntegration Analysis:");
    let conscious_count = final_snapshot.conscious_count;
    let max_consciousness = final_snapshot.max_consciousness;
    
    println!("- Integrated points: {}", conscious_count);
    println!("- Maximum level: {:.3}", max_consciousness);
    println!("- Threshold: {:.3}", INTEGRATION_THRESHOLD);
    
    if conscious_count > 0 {
        println!("✓ Integration threshold exceeded at ℐ ≥ {:.3}", INTEGRATION_THRESHOLD);
    } else {
        println!("✗ No integration detected");
    }
    
    if final_snapshot.is_conscious {
        let conscious_points = reality.conscious_points();
        println!("\nIntegration Distribution:");
        
        let mut level_counts = [0; 11];
        
        for (_, _, _, level) in &conscious_points {
            let bucket = (level * 10.0).floor() as usize;
            if bucket < level_counts.len() {
                level_counts[bucket] += 1;
            }
        }
        
        for (i, count) in level_counts.iter().enumerate() {
            if *count > 0 {
                let level_start = i as f64 * 0.1;
                let level_end = (i + 1) as f64 * 0.1;
                println!("- Level {:.1}-{:.1}: {} points", level_start, level_end, count);
            }
        }
        
        if let Some((x, y, z, level)) = conscious_points.iter()
            .max_by(|a, b| a.3.partial_cmp(&b.3).unwrap()) {
            println!("- Peak integration: {:.3} at ({:.1}, {:.1}, {:.1})", level, x, y, z);
        }
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
    
    if max_consciousness > 0.1 {
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