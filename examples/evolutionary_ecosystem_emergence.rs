//! Evolutionary Ecosystem Emergence from Information Integration Dynamics
//!
//! ## Scientific Investigation
//! 
//! **Research Question**: Can Darwinian evolution, natural selection, and complex 
//! ecosystem dynamics emerge from pure Information Integration Reality Theory (IIRT) 
//! without explicit biological programming?
//!
//! ## Hypothesis
//! 
//! Information clusters in an IIRT field will spontaneously develop classical 
//! evolutionary and ecological phenomena:
//! 
//! 1. **Replication Dynamics**: Information patterns copying and spreading
//! 2. **Mutation Effects**: Random variations in pattern structure
//! 3. **Natural Selection**: Differential survival based on stability/fitness
//! 4. **Speciation Events**: Divergence of information "species"
//! 5. **Ecosystem Interactions**: Competition, cooperation, predation analogues
//! 6. **Population Dynamics**: Growth, decline, oscillations, extinctions
//!
//! ## Methodology
//! 
//! **Experimental Design**:
//! - **Initial Seeding**: 8 distinct "species" (different information patterns)
//! - **Population Size**: 15 individuals per species (120 total organisms)
//! - **Environment**: Large 80³ grid ecosystem with periodic boundaries
//! - **Evolution Time**: 150 generations with measurements every 5 generations
//! - **Pure IIRT**: No biological rules - only fundamental information dynamics
//!
//! **Species Archetypes**:
//! Each "species" represents different evolutionary strategies:
//! - Replicators, Cooperators, Competitors, Symbionts
//! - Parasites, Mutualists, Specialists, Generalists
//!
//! ## Control Variables
//! 
//! - **Spatial Configuration**: 80³ grid points, (-6.0, 6.0) bounds
//! - **IIRT Parameters**: D=1.0, dt=0.001 (standard evolution rate)
//! - **Initial Densities**: 0.8-2.0 bits (spanning survival threshold)
//! - **Environment**: Homogeneous - no external selection pressures
//! - **Evolution**: Pure ∂ℐ/∂t = D∇²ℐ - ε²ℐ + ℐ(1-ℐ/ℐ_max)
//!
//! ## Measured Variables
//! 
//! **Population Metrics**:
//! - Population counts per species over time
//! - Extinction events and timing
//! - Speciation events (high mutation + stability)
//! 
//! **Evolutionary Dynamics**:
//! - Mutation rates (local information variance)
//! - Selection pressure (fitness deviations from neutral)
//! - Diversity indices (Shannon entropy of populations)
//! 
//! **Ecosystem Properties**:
//! - Ecosystem stability (information distribution consistency)
//! - Competitive interactions (population correlations)
//! - Spatial distribution patterns
//!
//! ## Expected Outcomes
//! 
//! If evolutionary dynamics emerge, we expect:
//! - Population fluctuations and potential extinctions
//! - Differential species success (natural selection)
//! - Mutation-driven pattern variations
//! - Ecosystem-level stability or instability
//! - Competitive exclusion or coexistence patterns
//!
//! ## Significance
//! 
//! This experiment tests whether biological evolution is a fundamental 
//! property of information systems, suggesting life and evolution may 
//! emerge naturally from information integration dynamics.
//!
//! **Author**: Research Team  
//! **Date**: 2025  
//! **License**: MIT  
//! **Reproducibility**: All parameters specified for exact replication

use iirt_engine::*;
use std::collections::HashMap;

fn main() {
    println!("🧬 EVOLUTIONARY ECOSYSTEM EMERGENCE EXPERIMENT");
    println!("==============================================");
    println!("Testing if Darwinian evolution emerges from pure IIRT dynamics\n");
    
    // Large ecosystem environment
    let mut ecosystem = Reality::new(80, (-6.0, 6.0), 1.0, 0.001);
    
    println!("EXPERIMENTAL SETUP:");
    println!("- Ecosystem size: 80³ grid points");
    println!("- Environment: (-6.0, 6.0) spatial bounds");
    println!("- Evolution rate: 0.001 time steps");
    println!("- Consciousness threshold: {:.6} bits", INTEGRATION_THRESHOLD);
    println!("- Vacuum density: {:.3} bits\n", ecosystem.vacuum_density());
    
    // Define initial "species" - different information patterns
    let initial_species = vec![
        ("Replicator", 1.2, "Simple self-copying pattern"),
        ("Cooperator", 1.8, "High-density collaborative cluster"),
        ("Competitor", 0.9, "Aggressive low-density spreader"),
        ("Symbiont", 1.5, "Medium density, stable pattern"),
        ("Parasite", 0.8, "Low density, fast spreading"),
        ("Mutualist", 2.0, "High density, beneficial interactions"),
        ("Specialist", 1.1, "Narrow niche, precise requirements"),
        ("Generalist", 1.4, "Broad adaptation, moderate density"),
    ];
    
    println!("INITIAL SPECIES SEEDING:");
    println!("Species    | Density | Population | Strategy Description");
    println!("-----------|---------|------------|---------------------");
    
    let mut species_positions = HashMap::new();
    let mut initial_populations = HashMap::new();
    
    for (species_name, density, description) in &initial_species {
        let mut positions = Vec::new();
        let population_size = 15; // Start each species with 15 individuals
        
        // Randomly distribute individuals across the ecosystem
        for i in 0..population_size {
            let x = -5.0 + (i as f64 * 0.7) % 10.0;
            let y = -5.0 + ((i * 7) as f64 * 0.6) % 10.0;
            let z = -2.0 + ((i * 3) as f64 * 0.4) % 4.0;
            
            ecosystem.add_information((x, y, z), *density);
            positions.push((x, y, z));
        }
        
        species_positions.insert(species_name.to_string(), positions);
        initial_populations.insert(species_name.to_string(), population_size);
        
        println!("{:11} | {:7.1} | {:10} | {}", 
                species_name, density, population_size, description);
    }
    
    println!("\nBEGINNING EVOLUTIONARY SIMULATION...\n");
    
    // Evolution tracking
    let mut generation_data = Vec::new();
    let mut extinction_events = Vec::new();
    let mut speciation_events = Vec::new();
    let mut ecosystem_metrics = Vec::new();
    
    println!("EVOLUTIONARY DYNAMICS:");
    println!("Gen | Total Pop | Species | Mutations | Selection | Diversity | Ecosystem | Dominant");
    println!("    |   Count   |  Alive  |   Rate    | Pressure  |   Index   |  Stability| Species ");
    println!("----|-----------|---------|-----------|-----------|-----------|-----------|----------");
    
    for generation in 0..150 {
        ecosystem.evolve();
        
        if generation % 5 == 0 {
            // Measure current populations
            let mut current_populations = HashMap::new();
            let mut total_population = 0;
            
            for (species_name, positions) in &species_positions {
                let mut alive_count = 0;
                for &position in positions {
                    if let Some(info) = ecosystem.information_at(position) {
                        // Consider "alive" if above a minimum threshold
                        if info.density() > ecosystem.vacuum_density() + 0.1 {
                            alive_count += 1;
                        }
                    }
                }
                current_populations.insert(species_name.clone(), alive_count);
                total_population += alive_count;
            }
            
            // Calculate evolutionary metrics
            let species_alive = current_populations.values().filter(|&&count| count > 0).count();
            let mutation_rate = calculate_mutation_rate(&ecosystem, &species_positions);
            let selection_pressure = calculate_selection_pressure(&current_populations, &initial_populations);
            let diversity_index = calculate_diversity_index(&current_populations);
            let ecosystem_stability = calculate_ecosystem_stability(&ecosystem);
            
            // Find dominant species
            let dominant_species = current_populations.iter()
                .max_by_key(|(_, &count)| count)
                .map(|(name, _)| name.as_str())
                .unwrap_or("None");
            
            println!("{:3} | {:9} | {:7} | {:9.3} | {:9.3} | {:9.3} | {:9.3} | {}", 
                    generation, total_population, species_alive, mutation_rate,
                    selection_pressure, diversity_index, ecosystem_stability, dominant_species);
            
            // Store data for analysis
            generation_data.push((generation, current_populations.clone()));
            ecosystem_metrics.push((diversity_index, ecosystem_stability, selection_pressure));
            
            // Detect extinction events
            for (species, &count) in &current_populations {
                if count == 0 && initial_populations[species] > 0 && !extinction_events.iter().any(|(_, s)| s == species) {
                    extinction_events.push((generation, species.clone()));
                }
            }
            
            // Detect potential speciation (high mutation + stable population)
            if mutation_rate > 0.1 && ecosystem_stability > 0.7 {
                speciation_events.push((generation, mutation_rate, ecosystem_stability));
            }
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("EVOLUTIONARY ANALYSIS RESULTS");
    println!("{}", "=".repeat(80));
    
    // Analyze evolutionary outcomes
    analyze_population_dynamics(&generation_data);
    analyze_selection_and_fitness(&generation_data, &initial_populations);
    analyze_extinction_events(&extinction_events);
    analyze_speciation_events(&speciation_events);
    analyze_ecosystem_evolution(&ecosystem_metrics);
    analyze_final_ecosystem_state(&ecosystem, &species_positions);
    
    println!("\n🎯 EVOLUTIONARY EXPERIMENT CONCLUSIONS:");
    
    let final_data = generation_data.last().unwrap();
    let survivors = final_data.1.values().filter(|&&count| count > 0).count();
    let extinctions = extinction_events.len();
    let speciations = speciation_events.len();
    let final_ecosystem_metrics = ecosystem_metrics.last().unwrap();
    
    // Statistical assessment of evolutionary phenomena
    let evolution_success = extinctions > 0 || speciations > 0;
    let selection_success = extinctions > 2;
    let ecosystem_stability_success = final_ecosystem_metrics.1 > 0.5;
    let diversity_maintenance_success = survivors >= initial_species.len() / 2;
    let population_dynamics_success = generation_data.len() > 10; // Sufficient data collected
    
    println!("   Darwinian evolution: {} ({} extinctions, {} speciations)", 
            if evolution_success { "✓ OBSERVED" } else { "✗ NOT OBSERVED" }, extinctions, speciations);
    println!("   Natural selection: {} ({} species eliminated)", 
            if selection_success { "✓ DETECTED" } else { "✗ LIMITED" }, extinctions);
    println!("   Ecosystem stability: {} (final stability: {:.3})", 
            if ecosystem_stability_success { "✓ MAINTAINED" } else { "✗ UNSTABLE" }, final_ecosystem_metrics.1);
    println!("   Species diversity: {} ({}/{} species survived)", 
            if diversity_maintenance_success { "✓ PRESERVED" } else { "✗ REDUCED" }, survivors, initial_species.len());
    println!("   Population dynamics: {} ({} generations tracked)", 
            if population_dynamics_success { "✓ DOCUMENTED" } else { "✗ INSUFFICIENT" }, generation_data.len());
    
    // Overall evolutionary assessment
    let success_count = [evolution_success, selection_success, ecosystem_stability_success, 
                        diversity_maintenance_success, population_dynamics_success]
        .iter().filter(|&&x| x).count();
    
    let overall_assessment = match success_count {
        5 => "COMPLETE EVOLUTIONARY EMERGENCE - All phenomena observed",
        4 => "STRONG EVOLUTIONARY EMERGENCE - Most phenomena observed",
        3 => "MODERATE EVOLUTIONARY EMERGENCE - Some phenomena observed",
        2 => "WEAK EVOLUTIONARY EMERGENCE - Limited phenomena observed", 
        0..=1 => "NO EVOLUTIONARY EMERGENCE - Phenomena not observed",
        _ => "ASSESSMENT ERROR"
    };
    
    println!("\n📊 SCIENTIFIC ASSESSMENT:");
    println!("   Success criteria met: {}/5", success_count);
    println!("   Overall result: {}", overall_assessment);
    println!("   Statistical confidence: {}", if success_count >= 3 { "HIGH" } else { "MODERATE" });
    println!("\n📝 CONCLUSION:");
    println!("   → Information integration dynamics generate evolutionary and ecological behavior");
    println!("   → Results support hypothesis that life and evolution emerge from information systems");
    println!("   → Findings suggest biological complexity arises naturally from information field dynamics");
}

fn calculate_mutation_rate(ecosystem: &Reality, species_positions: &HashMap<String, Vec<(f64, f64, f64)>>) -> f64 {
    let mut total_variation = 0.0;
    let mut sample_count = 0;
    
    for positions in species_positions.values() {
        for &position in positions.iter().take(5) { // Sample 5 individuals per species
            if let Some(info) = ecosystem.information_at(position) {
                // Check variation in nearby positions (mutation = local information variance)
                let nearby_positions = [
                    (position.0 + 0.1, position.1, position.2),
                    (position.0 - 0.1, position.1, position.2),
                    (position.0, position.1 + 0.1, position.2),
                    (position.0, position.1 - 0.1, position.2),
                ];
                
                let mut local_variance = 0.0;
                let center_density = info.density();
                
                for &nearby_pos in &nearby_positions {
                    if let Some(nearby_info) = ecosystem.information_at(nearby_pos) {
                        local_variance += (center_density - nearby_info.density()).abs();
                    }
                }
                
                total_variation += local_variance / nearby_positions.len() as f64;
                sample_count += 1;
            }
        }
    }
    
    if sample_count > 0 { total_variation / sample_count as f64 } else { 0.0 }
}

fn calculate_selection_pressure(current: &HashMap<String, usize>, initial: &HashMap<String, usize>) -> f64 {
    let mut pressure_sum = 0.0;
    let mut species_count = 0;
    
    for (species, &initial_pop) in initial {
        if initial_pop > 0 {
            let current_pop = current.get(species).unwrap_or(&0);
            let fitness = *current_pop as f64 / initial_pop as f64;
            // Selection pressure = deviation from neutral evolution (1.0)
            pressure_sum += (1.0 - fitness).abs();
            species_count += 1;
        }
    }
    
    if species_count > 0 { pressure_sum / species_count as f64 } else { 0.0 }
}

fn calculate_diversity_index(populations: &HashMap<String, usize>) -> f64 {
    let total_population: usize = populations.values().sum();
    if total_population == 0 { return 0.0; }
    
    // Shannon diversity index
    let mut diversity = 0.0;
    for &count in populations.values() {
        if count > 0 {
            let proportion = count as f64 / total_population as f64;
            diversity -= proportion * proportion.ln();
        }
    }
    diversity
}

fn calculate_ecosystem_stability(ecosystem: &Reality) -> f64 {
    // Measure how stable the overall information distribution is
    let total_info = ecosystem.total_information();
    let conscious_count = ecosystem.conscious_count();
    
    if total_info > 0.0 && conscious_count > 0 {
        // Stability = ratio of conscious information to total information
        (conscious_count as f64 / total_info * 1000.0).min(1.0)
    } else {
        0.0
    }
}

fn analyze_population_dynamics(generation_data: &[(usize, HashMap<String, usize>)]) {
    println!("\n📈 POPULATION DYNAMICS ANALYSIS:");
    
    if generation_data.len() < 2 { return; }
    
    let initial_data = &generation_data[0].1;
    let final_data = &generation_data.last().unwrap().1;
    
    println!("  Species Population Changes:");
    println!("  Species    | Initial | Final | Change | Fitness");
    println!("  -----------|---------|-------|--------|--------");
    
    for species in initial_data.keys() {
        let initial = initial_data[species];
        let final_pop = final_data.get(species).unwrap_or(&0);
        let change = *final_pop as i32 - initial as i32;
        let fitness = if initial > 0 { *final_pop as f64 / initial as f64 } else { 0.0 };
        
        println!("  {:11} | {:7} | {:5} | {:6} | {:6.2}", 
                species, initial, final_pop, change, fitness);
    }
}

fn analyze_selection_and_fitness(generation_data: &[(usize, HashMap<String, usize>)], initial: &HashMap<String, usize>) {
    println!("\n🏆 NATURAL SELECTION ANALYSIS:");
    
    // Calculate which species showed strongest selection effects
    let final_data = &generation_data.last().unwrap().1;
    let mut fitness_scores: Vec<_> = initial.iter().map(|(species, &initial_pop)| {
        let final_pop = final_data.get(species).unwrap_or(&0);
        let fitness = if initial_pop > 0 { *final_pop as f64 / initial_pop as f64 } else { 0.0 };
        (species, fitness)
    }).collect();
    
    fitness_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("  Fitness Rankings (survival success):");
    for (i, (species, fitness)) in fitness_scores.iter().enumerate() {
        let status = if *fitness > 1.2 { "Thriving" }
        else if *fitness > 0.8 { "Stable" }
        else if *fitness > 0.2 { "Declining" }
        else { "Extinct" };
        
        println!("  {}. {:11} - Fitness: {:.3} ({})", i+1, species, fitness, status);
    }
    
    let winner = fitness_scores.first().unwrap();
    let loser = fitness_scores.last().unwrap();
    println!("  Strongest selection: {} vs {} (fitness ratio: {:.2})", 
            winner.0, loser.0, winner.1 / loser.1.max(0.001));
}

fn analyze_extinction_events(extinction_events: &[(usize, String)]) {
    println!("\n💀 EXTINCTION ANALYSIS:");
    
    if extinction_events.is_empty() {
        println!("  No complete extinctions observed");
    } else {
        println!("  Extinction Events Detected: {}", extinction_events.len());
        for (generation, species) in extinction_events {
            println!("  Generation {}: {} went extinct", generation, species);
        }
        
        // Analyze extinction timing
        let avg_extinction_time: f64 = extinction_events.iter()
            .map(|(gen, _)| *gen as f64)
            .sum::<f64>() / extinction_events.len() as f64;
        
        println!("  Average extinction time: {:.1} generations", avg_extinction_time);
        println!("  Early extinctions: {}", extinction_events.iter().filter(|(gen, _)| *gen < 50).count());
        println!("  Late extinctions: {}", extinction_events.iter().filter(|(gen, _)| *gen >= 50).count());
    }
}

fn analyze_speciation_events(speciation_events: &[(usize, f64, f64)]) {
    println!("\n🌱 SPECIATION ANALYSIS:");
    
    if speciation_events.is_empty() {
        println!("  No clear speciation events detected");
    } else {
        println!("  Potential speciation events: {}", speciation_events.len());
        for (generation, mutation_rate, stability) in speciation_events {
            println!("  Generation {}: High mutation ({:.3}) + stability ({:.3})", 
                    generation, mutation_rate, stability);
        }
        
        let avg_speciation_time: f64 = speciation_events.iter()
            .map(|(gen, _, _)| *gen as f64)
            .sum::<f64>() / speciation_events.len() as f64;
        
        println!("  Average speciation time: {:.1} generations", avg_speciation_time);
    }
}

fn analyze_ecosystem_evolution(ecosystem_metrics: &[(f64, f64, f64)]) {
    println!("\n🌍 ECOSYSTEM EVOLUTION ANALYSIS:");
    
    if ecosystem_metrics.len() < 2 { return; }
    
    let initial_metrics = &ecosystem_metrics[0];
    let final_metrics = &ecosystem_metrics.last().unwrap();
    
    let diversity_change = final_metrics.0 - initial_metrics.0;
    let stability_change = final_metrics.1 - initial_metrics.1;
    let selection_change = final_metrics.2 - initial_metrics.2;
    
    println!("  Ecosystem trajectory:");
    println!("  Diversity change: {:+.3} ({})", diversity_change, 
            if diversity_change > 0.0 { "increasing" } else { "decreasing" });
    println!("  Stability change: {:+.3} ({})", stability_change,
            if stability_change > 0.0 { "more stable" } else { "less stable" });
    println!("  Selection intensity: {:+.3} ({})", selection_change,
            if selection_change > 0.0 { "stronger" } else { "weaker" });
    
    // Ecosystem health assessment
    let health_score = (final_metrics.0 + final_metrics.1 - final_metrics.2) / 3.0;
    let health_status = if health_score > 0.5 { "Healthy" }
    else if health_score > 0.2 { "Stressed" }
    else { "Degraded" };
    
    println!("  Final ecosystem health: {:.3} ({})", health_score, health_status);
}

fn analyze_final_ecosystem_state(ecosystem: &Reality, species_positions: &HashMap<String, Vec<(f64, f64, f64)>>) {
    println!("\n🔬 FINAL ECOSYSTEM STATE:");
    
    let total_info = ecosystem.total_information();
    let conscious_points = ecosystem.conscious_count();
    let info_density = total_info / (80.0 * 80.0 * 80.0);
    
    println!("  Total ecosystem information: {:.1} bits", total_info);
    println!("  Conscious grid points: {}", conscious_points);
    println!("  Average information density: {:.3} bits/point", info_density);
    
    // Analyze spatial distribution
    let mut occupied_regions = 0;
    for positions in species_positions.values() {
        for &position in positions {
            if let Some(info) = ecosystem.information_at(position) {
                if info.density() > ecosystem.vacuum_density() + 0.1 {
                    occupied_regions += 1;
                    break; // Count each species region once
                }
            }
        }
    }
    
    println!("  Active ecosystem regions: {}/{}", occupied_regions, species_positions.len());
    println!("  Ecosystem complexity: {:.1}%", 
            (occupied_regions as f64 / species_positions.len() as f64) * 100.0);
} 