//! Reference accuracy tests against Python TA-Lib
//! 
//! These tests compare TA-Rust output with Python TA-Lib reference implementation
//! to ensure mathematical accuracy and compatibility.

use serde_json::{Value, Map};
use ta_rust::{
    overlap::{sma, ema},
    momentum::{rsi, willr},
    volatility::{atr, trange},
    price_transform::typprice,
};

/// Standard tolerance for accuracy tests
const ACCURACY_TOLERANCE: f64 = 1e-10;

/// Relaxed tolerance for complex calculations
const RELAXED_TOLERANCE: f64 = 1e-8;

/// Load reference data from JSON file
fn load_reference_data() -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    // Try multiple possible paths
    let possible_paths = [
        "scripts/reference_data.json",
        "../scripts/reference_data.json", 
        "./scripts/reference_data.json",
        "reference_data.json"
    ];
    
    let mut reference_file = None;
    for path in &possible_paths {
        let p = std::path::Path::new(path);
        if p.exists() {
            reference_file = Some(p);
            break;
        }
    }
    
    let reference_file = reference_file.ok_or("Reference data file not found in any expected location")?;
    
    if !reference_file.exists() {
        return Err("Reference data file not found. Run 'python scripts/accuracy_test.py' first.".into());
    }
    
    let content = std::fs::read_to_string(reference_file)?;
    let data: Value = serde_json::from_str(&content)?;
    
    match data {
        Value::Object(map) => Ok(map),
        _ => Err("Invalid reference data format".into()),
    }
}

/// Extract f64 vector from JSON array, handling NaN values
fn extract_f64_vec(value: &Value) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    match value {
        Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                match item {
                    Value::Number(n) => {
                        if let Some(f) = n.as_f64() {
                            result.push(f);
                        } else {
                            return Err("Invalid number in array".into());
                        }
                    }
                    Value::Null => result.push(f64::NAN),
                    _ => return Err("Invalid array item type".into()),
                }
            }
            Ok(result)
        }
        _ => Err("Expected array".into()),
    }
}

/// Compare two f64 vectors with tolerance for NaN handling
fn compare_vectors(actual: &[f64], expected: &[f64], tolerance: f64, test_name: &str) -> bool {
    if actual.len() != expected.len() {
        eprintln!("{}: Length mismatch - actual: {}, expected: {}", 
                 test_name, actual.len(), expected.len());
        return false;
    }
    
    let mut max_error: f64 = 0.0;
    let mut error_count = 0;
    let mut total_compared = 0;
    
    for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
        // Handle NaN cases
        if e.is_nan() && a.is_nan() {
            continue; // Both NaN is OK
        }
        
        if e.is_nan() || a.is_nan() {
            eprintln!("{}: NaN mismatch at index {} - actual: {}, expected: {}", 
                     test_name, i, a, e);
            error_count += 1;
            continue;
        }
        
        let error = (a - e).abs();
        max_error = max_error.max(error);
        total_compared += 1;
        
        if error > tolerance {
            eprintln!("{}: Error at index {} - actual: {}, expected: {}, error: {}", 
                     test_name, i, a, e, error);
            error_count += 1;
        }
    }
    
    let success = error_count == 0;
    
    if success {
        println!("{}: PASS (max_error: {:.2e}, compared: {})", 
                test_name, max_error, total_compared);
    } else {
        println!("{}: FAIL ({} errors out of {} comparisons, max_error: {:.2e})", 
                test_name, error_count, total_compared, max_error);
    }
    
    success
}

/// Test SMA accuracy against reference
fn test_sma_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_passed = true;
    
    for (key, test_data) in reference_data {
        if key.starts_with("sma_") {
            let input = extract_f64_vec(&test_data["input"])?;
            let period = test_data["period"].as_u64().unwrap() as usize;
            let expected = extract_f64_vec(&test_data["expected"])?;
            
            let actual = sma(&input, period)?;
            
            let passed = compare_vectors(&actual, &expected, ACCURACY_TOLERANCE, key);
            all_passed &= passed;
        }
    }
    
    Ok(all_passed)
}

/// Test EMA accuracy against reference
fn test_ema_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_passed = true;
    
    for (key, test_data) in reference_data {
        if key.starts_with("ema_") {
            let input = extract_f64_vec(&test_data["input"])?;
            let period = test_data["period"].as_u64().unwrap() as usize;
            let expected = extract_f64_vec(&test_data["expected"])?;
            
            let actual = ema(&input, period)?;
            
            let passed = compare_vectors(&actual, &expected, RELAXED_TOLERANCE, key);
            all_passed &= passed;
        }
    }
    
    Ok(all_passed)
}

/// Test RSI accuracy against reference
fn test_rsi_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_passed = true;
    
    for (key, test_data) in reference_data {
        if key.starts_with("rsi_") {
            let input = extract_f64_vec(&test_data["input"])?;
            let period = test_data["period"].as_u64().unwrap() as usize;
            let expected = extract_f64_vec(&test_data["expected"])?;
            
            let actual = rsi(&input, period)?;
            
            let passed = compare_vectors(&actual, &expected, RELAXED_TOLERANCE, key);
            all_passed &= passed;
        }
    }
    
    Ok(all_passed)
}

/// Test ATR accuracy against reference
fn test_atr_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_passed = true;
    
    for (key, test_data) in reference_data {
        if key.starts_with("atr_") {
            let high = extract_f64_vec(&test_data["high"])?;
            let low = extract_f64_vec(&test_data["low"])?;
            let close = extract_f64_vec(&test_data["close"])?;
            let period = test_data["period"].as_u64().unwrap() as usize;
            let expected = extract_f64_vec(&test_data["expected"])?;
            
            let actual = atr(&high, &low, &close, period)?;
            
            let passed = compare_vectors(&actual, &expected, RELAXED_TOLERANCE, key);
            all_passed &= passed;
        }
    }
    
    Ok(all_passed)
}

/// Test Williams %R accuracy against reference
fn test_willr_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_passed = true;
    
    for (key, test_data) in reference_data {
        if key.starts_with("willr_") {
            let high = extract_f64_vec(&test_data["high"])?;
            let low = extract_f64_vec(&test_data["low"])?;
            let close = extract_f64_vec(&test_data["close"])?;
            let period = test_data["period"].as_u64().unwrap() as usize;
            let expected = extract_f64_vec(&test_data["expected"])?;
            
            let actual = willr(&high, &low, &close, period)?;
            
            let passed = compare_vectors(&actual, &expected, RELAXED_TOLERANCE, key);
            all_passed &= passed;
        }
    }
    
    Ok(all_passed)
}

/// Test Typical Price accuracy against reference
fn test_typprice_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(test_data) = reference_data.get("typprice") {
        let high = extract_f64_vec(&test_data["high"])?;
        let low = extract_f64_vec(&test_data["low"])?;
        let close = extract_f64_vec(&test_data["close"])?;
        let expected = extract_f64_vec(&test_data["expected"])?;
        
        let actual = typprice(&high, &low, &close)?;
        
        let passed = compare_vectors(&actual, &expected, ACCURACY_TOLERANCE, "typprice");
        return Ok(passed);
    }
    
    Ok(true) // No test data found
}

/// Test True Range accuracy against reference
fn test_trange_accuracy(reference_data: &Map<String, Value>) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(test_data) = reference_data.get("trange") {
        let high = extract_f64_vec(&test_data["high"])?;
        let low = extract_f64_vec(&test_data["low"])?;
        let close = extract_f64_vec(&test_data["close"])?;
        let expected = extract_f64_vec(&test_data["expected"])?;
        
        let actual = trange(&high, &low, &close)?;
        
        let passed = compare_vectors(&actual, &expected, ACCURACY_TOLERANCE, "trange");
        return Ok(passed);
    }
    
    Ok(true) // No test data found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test reference_accuracy -- --ignored
    fn test_all_reference_accuracy() {
        println!("\n=== TA-Rust Reference Accuracy Tests ===");
        
        let reference_data = match load_reference_data() {
            Ok(data) => data,
            Err(e) => {
                println!("Failed to load reference data: {}", e);
                println!("Run 'python scripts/accuracy_test.py' to generate reference data first.");
                panic!("Reference data not available");
            }
        };
        
        println!("Loaded {} test cases from reference data", reference_data.len());
        
        let mut all_tests_passed = true;
        
        // Run all accuracy tests
        println!("\n--- Testing SMA ---");
        all_tests_passed &= test_sma_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n--- Testing EMA ---");
        all_tests_passed &= test_ema_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n--- Testing RSI ---");
        all_tests_passed &= test_rsi_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n--- Testing ATR ---");
        all_tests_passed &= test_atr_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n--- Testing Williams %R ---");
        all_tests_passed &= test_willr_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n--- Testing Typical Price ---");
        all_tests_passed &= test_typprice_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n--- Testing True Range ---");
        all_tests_passed &= test_trange_accuracy(&reference_data).unwrap_or(false);
        
        println!("\n=== Summary ===");
        if all_tests_passed {
            println!("✅ ALL ACCURACY TESTS PASSED!");
            println!("TA-Rust output matches Python TA-Lib reference implementation.");
        } else {
            println!("❌ SOME ACCURACY TESTS FAILED!");
            println!("Check the detailed output above for specific failures.");
        }
        
        assert!(all_tests_passed, "Reference accuracy tests failed");
    }
    
    #[test]
    fn test_reference_data_loading() {
        // This test just checks if we can load the reference data structure
        // It won't fail if the file doesn't exist, just skip
        if let Ok(data) = load_reference_data() {
            assert!(!data.is_empty(), "Reference data should not be empty");
            println!("Reference data loaded successfully with {} test cases", data.len());
        } else {
            println!("Reference data not available - run 'python scripts/accuracy_test.py' to generate");
        }
    }
}