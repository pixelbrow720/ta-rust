//! Trigonometric Functions
//! 
//! Vector trigonometric functions that operate on arrays of values.
//! All angle inputs and outputs are in radians.

use crate::common::{TAError, validate_prices, validate_not_empty};

/// Calculates sine of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values in radians
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing sine values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::sin;
/// 
/// let input = vec![0.0, std::f64::consts::PI / 2.0, std::f64::consts::PI];
/// let result = sin(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - 1.0).abs() < 1e-10);
/// assert!((result[2] - 0.0).abs() < 1e-10);
/// ```
pub fn sin(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.sin()).collect();
    Ok(result)
}

/// Calculates cosine of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values in radians
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing cosine values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::cos;
/// 
/// let input = vec![0.0, std::f64::consts::PI / 2.0, std::f64::consts::PI];
/// let result = cos(&input).unwrap();
/// assert!((result[0] - 1.0).abs() < 1e-10);
/// assert!((result[1] - 0.0).abs() < 1e-10);
/// assert!((result[2] - (-1.0)).abs() < 1e-10);
/// ```
pub fn cos(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.cos()).collect();
    Ok(result)
}

/// Calculates tangent of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values in radians
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing tangent values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::tan;
/// 
/// let input = vec![0.0, std::f64::consts::PI / 4.0];
/// let result = tan(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - 1.0).abs() < 1e-10);
/// ```
pub fn tan(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.tan()).collect();
    Ok(result)
}

/// Calculates arcsine (inverse sine) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (must be in range [-1, 1])
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing arcsine values in radians [-π/2, π/2], 
/// or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::asin;
/// 
/// let input = vec![0.0, 0.5, 1.0];
/// let result = asin(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[2] - std::f64::consts::PI / 2.0).abs() < 1e-10);
/// ```
pub fn asin(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    // Check that all values are in valid range [-1, 1]
    for &x in input {
        if x < -1.0 || x > 1.0 {
            return Err(TAError::invalid_input(
                format!("ASIN input value {} is outside valid range [-1, 1]", x)
            ));
        }
    }
    
    let result = input.iter().map(|&x| x.asin()).collect();
    Ok(result)
}

/// Calculates arccosine (inverse cosine) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (must be in range [-1, 1])
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing arccosine values in radians [0, π], 
/// or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::acos;
/// 
/// let input = vec![1.0, 0.0, -1.0];
/// let result = acos(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - std::f64::consts::PI / 2.0).abs() < 1e-10);
/// assert!((result[2] - std::f64::consts::PI).abs() < 1e-10);
/// ```
pub fn acos(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    // Check that all values are in valid range [-1, 1]
    for &x in input {
        if x < -1.0 || x > 1.0 {
            return Err(TAError::invalid_input(
                format!("ACOS input value {} is outside valid range [-1, 1]", x)
            ));
        }
    }
    
    let result = input.iter().map(|&x| x.acos()).collect();
    Ok(result)
}

/// Calculates arctangent (inverse tangent) of each value in the input array.
/// 
/// # Arguments
/// 
/// * `input` - Input values (any real number)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing arctangent values in radians [-π/2, π/2], 
/// or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::atan;
/// 
/// let input = vec![0.0, 1.0, -1.0];
/// let result = atan(&input).unwrap();
/// assert!((result[0] - 0.0).abs() < 1e-10);
/// assert!((result[1] - std::f64::consts::PI / 4.0).abs() < 1e-10);
/// assert!((result[2] - (-std::f64::consts::PI / 4.0)).abs() < 1e-10);
/// ```
pub fn atan(input: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let result = input.iter().map(|&x| x.atan()).collect();
    Ok(result)
}

/// Calculates sine and cosine simultaneously for better performance.
/// 
/// # Arguments
/// 
/// * `input` - Input values in radians
/// 
/// # Returns
/// 
/// Returns `Ok((Vec<f64>, Vec<f64>))` containing (sine, cosine) values, 
/// or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::sin_cos;
/// 
/// let input = vec![0.0, std::f64::consts::PI / 2.0];
/// let (sin_vals, cos_vals) = sin_cos(&input).unwrap();
/// assert!((sin_vals[0] - 0.0).abs() < 1e-10);
/// assert!((cos_vals[0] - 1.0).abs() < 1e-10);
/// ```
pub fn sin_cos(input: &[f64]) -> Result<(Vec<f64>, Vec<f64>), TAError> {
    validate_not_empty(input, "input")?;
    validate_prices(input, "input")?;
    
    let mut sin_result = Vec::with_capacity(input.len());
    let mut cos_result = Vec::with_capacity(input.len());
    
    for &x in input {
        let (s, c) = x.sin_cos();
        sin_result.push(s);
        cos_result.push(c);
    }
    
    Ok((sin_result, cos_result))
}

/// Converts degrees to radians.
/// 
/// # Arguments
/// 
/// * `degrees` - Input values in degrees
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing values in radians, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::deg_to_rad;
/// 
/// let degrees = vec![0.0, 90.0, 180.0, 360.0];
/// let radians = deg_to_rad(&degrees).unwrap();
/// assert!((radians[0] - 0.0).abs() < 1e-10);
/// assert!((radians[1] - std::f64::consts::PI / 2.0).abs() < 1e-10);
/// assert!((radians[2] - std::f64::consts::PI).abs() < 1e-10);
/// assert!((radians[3] - 2.0 * std::f64::consts::PI).abs() < 1e-10);
/// ```
pub fn deg_to_rad(degrees: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(degrees, "degrees")?;
    validate_prices(degrees, "degrees")?;
    
    let result = degrees.iter().map(|&x| x.to_radians()).collect();
    Ok(result)
}

/// Converts radians to degrees.
/// 
/// # Arguments
/// 
/// * `radians` - Input values in radians
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing values in degrees, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::math_transform::rad_to_deg;
/// 
/// let radians = vec![0.0, std::f64::consts::PI / 2.0, std::f64::consts::PI];
/// let degrees = rad_to_deg(&radians).unwrap();
/// assert!((degrees[0] - 0.0).abs() < 1e-10);
/// assert!((degrees[1] - 90.0).abs() < 1e-10);
/// assert!((degrees[2] - 180.0).abs() < 1e-10);
/// ```
pub fn rad_to_deg(radians: &[f64]) -> Result<Vec<f64>, TAError> {
    validate_not_empty(radians, "radians")?;
    validate_prices(radians, "radians")?;
    
    let result = radians.iter().map(|&x| x.to_degrees()).collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_sin_basic() {
        let input = vec![0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI];
        let result = sin(&input).unwrap();
        
        assert_eq!(result.len(), 6);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - 0.5).abs() < 1e-10);
        assert!((result[2] - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((result[3] - (3.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((result[4] - 1.0).abs() < 1e-10);
        assert!((result[5] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos_basic() {
        let input = vec![0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI];
        let result = cos(&input).unwrap();
        
        assert_eq!(result.len(), 6);
        assert!((result[0] - 1.0).abs() < 1e-10);
        assert!((result[1] - (3.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((result[2] - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((result[3] - 0.5).abs() < 1e-10);
        assert!((result[4] - 0.0).abs() < 1e-10);
        assert!((result[5] - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_tan_basic() {
        let input = vec![0.0, PI / 6.0, PI / 4.0, PI / 3.0];
        let result = tan(&input).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - (1.0 / 3.0_f64.sqrt())).abs() < 1e-10);
        assert!((result[2] - 1.0).abs() < 1e-10);
        assert!((result[3] - 3.0_f64.sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_asin_basic() {
        let input = vec![0.0, 0.5, 1.0, -0.5, -1.0];
        let result = asin(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - (PI / 6.0)).abs() < 1e-10);
        assert!((result[2] - (PI / 2.0)).abs() < 1e-10);
        assert!((result[3] - (-PI / 6.0)).abs() < 1e-10);
        assert!((result[4] - (-PI / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_asin_invalid_range() {
        let input = vec![1.5];
        let result = asin(&input);
        assert!(result.is_err());
        
        let input = vec![-1.5];
        let result = asin(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_acos_basic() {
        let input = vec![1.0, 0.5, 0.0, -0.5, -1.0];
        let result = acos(&input).unwrap();
        
        assert_eq!(result.len(), 5);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - (PI / 3.0)).abs() < 1e-10);
        assert!((result[2] - (PI / 2.0)).abs() < 1e-10);
        assert!((result[3] - (2.0 * PI / 3.0)).abs() < 1e-10);
        assert!((result[4] - PI).abs() < 1e-10);
    }

    #[test]
    fn test_acos_invalid_range() {
        let input = vec![1.5];
        let result = acos(&input);
        assert!(result.is_err());
        
        let input = vec![-1.5];
        let result = acos(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_atan_basic() {
        let input = vec![0.0, 1.0, -1.0, 3.0_f64.sqrt()];
        let result = atan(&input).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 0.0).abs() < 1e-10);
        assert!((result[1] - (PI / 4.0)).abs() < 1e-10);
        assert!((result[2] - (-PI / 4.0)).abs() < 1e-10);
        assert!((result[3] - (PI / 3.0)).abs() < 1e-10);
    }

    #[test]
    fn test_sin_cos_combined() {
        let input = vec![0.0, PI / 4.0, PI / 2.0];
        let (sin_vals, cos_vals) = sin_cos(&input).unwrap();
        
        assert_eq!(sin_vals.len(), 3);
        assert_eq!(cos_vals.len(), 3);
        
        // Check sin values
        assert!((sin_vals[0] - 0.0).abs() < 1e-10);
        assert!((sin_vals[1] - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((sin_vals[2] - 1.0).abs() < 1e-10);
        
        // Check cos values
        assert!((cos_vals[0] - 1.0).abs() < 1e-10);
        assert!((cos_vals[1] - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((cos_vals[2] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_deg_to_rad() {
        let degrees = vec![0.0, 30.0, 45.0, 60.0, 90.0, 180.0, 360.0];
        let radians = deg_to_rad(&degrees).unwrap();
        
        assert_eq!(radians.len(), 7);
        assert!((radians[0] - 0.0).abs() < 1e-10);
        assert!((radians[1] - (PI / 6.0)).abs() < 1e-10);
        assert!((radians[2] - (PI / 4.0)).abs() < 1e-10);
        assert!((radians[3] - (PI / 3.0)).abs() < 1e-10);
        assert!((radians[4] - (PI / 2.0)).abs() < 1e-10);
        assert!((radians[5] - PI).abs() < 1e-10);
        assert!((radians[6] - (2.0 * PI)).abs() < 1e-10);
    }

    #[test]
    fn test_rad_to_deg() {
        let radians = vec![0.0, PI / 6.0, PI / 4.0, PI / 3.0, PI / 2.0, PI, 2.0 * PI];
        let degrees = rad_to_deg(&radians).unwrap();
        
        assert_eq!(degrees.len(), 7);
        assert!((degrees[0] - 0.0).abs() < 1e-10);
        assert!((degrees[1] - 30.0).abs() < 1e-10);
        assert!((degrees[2] - 45.0).abs() < 1e-10);
        assert!((degrees[3] - 60.0).abs() < 1e-10);
        assert!((degrees[4] - 90.0).abs() < 1e-10);
        assert!((degrees[5] - 180.0).abs() < 1e-10);
        assert!((degrees[6] - 360.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        
        let result = sin(&input);
        assert!(result.is_err());
        
        let result = cos(&input);
        assert!(result.is_err());
        
        let result = tan(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_values() {
        let input = vec![-PI / 2.0, -PI / 4.0, -PI / 6.0];
        
        let sin_result = sin(&input).unwrap();
        assert!((sin_result[0] - (-1.0)).abs() < 1e-10);
        assert!((sin_result[1] - (-2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((sin_result[2] - (-0.5)).abs() < 1e-10);
        
        let cos_result = cos(&input).unwrap();
        assert!((cos_result[0] - 0.0).abs() < 1e-10);
        assert!((cos_result[1] - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!((cos_result[2] - (3.0_f64.sqrt() / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_large_values() {
        let input = vec![10.0 * PI, 100.0 * PI];
        
        let sin_result = sin(&input).unwrap();
        let cos_result = cos(&input).unwrap();
        
        // Should handle large values gracefully
        for &val in &sin_result {
            assert!(val >= -1.0 && val <= 1.0);
        }
        
        for &val in &cos_result {
            assert!(val >= -1.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_special_values() {
        // Test special trigonometric values
        let input = vec![0.0, PI / 2.0, PI, 3.0 * PI / 2.0, 2.0 * PI];
        
        let sin_result = sin(&input).unwrap();
        assert!((sin_result[0] - 0.0).abs() < 1e-10);
        assert!((sin_result[1] - 1.0).abs() < 1e-10);
        assert!((sin_result[2] - 0.0).abs() < 1e-10);
        assert!((sin_result[3] - (-1.0)).abs() < 1e-10);
        assert!((sin_result[4] - 0.0).abs() < 1e-10);
        
        let cos_result = cos(&input).unwrap();
        assert!((cos_result[0] - 1.0).abs() < 1e-10);
        assert!((cos_result[1] - 0.0).abs() < 1e-10);
        assert!((cos_result[2] - (-1.0)).abs() < 1e-10);
        assert!((cos_result[3] - 0.0).abs() < 1e-10);
        assert!((cos_result[4] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse_functions_consistency() {
        // Test that sin(asin(x)) = x for valid range
        let input = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
        
        let asin_result = asin(&input).unwrap();
        let sin_asin_result = sin(&asin_result).unwrap();
        
        for (i, &original) in input.iter().enumerate() {
            assert!((sin_asin_result[i] - original).abs() < 1e-10);
        }
        
        // Test that cos(acos(x)) = x for valid range
        let acos_result = acos(&input).unwrap();
        let cos_acos_result = cos(&acos_result).unwrap();
        
        for (i, &original) in input.iter().enumerate() {
            assert!((cos_acos_result[i] - original).abs() < 1e-10);
        }
    }
}