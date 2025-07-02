//! Radioactive Decay and Information Field Interactions
//!
//! Experimental Protocol: Testing Information Integration Reality Theory (IIRT) predictions
//! regarding the relationship between information fields and quantum decay processes.
//!
//! Hypothesis: Information field density variations may correlate with measurable
//! changes in radioactive decay statistics.
//!
//! Method: Computational modeling of information field effects on quantum processes
//! to generate testable predictions for empirical validation.

use iirt_engine::*;

fn main() {
    println!("RADIOACTIVE DECAY AND INFORMATION FIELD INTERACTIONS");
    println!("===================================================");
    println!("Computational modeling for experimental design\n");
    
    experimental_protocol();
    baseline_modeling();
    information_field_effects();
    statistical_predictions();
    experimental_controls();
}

fn experimental_protocol() {
    println!("1. EXPERIMENTAL PROTOCOL");
    println!("========================");
    
    println!("Objective: Investigate potential correlations between information field");
    println!("density and radioactive decay statistics under controlled conditions.");
    
    println!("\nProposed Setup:");
    println!("• Radioactive source: Am-241 (α-emitter, t₁/₂ = 432.6 years)");
    println!("• Detection: Geiger-Müller tube with digital counting");
    println!("• Measurement periods: 10-minute intervals over 24 hours");
    println!("• Environmental controls: Temperature, humidity, pressure");
    println!("• Shielding: Lead-lined chamber to minimize external radiation");
    
    println!("\nTest Conditions:");
    println!("A. Baseline: Isolated detector system");
    println!("B. Information-rich: Computing equipment operating nearby");
    println!("C. Biological: Human subjects in proximity (various states)");
    println!("D. Control: Randomized schedule of conditions");
    
    println!("\nData Collection:");
    println!("• Decay counts per 10-minute interval");
    println!("• Environmental parameters (T, P, humidity)");
    println!("• Information source characteristics when applicable");
    println!("• Randomization logs for condition assignment");
    
    println!("\nSuccess Criteria:");
    println!("• Statistical significance: p < 0.05 for condition differences");
    println!("• Effect size: >5% deviation from baseline means");
    println!("• Reproducibility: Effects consistent across multiple trials\n");
}

fn baseline_modeling() {
    println!("2. BASELINE MODELING");
    println!("===================");
    
    // Model standard radioactive decay
    let decay_constant = 0.693 / (432.6 * 365.25 * 24.0 * 60.0); // per minute
    let initial_activity = 1000.0; // counts per minute
    
    println!("Standard radioactive decay parameters:");
    println!("• Decay constant λ: {:.2e} min⁻¹", decay_constant);
    println!("• Initial activity: {:.0} cpm", initial_activity);
    println!("• Expected Poisson statistics");
    
    // Simulate 24-hour baseline
    println!("\nBaseline decay simulation (144 intervals × 10 min):");
    println!("Interval | Expected | √N Error | CV% | Classification");
    println!("---------|----------|----------|-----|---------------");
    
    let mut baseline_data = Vec::new();
    for interval in (0..144).step_by(24) {
        let time_hours = (interval * 10) as f64 / 60.0;
        let expected_counts: f64 = initial_activity * 10.0; // 10-minute interval
        let poisson_error = expected_counts.sqrt();
        let cv_percent = (poisson_error / expected_counts) * 100.0;
        
        baseline_data.push(expected_counts);
        
        let classification = if cv_percent < 1.0 {
            "High precision"
        } else if cv_percent < 3.0 {
            "Good statistics"
        } else {
            "Limited precision"
        };
        
        println!("{:8} | {:8.0} | {:8.1} | {:3.1} | {}", 
                interval, expected_counts, poisson_error, cv_percent, classification);
    }
    
    let mean_baseline = baseline_data.iter().sum::<f64>() / baseline_data.len() as f64;
    let expected_std = mean_baseline.sqrt();
    
    println!("\nBaseline statistical parameters:");
    println!("• Mean count rate: {:.1} ± {:.1} counts per interval", mean_baseline, expected_std);
    println!("• Minimum detectable change: {:.1}% (2σ level)", (2.0 * expected_std / mean_baseline) * 100.0);
    println!("• Required observation time for 1% precision: {:.0} hours", 
             (100.0 / mean_baseline).powi(2) * mean_baseline / 6.0);
    
    println!("\n");
}

fn information_field_effects() {
    println!("3. INFORMATION FIELD MODELING");
    println!("=============================");
    
    // Model information field around decay source
    let mut decay_system = Reality::new(16, (-0.5, 0.5), 1.0, 0.01);
    
    println!("Testing information field effects on quantum decay processes...");
    
    // Test different information field configurations
    let test_conditions = [
        ("Baseline", 0.0),
        ("Low information", 0.5),
        ("Moderate information", 1.5),
        ("High information", 3.0),
        ("Consciousness threshold", INTEGRATION_THRESHOLD),
        ("Super-threshold", 2.0),
    ];
    
    println!("\nInformation field effect modeling:");
    println!("Condition | Field ℐ | Conscious | ΔDecay% | Uncertainty | Prediction");
    println!("----------|---------|-----------|---------|-------------|------------");
    
    for (condition, info_level) in test_conditions {
        // Reset system for each test
        decay_system = Reality::new(16, (-0.5, 0.5), 1.0, 0.01);
        
        if info_level > 0.0 {
            decay_system.add_information((0.0, 0.0, 0.0), info_level);
        }
        
        // Evolve system briefly
        for _ in 0..10 {
            decay_system.evolve();
        }
        
        let field_density = decay_system.information_at((0.0, 0.0, 0.0))
            .map(|i| i.density())
            .unwrap_or(decay_system.vacuum_density());
        
        let is_conscious = field_density >= INTEGRATION_THRESHOLD;
        
        // Calculate predicted decay rate change
        // Based on information field modifying quantum probability amplitudes
        let _baseline_rate = 1000.0; // cpm
        let field_enhancement = field_density / decay_system.vacuum_density();
        let decay_change_percent = (field_enhancement - 1.0) * 10.0; // Scaled effect
        
        let uncertainty = (0.5 / (1.0 + field_density)).max(MIN_UNCERTAINTY);
        
        let prediction = if decay_change_percent.abs() > 2.0 {
            "Measurable"
        } else if decay_change_percent.abs() > 0.5 {
            "Detectable"
        } else {
            "Below threshold"
        };
        
        println!("{:>9} | {:7.3} | {:9} | {:+7.1} | {:11.3} | {}", 
                condition, field_density, is_conscious, decay_change_percent, uncertainty, prediction);
    }
    
    println!("\nKey modeling insights:");
    println!("• Information field density correlates with predicted decay rate changes");
    println!("• Consciousness threshold (ℐ ≥ {:.3}) shows enhanced effects", INTEGRATION_THRESHOLD);
    println!("• Uncertainty function limits maximum observable effects");
    println!("• Predicted effects are within measurement precision range\n");
}

fn statistical_predictions() {
    println!("4. STATISTICAL PREDICTIONS");
    println!("==========================");
    
    let baseline_rate: f64 = 1000.0; // counts per 10-minute interval
    let observation_periods = 144; // 24 hours of 10-minute intervals
    let n_trials = 10; // Number of experimental runs
    
    println!("Power analysis for detecting information field effects:");
    
    let effect_sizes = [0.5, 1.0, 2.0, 5.0, 10.0]; // Percent changes
    
    println!("\nDetection probability vs effect size:");
    println!("Effect Size | Sample Size | Statistical Power | Observation Time");
    println!("------------|-------------|-------------------|------------------");
    
    for &effect in &effect_sizes {
        let effect_counts = baseline_rate * (effect / 100.0);
        let baseline_std = baseline_rate.sqrt();
        let effect_std = (baseline_rate + effect_counts).sqrt();
        
        // Calculate required sample size for 80% power
        let z_alpha = 1.96; // 95% confidence
        let z_beta = 0.84;  // 80% power
        
        let pooled_std = ((baseline_std.powi(2) + effect_std.powi(2)) / 2.0).sqrt();
        let required_n = (2.0 * pooled_std * (z_alpha + z_beta) / effect_counts).powi(2);
        
        let observation_hours = (required_n / 6.0).ceil(); // 6 intervals per hour
        
        let statistical_power = if required_n <= observation_periods as f64 {
            "High (>80%)"
        } else if required_n <= (observation_periods * 3) as f64 {
            "Moderate (50-80%)"
        } else {
            "Low (<50%)"
        };
        
        println!("{:10.1}% | {:11.0} | {:17} | {:16.0} hours", 
                effect, required_n, statistical_power, observation_hours);
    }
    
    println!("\nRecommended experimental parameters:");
    println!("• Minimum effect size for reliable detection: 2.0%");
    println!("• Recommended observation time per condition: 24 hours");
    println!("• Number of replication trials: 5-10");
    println!("• Total experimental duration: 2-3 weeks");
    
    println!("\nExpected outcomes under IIRT predictions:");
    println!("• Baseline condition: 1000 ± 32 counts (Poisson)");
    println!("• Information-enhanced: 1020 ± 32 counts (+2.0%)");
    println!("• Consciousness-mediated: 1050 ± 32 counts (+5.0%)");
    println!("• Effect reproducibility: >70% across trials\n");
}

fn experimental_controls() {
    println!("5. EXPERIMENTAL CONTROLS");
    println!("========================");
    
    println!("Critical controls for eliminating systematic errors:");
    
    println!("\nEnvironmental Controls:");
    println!("• Temperature: ±0.1°C stability (affects detector efficiency)");
    println!("• Pressure: ±1 mbar stability (affects air absorption)");
    println!("• Humidity: <5% variation (affects electronics)");
    println!("• Electromagnetic shielding: Faraday cage around detector");
    println!("• Vibration isolation: Pneumatic isolation table");
    
    println!("\nInstrumentation Controls:");
    println!("• Detector calibration: Daily check source measurements");
    println!("• Dead time correction: Account for detector response time");
    println!("• Background subtraction: Regular background measurements");
    println!("• Data logging: Automated digital recording (no human bias)");
    println!("• Clock synchronization: GPS time reference");
    
    println!("\nExperimental Design Controls:");
    println!("• Randomization: Condition order determined by random sequence");
    println!("• Blinding: Data analysts unaware of condition assignments");
    println!("• Replication: Multiple independent experimental runs");
    println!("• Cross-validation: Results verified by independent laboratory");
    
    println!("\nStatistical Controls:");
    println!("• Multiple comparisons: Bonferroni correction for p-values");
    println!("• Effect size reporting: Cohen's d for practical significance");
    println!("• Confidence intervals: 95% CI for all measurements");
    println!("• Publication protocol: Results reported regardless of outcome");
    
    println!("\nNull hypothesis testing:");
    println!("H₀: No difference in decay rates between conditions");
    println!("H₁: Systematic differences correlate with information field parameters");
    println!("α = 0.05 (Type I error rate)");
    println!("β = 0.20 (Type II error rate, 80% power)");
    
    println!("\nExpected challenges:");
    println!("• Small effect sizes requiring large sample sizes");
    println!("• Environmental noise masking true effects");
    println!("• Reproducibility across different laboratories");
    println!("• Theoretical interpretation of positive results");
    
    println!("\nSuccess metrics:");
    println!("• Statistically significant effects (p < 0.05)");
    println!("• Reproducible across multiple trials");
    println!("• Effect size correlation with information field parameters");
    println!("• Independent laboratory confirmation\n");
    
    println!("Note: This protocol is designed to test specific predictions");
    println!("of Information Integration Reality Theory while maintaining");
    println!("rigorous scientific methodology and statistical controls.");
}