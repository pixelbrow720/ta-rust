//! Logarithmic Functions
//! 
//! Vector logarithmic and exponential functions that operate on arrays of values.

use crate::common::{TAError, validate_prices, validate_not_empty};

/// Calculates natural logarithm of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (must be > 0)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing natural logarithm values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::ln;
/// 
/// let input = vec![1.0, std::f64::consts::E, std::f64::consts::E.powi(2)];
/// let result = ln(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - 1.0).abs() < 1e-10);
/// assert!((result[2] - 2.0).abs() < 1e-10);
/// ```
pub fn ln(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    // Check that all values are positive
    for &x in input {
        if x <= 0.0 {
            return Err(TAError::invalid_input(
                format!("LN input value {} must be positive", x)
            ));
        }
    }
    
    let result = input.iter().map(|&x| x.ln()).collect();
    Ok(result)
}

/// Calculates base-10 logarithm of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (must be > 0)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing base-10 logarithm values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::log10;
/// 
/// let input = vec![1.0, 10.0, 100.0, 1000.0];
/// let result = log10(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - 1.0).abs() < 1e-10);
/// assert!((result[2] - 2.0).abs() < 1e-10);
/// assert!((result[3] - 3.0).abs() < 1e-10);
/// ```
pub fn log10(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    // Check that all values are positive
    for &x in input {
        if x <= 0.0 {
            return Err(TAError::invalid_input(
                format!("LOG10 input value {} must be positive", x)
            ));
        }
    }
    
    let result = input.iter().map(|&x| x.log10()).collect();
    Ok(result)
}

/// Calculates exponential (e^x) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (any real number)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing exponential values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::exp;
/// 
/// let input = vec![0.0, 1.0, 2.0];
/// let result = exp(&input).unwrap();
/// assert!((result[0] - 1.0).abs() < 1e-10);
/// assert!((result[1] - std::f64::consts::E).abs() < 1e-10);
/// assert!((result[2] - std::f64::consts::E.powi(2)).abs() < 1e-10);
/// ```
pub fn exp(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.exp()).collect();
    Ok(result)
}

/// Calculates logarithm with custom base of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (must be > 0)
/// * `base` - Logarithm base (must be > 0 and != 1)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing logarithm values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::log_base;
/// 
/// let input = vec![1.0, 2.0, 4.0, 8.0];
/// let result = log_base(&input, 2.0).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - 1.0).abs() < 1e-10);
/// assert!((result[2] - 2.0).abs() < 1e-10);
/// assert!((result[3] - 3.0).abs() < 1e-10);
/// ```
pub fn log_base(input: &[f64], base: f64) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    if base <= 0.0 || base == 1.0 {
        return Err(TAError::invalid_input(
            format!("Logarithm base {} must be positive and not equal to 1", base)
        ));
    }
    
    // Check that all values are positive
    for &x in input {
        if x <= 0.0 {
            return Err(TAError::invalid_input(
                format!("LOG_BASE input value {} must be positive", x)
            ));
        }
    }
    
    let log_base = base.ln();
    let result = input.iter().map(|&x| x.ln() / log_base).collect();
    Ok(result)
}

/// Calculates power (base^x) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (exponents)
/// * `base` - Base value (must be > 0)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing power values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::pow_base;
/// 
/// let input = vec![0.0, 1.0, 2.0, 3.0];
/// let result = pow_base(&input, 2.0).unwrap();
/// assert!((result[0] - 1.0).abs() < 1e-10);
/// assert!((result[1] - 2.0).abs() < 1e-10);
/// assert!((result[2] - 4.0).abs() < 1e-10);
/// assert!((result[3] - 8.0).abs() < 1e-10);
/// ```
pub fn pow_base(input: &[f64], base: f64) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    if base <= 0.0 {
        return Err(TAError::invalid_input(
            format!("Power base {} must be positive", base)
        ));
    }
    
    let result = input.iter().map(|&x| base.powf(x)).collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{E, LN_10};

    #[test]
    fn test_ln_basic() {
        let input = vec![1.0, E, E.powi(2), E.powi(3)];
        let result = ln(&input).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);
        assert!((result[2] - 2.0).abs() < 1e-10);
        assert!((result[3] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_invalid_input() {
        let input = vec![0.0];
        let result = ln(&input);
        assert!(result.is_err());
        
        let input = vec![-1.0];
        let result = ln(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_log10_basic() {
        let input = vec![1.0, 10.0, 100.0, 1000.0];
        let result = log10(&input).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);
        assert!((result[2] - 2.0).abs() < 1e-10);
        assert!((result[3] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_log10_invalid_input() {
        let input = vec![0.0];
        let result = log10(&input);
        assert!(result.is_err());
        
        let input = vec![-1.0];
        let result = log10(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_exp_basic() {
        let input = vec![0.0, 1.0, 2.0, -1.0];
        let result = exp(&input).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - E).abs() < 1e-10);
        assert!((result[2] - E.powi(2)).abs() < 1e-10);
        assert!((result[3] - (1.0 / E)).abs() < 1e-10);
    }

    #[test]
    fn test_log_base_basic() {
        let input = vec![1.0, 2.0, 4.0, 8.0];
        let result = log_base(&input, 2.0).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0).abs() < 1e-10);
        assert!((result[2] - 2.0).abs() < 1e-10);
        assert!((result[3] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_log_base_invalid_base() {
        let input = vec![1.0, 2.0];
        
        let result = log_base(&input, 0.0);
        assert!(result.is_err());
        
        let result = log_base(&input, 1.0);
        assert!(result.is_err());
        
        let result = log_base(&input, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_pow_base_basic() {
        let input = vec![0.0, 1.0, 2.0, 3.0];
        let result = pow_base(&input, 2.0).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 2.0).abs() < 1e-10);
        assert!((result[2] - 4.0).abs() < 1e-10);
        assert!((result[3] - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_pow_base_invalid_base() {
        let input = vec![1.0, 2.0];
        
        let result = pow_base(&input, 0.0);
        assert!(result.is_err());
        
        let result = pow_base(&input, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        
        let result = ln(&input);
        assert!(result.is_err());
        
        let result = log10(&input);
        assert!(result.is_err());
        
        let result = exp(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_ln_exp_inverse() {
        // Test that ln(exp(x)) = x
        let input = vec![0.0, 1.0, 2.0, -1.0, 0.5];
        
        let exp_result = exp(&input).unwrap();
        let ln_exp_result = ln(&exp_result).unwrap();
        
        for (i, &original) in input.iter().enumerate() {
            assert!((ln_exp_result[i] - original).abs() < 1e-10);
        }
    }

    #[test]
    fn test_log10_pow10_inverse() {
        // Test that log10(10^x) = x
        let input = vec![0.0, 1.0, 2.0, 0.5];
        
        let pow_result = pow_base(&input, 10.0).unwrap();
        let log_pow_result = log10(&pow_result).unwrap();
        
        for (i, &original) in input.iter().enumerate() {
            assert!((log_pow_result[i] - original).abs() < 1e-10);
        }
    }

    #[test]
    fn test_large_values() {
        let input = vec![100.0, 1000.0];
        
        let ln_result = ln(&input).unwrap();
        let log10_result = log10(&input).unwrap();
        
        // Should handle large values gracefully
        for &val in &ln_result {
            assert!(val.is_finite());
        }
        
        for &val in &log10_result {
            assert!(val.is_finite());
        }
    }

    #[test]
    fn test_small_values() {
        let input = vec![0.001, 0.01, 0.1];
        
        let ln_result = ln(&input).unwrap();
        let log10_result = log10(&input).unwrap();
        
        // Should handle small positive values gracefully
        for &val in &ln_result {
            assert!(val.is_finite());
            assert!(val < 0.0); // ln of values < 1 should be negative
        }
        
        for &val in &log10_result {
            assert!(val.is_finite());
            assert!(val < 0.0); // log10 of values < 1 should be negative
        }
    }

    #[test]
    fn test_relationship_ln_log10() {
        // Test that log10(x) = ln(x) / ln(10)
        let input = vec![1.0, 10.0, 100.0, 2.5];
        
        let ln_result = ln(&input).unwrap();
        let log10_result = log10(&input).unwrap();
        
        for (_i, (&ln_val, &log10_val)) in ln_result.iter().zip(log10_result.iter()).enumerate() {
            let expected_log10 = ln_val / LN_10;
            assert!((log10_val - expected_log10).abs() < 1e-10);
        }
    }
}