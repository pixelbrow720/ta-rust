//! Weighted Close Price (WCLPRICE)

use crate::common::{TAResult, Price};
use crate::common::utils::{validate_not_empty, validate_same_length, allocate_output};

/// Calculates the Weighted Close Price (WCLPRICE)
///
/// The Weighted Close Price gives more weight to the closing price,
/// which is often considered the most important price of the period.
/// It's calculated as the average of High, Low, and twice the Close price.
///
/// # Formula
/// ```text
/// WCLPRICE = (High + Low + 2 × Close) / 4
/// ```
///
/// # Parameters
/// - `high`: Slice of high prices
/// - `low`: Slice of low prices
/// - `close`: Slice of close prices
///
/// # Returns
/// Vector of weighted close price values.
///
/// # Errors
/// - `EmptyInput` if any input array is empty
/// - `MismatchedInputs` if input arrays have different lengths
///
/// # Example
/// ```rust
/// use ta_rust::price_transform::wclprice;
///
/// let high = vec![12.0, 13.0, 14.0];
/// let low = vec![9.0, 10.0, 11.0];
/// let close = vec![11.0, 12.0, 13.0];
/// 
/// let result = wclprice(&high, &low, &close).unwrap();
/// // result[0] = (12+9+2*11)/4 = 10.75
/// assert_eq!(result[0], 10.75);
/// ```
pub fn wclprice(high: &[Price], low: &[Price], close: &[Price]) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_not_empty(close, "close")?;
    validate_same_length(high, low, "high", "low")?;
    validate_same_length(high, close, "high", "close")?;

    let mut output = allocate_output(high.len());
    
    // Calculate weighted close price for each period
    for i in 0..high.len() {
        output[i] = (high[i] + low[i] + 2.0 * close[i]) / 4.0;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_wclprice_basic() {
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0];
        let close = vec![11.0, 12.0, 13.0];
        
        let result = wclprice(&high, &low, &close).unwrap();
        
        let expected = vec![
            (12.0 + 9.0 + 2.0 * 11.0) / 4.0,   // 10.75
            (13.0 + 10.0 + 2.0 * 12.0) / 4.0,  // 11.75
            (14.0 + 11.0 + 2.0 * 13.0) / 4.0,  // 12.75
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_wclprice_single_value() {
        let high = vec![12.0];
        let low = vec![9.0];
        let close = vec![11.0];
        
        let result = wclprice(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 10.75);
    }

    #[test]
    fn test_wclprice_equal_hlc() {
        let high = vec![10.0, 10.0, 10.0];
        let low = vec![10.0, 10.0, 10.0];
        let close = vec![10.0, 10.0, 10.0];
        
        let result = wclprice(&high, &low, &close).unwrap();
        
        for &value in &result {
            assert_eq!(value, 10.0);
        }
    }

    #[test]
    fn test_wclprice_vs_typprice() {
        // WCLPRICE gives more weight to close than TYPPRICE
        let high = vec![12.0];
        let low = vec![8.0];
        let close = vec![11.0]; // Close to high
        
        let wclprice_result = wclprice(&high, &low, &close).unwrap();
        let typprice_result = crate::price_transform::typprice::typprice(&high, &low, &close).unwrap();
        
        // WCLPRICE should be higher than TYPPRICE when close is high
        assert!(wclprice_result[0] > typprice_result[0]);
    }

    #[test]
    fn test_wclprice_mismatched_lengths() {
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0]; // Different length
        let close = vec![11.0, 12.0, 13.0];
        
        assert!(wclprice(&high, &low, &close).is_err());
    }

    #[test]
    fn test_wclprice_empty_input() {
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        assert!(wclprice(&high, &low, &close).is_err());
    }
}