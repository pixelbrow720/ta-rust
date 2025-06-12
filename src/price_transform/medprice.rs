//! Median Price (MEDPRICE)

use crate::common::{TAResult, Price};
use crate::common::utils::{validate_not_empty, validate_same_length, allocate_output};

/// Calculates the Median Price (MEDPRICE)
///
/// The Median Price is the average of the High and Low prices.
/// It represents the midpoint of the trading range for each period.
///
/// # Formula
/// ```text
/// MEDPRICE = (High + Low) / 2
/// ```
///
/// # Parameters
/// - `high`: Slice of high prices
/// - `low`: Slice of low prices
///
/// # Returns
/// Vector of median price values.
///
/// # Errors
/// - `EmptyInput` if any input array is empty
/// - `MismatchedInputs` if input arrays have different lengths
///
/// # Example
/// ```rust
/// use ta_rust::price_transform::medprice;
///
/// let high = vec![12.0, 13.0, 14.0];
/// let low = vec![9.0, 10.0, 11.0];
/// 
/// let result = medprice(&high, &low).unwrap();
/// // result[0] = (12+9)/2 = 10.5
/// assert_eq!(result[0], 10.5);
/// ```
pub fn medprice(high: &[Price], low: &[Price]) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_same_length(high, low, "high", "low")?;

    let mut output = allocate_output(high.len());
    
    // Calculate median price for each period
    for i in 0..high.len() {
        output[i] = (high[i] + low[i]) / 2.0;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_medprice_basic() {
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0];
        
        let result = medprice(&high, &low).unwrap();
        
        let expected = vec![
            (12.0 + 9.0) / 2.0,   // 10.5
            (13.0 + 10.0) / 2.0, // 11.5
            (14.0 + 11.0) / 2.0, // 12.5
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_medprice_single_value() {
        let high = vec![12.0];
        let low = vec![9.0];
        
        let result = medprice(&high, &low).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 10.5);
    }

    #[test]
    fn test_medprice_equal_high_low() {
        let high = vec![10.0, 10.0, 10.0];
        let low = vec![10.0, 10.0, 10.0];
        
        let result = medprice(&high, &low).unwrap();
        
        for &value in &result {
            assert_eq!(value, 10.0);
        }
    }

    #[test]
    fn test_medprice_mismatched_lengths() {
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0]; // Different length
        
        assert!(medprice(&high, &low).is_err());
    }

    #[test]
    fn test_medprice_empty_input() {
        let high = vec![];
        let low = vec![];
        
        assert!(medprice(&high, &low).is_err());
    }
}