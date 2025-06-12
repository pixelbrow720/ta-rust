//! Arithmetic Functions
//! 
//! Vector arithmetic functions that operate on arrays of values.

use crate::common::{TAError, validate_prices, validate_not_empty};

/// Calculates square root of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (must be >= 0)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing square root values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::sqrt;
/// 
/// let input = vec![0.0, 1.0, 4.0, 9.0, 16.0];
/// let result = sqrt(&input).unwrap();
/// assert_eq!(result[0], 0.0);
/// assert_eq!(result[1], 1.0);
/// assert_eq!(result[2], 2.0);
/// assert_eq!(result[3], 3.0);
/// assert_eq!(result[4], 4.0);
/// ```
pub fn sqrt(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    // Check that all values are non-negative
    for &x in input {
        if x < 0.0 {
            return Err(TAError::invalid_input(
                format!("SQRT input value {} must be non-negative", x)
            ));
        }
    }
    
    let result = input.iter().map(|&x| x.sqrt()).collect();
    Ok(result)
}

/// Calculates absolute value of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing absolute values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::abs;
/// 
/// let input = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
/// let result = abs(&input).unwrap();
/// assert_eq!(result[0], 3.0);
/// assert_eq!(result[1], 1.0);
/// assert_eq!(result[2], 0.0);
/// assert_eq!(result[3], 1.0);
/// assert_eq!(result[4], 3.0);
/// ```
pub fn abs(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.abs()).collect();
    Ok(result)
}

/// Calculates power (x^exponent) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (base)
/// * `exponent` - Exponent value
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing power values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::pow;
/// 
/// let input = vec![1.0, 2.0, 3.0, 4.0];
/// let result = pow(&input, 2.0).unwrap();
/// assert_eq!(result[0], 1.0);
/// assert_eq!(result[1], 4.0);
/// assert_eq!(result[2], 9.0);
/// assert_eq!(result[3], 16.0);
/// ```
pub fn pow(input: &[f64], exponent: f64) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.powf(exponent)).collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_basic() {
        let input = vec![0.0, 1.0, 4.0, 9.0, 16.0];
        let result = sqrt(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[1], 1.0);
        assert_eq!(result[2], 2.0);
        assert_eq!(result[3], 3.0);
        assert_eq!(result[4], 4.0);
    }

    #[test]
    fn test_sqrt_invalid_input() {
        let input = vec![-1.0];
        let result = sqrt(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_abs_basic() {
        let input = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
        let result = abs(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 3.0);
        assert_eq!(result[1], 1.0);
        assert_eq!(result[2], 0.0);
        assert_eq!(result[3], 1.0);
        assert_eq!(result[4], 3.0);
    }

    #[test]
    fn test_pow_basic() {
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let result = pow(&input, 2.0).unwrap();
        
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 4.0);
        assert_eq!(result[2], 9.0);
        assert_eq!(result[3], 16.0);
    }

    #[test]
    fn test_pow_fractional() {
        let input = vec![1.0, 4.0, 9.0, 16.0];
        let result = pow(&input, 0.5).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - 2.0).abs() < 1e-10);
        assert!((result[2] - 3.0).abs() < 1e-10);
        assert!((result[3] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        
        let result = sqrt(&input);
        assert!(result.is_err());
        
        let result = abs(&input);
        assert!(result.is_err());
        
        let result = pow(&input, 2.0);
        assert!(result.is_err());
    }
}