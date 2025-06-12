//! Typical Price (TYPPRICE)

use crate::common::{TAResult, Price};
use crate::common::utils::{validate_not_empty, validate_same_length, allocate_output};

/// Calculates the Typical Price (TYPPRICE)
///
/// The Typical Price is the average of the High, Low, and Close prices.
/// It's commonly used in volume-weighted indicators and provides a 
/// representative price that considers the full trading range.
///
/// # Formula
/// ```text
/// TYPPRICE = (High + Low + Close) / 3
/// ```
///
/// # Parameters
/// - `high`: Slice of high prices
/// - `low`: Slice of low prices
/// - `close`: Slice of close prices
///
/// # Returns
/// Vector of typical price values.
///
/// # Errors
/// - `EmptyInput` if any input array is empty
/// - `MismatchedInputs` if input arrays have different lengths
///
/// # Example
/// ```rust
/// use ta_rust::price_transform::typprice;
///
/// let high = vec![12.0, 13.0, 14.0];
/// let low = vec![9.0, 10.0, 11.0];
/// let close = vec![11.0, 12.0, 13.0];
/// 
/// let result = typprice(&high, &low, &close).unwrap();
/// // result[0] = (12+9+11)/3 = 10.667
/// assert!((result[0] - 10.666666666666666).abs() < 1e-10);
/// ```
pub fn typprice(high: &[Price], low: &[Price], close: &[Price]) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_not_empty(close, "close")?;
    validate_same_length(high, low, "high", "low")?;
    validate_same_length(high, close, "high", "close")?;

    let mut output = allocate_output(high.len());
    
    // Calculate typical price for each period
    for i in 0..high.len() {
        output[i] = (high[i] + low[i] + close[i]) / 3.0;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_typprice_basic() {
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0];
        let close = vec![11.0, 12.0, 13.0];
        
        let result = typprice(&high, &low, &close).unwrap();
        
        let expected = vec![
            (12.0 + 9.0 + 11.0) / 3.0,   // 10.666...
            (13.0 + 10.0 + 12.0) / 3.0,  // 11.666...
            (14.0 + 11.0 + 13.0) / 3.0,  // 12.666...
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_typprice_single_value() {
        let high = vec![12.0];
        let low = vec![9.0];
        let close = vec![11.0];
        
        let result = typprice(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 1);
        assert!((result[0] - 10.666666666666666).abs() < DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_typprice_equal_hlc() {
        let high = vec![10.0, 10.0, 10.0];
        let low = vec![10.0, 10.0, 10.0];
        let close = vec![10.0, 10.0, 10.0];
        
        let result = typprice(&high, &low, &close).unwrap();
        
        for &value in &result {
            assert_eq!(value, 10.0);
        }
    }

    #[test]
    fn test_typprice_mismatched_lengths() {
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0]; // Different length
        let close = vec![11.0, 12.0, 13.0];
        
        assert!(typprice(&high, &low, &close).is_err());
    }

    #[test]
    fn test_typprice_empty_input() {
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        assert!(typprice(&high, &low, &close).is_err());
    }
}