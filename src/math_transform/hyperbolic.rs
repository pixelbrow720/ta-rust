//! Hyperbolic Functions
//! 
//! Vector hyperbolic functions that operate on arrays of values.

use crate::common::{TAError, validate_prices};

/// Calculates hyperbolic sine of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing hyperbolic sine values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::sinh;
/// 
/// let input = vec![0.0, 1.0, -1.0];
/// let result = sinh(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// ```
pub fn sinh(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.sinh()).collect();
    Ok(result)
}

/// Calculates hyperbolic cosine of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing hyperbolic cosine values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::cosh;
/// 
/// let input = vec![0.0, 1.0, -1.0];
/// let result = cosh(&input).unwrap();
/// assert!((result[0] - 1.0).abs() < 1e-10);
/// ```
pub fn cosh(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.cosh()).collect();
    Ok(result)
}

/// Calculates hyperbolic tangent of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing hyperbolic tangent values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::tanh;
/// 
/// let input = vec![0.0, 1.0, -1.0];
/// let result = tanh(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// ```
pub fn tanh(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.tanh()).collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinh_basic() {
        let input = vec![0.0, 1.0, -1.0];
        let result = sinh(&input).unwrap();
        
        assert_eq!(result.len(), 3);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0_f64.sinh()).abs() < 1e-10);
        assert!((result[2] - (-1.0_f64.sinh())).abs() < 1e-10);
    }

    #[test]
    fn test_cosh_basic() {
        let input = vec![0.0, 1.0, -1.0];
        let result = cosh(&input).unwrap();
        
        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 1.0_f64.cosh()).abs() < 1e-10);
        assert!((result[2] - 1.0_f64.cosh()).abs() < 1e-10); // cosh(-x) = cosh(x)
    }

    #[test]
    fn test_tanh_basic() {
        let input = vec![0.0, 1.0, -1.0];
        let result = tanh(&input).unwrap();
        
        assert_eq!(result.len(), 3);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 1.0_f64.tanh()).abs() < 1e-10);
        assert!((result[2] - (-1.0_f64.tanh())).abs() < 1e-10);
    }
}