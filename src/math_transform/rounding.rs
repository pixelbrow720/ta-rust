//! Rounding Functions
//! 
//! Vector rounding functions that operate on arrays of values.

use crate::common::{TAError, validate_prices, validate_not_empty};

/// Calculates ceiling (smallest integer >= input) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ceiling values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::ceil;
/// 
/// let input = vec![1.1, 2.7, -1.3, 0.0];
/// let result = ceil(&input).unwrap();
/// assert_eq!(result[0], 2.0);
/// assert_eq!(result[1], 3.0);
/// assert_eq!(result[2], -1.0);
/// assert_eq!(result[3], 0.0);
/// ```
pub fn ceil(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.ceil()).collect();
    Ok(result)
}

/// Calculates floor (largest integer <= input) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing floor values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::floor;
/// 
/// let input = vec![1.1, 2.7, -1.3, 0.0];
/// let result = floor(&input).unwrap();
/// assert_eq!(result[0], 1.0);
/// assert_eq!(result[1], 2.0);
/// assert_eq!(result[2], -2.0);
/// assert_eq!(result[3], 0.0);
/// ```
pub fn floor(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.floor()).collect();
    Ok(result)
}

/// Calculates round (nearest integer) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing rounded values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::round;
/// 
/// let input = vec![1.1, 2.7, -1.3, 0.5];
/// let result = round(&input).unwrap();
/// assert_eq!(result[0], 1.0);
/// assert_eq!(result[1], 3.0);
/// assert_eq!(result[2], -1.0);
/// assert_eq!(result[3], 1.0);
/// ```
pub fn round(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.round()).collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceil_basic() {
        let input = vec![1.1, 2.7, -1.3, 0.0, -0.1];
        let result = ceil(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 2.0);
        assert_eq!(result[1], 3.0);
        assert_eq!(result[2], -1.0);
        assert_eq!(result[3], 0.0);
        assert_eq!(result[4], 0.0);
    }

    #[test]
    fn test_floor_basic() {
        let input = vec![1.1, 2.7, -1.3, 0.0, -0.1];
        let result = floor(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 2.0);
        assert_eq!(result[2], -2.0);
        assert_eq!(result[3], 0.0);
        assert_eq!(result[4], -1.0);
    }

    #[test]
    fn test_round_basic() {
        let input = vec![1.1, 2.7, -1.3, 0.5, -0.5];
        let result = round(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1.0);
        assert_eq!(result[1], 3.0);
        assert_eq!(result[2], -1.0);
        assert_eq!(result[3], 1.0);
        assert_eq!(result[4], -1.0);
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        
        let result = ceil(&input);
        assert!(result.is_err());
        
        let result = floor(&input);
        assert!(result.is_err());
        
        let result = round(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_large_values() {
        let input = vec![1e10, -1e10, 1e-10, -1e-10];
        
        let ceil_result = ceil(&input).unwrap();
        let floor_result = floor(&input).unwrap();
        let round_result = round(&input).unwrap();
        
        // Should handle large values gracefully
        for &val in &ceil_result {
            assert!(val.is_finite());
        }
        
        for &val in &floor_result {
            assert!(val.is_finite());
        }
        
        for &val in &round_result {
            assert!(val.is_finite());
        }
    }
}