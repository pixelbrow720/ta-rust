//! Average Price (AVGPRICE)

use crate::common::{TAResult, Price};
use crate::common::utils::{validate_not_empty, validate_same_length, allocate_output};

/// Calculates the Average Price (AVGPRICE)
///
/// The Average Price is the arithmetic mean of the Open, High, Low, and Close prices.
/// It provides a single representative price for each period.
///
/// # Formula
/// ```text
/// AVGPRICE = (Open + High + Low + Close) / 4
/// ```
///
/// # Parameters
/// - `open`: Slice of opening prices
/// - `high`: Slice of high prices
/// - `low`: Slice of low prices
/// - `close`: Slice of closing prices
///
/// # Returns
/// Vector of average price values.
///
/// # Errors
/// - `EmptyInput` if any input array is empty
/// - `MismatchedInputs` if input arrays have different lengths
///
/// # Example
/// ```rust
/// use ta_rust::price_transform::avgprice;
///
/// let open = vec![10.0, 11.0, 12.0];
/// let high = vec![12.0, 13.0, 14.0];
/// let low = vec![9.0, 10.0, 11.0];
/// let close = vec![11.0, 12.0, 13.0];
/// 
/// let result = avgprice(&open, &high, &low, &close).unwrap();
/// // result[0] = (10+12+9+11)/4 = 10.5
/// assert_eq!(result[0], 10.5);
/// ```
pub fn avgprice(open: &[Price], high: &[Price], low: &[Price], close: &[Price]) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(open, "open")?;
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_not_empty(close, "close")?;
    
    validate_same_length(open, high, "open", "high")?;
    validate_same_length(open, low, "open", "low")?;
    validate_same_length(open, close, "open", "close")?;

    let mut output = allocate_output(open.len());
    
    // Calculate average price for each period
    for i in 0..open.len() {
        output[i] = (open[i] + high[i] + low[i] + close[i]) / 4.0;
    }

    Ok(output)
}

/// Calculates Average Price from OHLC struct data
///
/// Convenience function for when data is already in OHLC format.
///
/// # Parameters
/// - `ohlc_data`: Slice of OHLC data structures
///
/// # Returns
/// Vector of average price values.
pub fn avgprice_from_ohlc(ohlc_data: &[crate::common::types::OHLC]) -> TAResult<Vec<Price>> {
    validate_not_empty(ohlc_data, "ohlc_data")?;
    
    let mut output = allocate_output(ohlc_data.len());
    
    for (i, ohlc) in ohlc_data.iter().enumerate() {
        output[i] = ohlc.average_price();
    }
    
    Ok(output)
}

/// Calculates weighted average price with custom weights
///
/// This version allows for custom weighting of the OHLC components.
///
/// # Parameters
/// - `open`: Slice of opening prices
/// - `high`: Slice of high prices
/// - `low`: Slice of low prices
/// - `close`: Slice of closing prices
/// - `weights`: Tuple of weights (open_weight, high_weight, low_weight, close_weight)
///
/// # Example
/// ```rust
/// use ta_rust::price_transform::avgprice_weighted;
///
/// let open = vec![10.0, 11.0];
/// let high = vec![12.0, 13.0];
/// let low = vec![9.0, 10.0];
/// let close = vec![11.0, 12.0];
/// 
/// // Give more weight to close price
/// let weights = (1.0, 1.0, 1.0, 2.0);
/// let result = avgprice_weighted(&open, &high, &low, &close, weights).unwrap();
/// ```
pub fn avgprice_weighted(
    open: &[Price], 
    high: &[Price], 
    low: &[Price], 
    close: &[Price],
    weights: (Price, Price, Price, Price)
) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(open, "open")?;
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_not_empty(close, "close")?;
    
    validate_same_length(open, high, "open", "high")?;
    validate_same_length(open, low, "open", "low")?;
    validate_same_length(open, close, "open", "close")?;
    
    let (w_open, w_high, w_low, w_close) = weights;
    let weight_sum = w_open + w_high + w_low + w_close;
    
    if weight_sum <= 0.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "weights",
            "sum of weights must be positive"
        ));
    }

    let mut output = allocate_output(open.len());
    
    // Calculate weighted average price for each period
    for i in 0..open.len() {
        output[i] = (open[i] * w_open + high[i] * w_high + low[i] * w_low + close[i] * w_close) / weight_sum;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};
    use crate::common::types::OHLC;

    #[test]
    fn test_avgprice_basic() {
        let open = vec![10.0, 11.0, 12.0];
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0];
        let close = vec![11.0, 12.0, 13.0];
        
        let result = avgprice(&open, &high, &low, &close).unwrap();
        
        let expected = vec![
            (10.0 + 12.0 + 9.0 + 11.0) / 4.0,  // 10.5
            (11.0 + 13.0 + 10.0 + 12.0) / 4.0, // 11.5
            (12.0 + 14.0 + 11.0 + 13.0) / 4.0, // 12.5
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_avgprice_single_value() {
        let open = vec![10.0];
        let high = vec![12.0];
        let low = vec![9.0];
        let close = vec![11.0];
        
        let result = avgprice(&open, &high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 10.5);
    }

    #[test]
    fn test_avgprice_from_ohlc() {
        let ohlc_data = vec![
            OHLC::new(10.0, 12.0, 9.0, 11.0),
            OHLC::new(11.0, 13.0, 10.0, 12.0),
            OHLC::new(12.0, 14.0, 11.0, 13.0),
        ];
        
        let result = avgprice_from_ohlc(&ohlc_data).unwrap();
        
        let expected = vec![10.5, 11.5, 12.5];
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_avgprice_weighted() {
        let open = vec![10.0, 11.0];
        let high = vec![12.0, 13.0];
        let low = vec![9.0, 10.0];
        let close = vec![11.0, 12.0];
        
        // Equal weights (should equal regular avgprice)
        let weights = (1.0, 1.0, 1.0, 1.0);
        let result1 = avgprice_weighted(&open, &high, &low, &close, weights).unwrap();
        let result2 = avgprice(&open, &high, &low, &close).unwrap();
        
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
        
        // Double weight on close
        let weights = (1.0, 1.0, 1.0, 2.0);
        let result_weighted = avgprice_weighted(&open, &high, &low, &close, weights).unwrap();
        
        let expected = vec![
            (10.0 + 12.0 + 9.0 + 11.0 * 2.0) / 5.0, // 10.6
            (11.0 + 13.0 + 10.0 + 12.0 * 2.0) / 5.0, // 11.6
        ];
        
        assert_arrays_approx_equal(&result_weighted, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_avgprice_constant_ohlc() {
        // When all OHLC values are the same
        let open = vec![10.0; 5];
        let high = vec![10.0; 5];
        let low = vec![10.0; 5];
        let close = vec![10.0; 5];
        
        let result = avgprice(&open, &high, &low, &close).unwrap();
        
        for &value in &result {
            assert_eq!(value, 10.0);
        }
    }

    #[test]
    fn test_avgprice_mismatched_lengths() {
        let open = vec![10.0, 11.0, 12.0];
        let high = vec![12.0, 13.0]; // Different length
        let low = vec![9.0, 10.0, 11.0];
        let close = vec![11.0, 12.0, 13.0];
        
        assert!(avgprice(&open, &high, &low, &close).is_err());
    }

    #[test]
    fn test_avgprice_empty_input() {
        let open = vec![];
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        assert!(avgprice(&open, &high, &low, &close).is_err());
    }

    #[test]
    fn test_avgprice_weighted_zero_weights() {
        let open = vec![10.0];
        let high = vec![12.0];
        let low = vec![9.0];
        let close = vec![11.0];
        
        let weights = (0.0, 0.0, 0.0, 0.0);
        assert!(avgprice_weighted(&open, &high, &low, &close, weights).is_err());
    }

    #[test]
    fn test_avgprice_weighted_negative_weights() {
        let open = vec![10.0];
        let high = vec![12.0];
        let low = vec![9.0];
        let close = vec![11.0];
        
        // Negative weights are allowed as long as sum is positive
        let weights = (-1.0, 2.0, 2.0, 2.0);
        let result = avgprice_weighted(&open, &high, &low, &close, weights).unwrap();
        
        let expected = (-10.0 + 24.0 + 18.0 + 22.0) / 5.0; // 10.8
        assert!((result[0] - expected).abs() < DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_avgprice_real_market_data() {
        let open = vec![44.20, 44.30, 44.09, 44.15, 43.61];
        let high = vec![44.34, 44.30, 44.20, 44.20, 44.45];
        let low = vec![44.09, 44.09, 44.05, 43.61, 43.61];
        let close = vec![44.34, 44.09, 44.15, 43.61, 44.33];
        
        let result = avgprice(&open, &high, &low, &close).unwrap();
        
        // All values should be reasonable prices
        for &price in &result {
            assert!(price > 40.0 && price < 50.0);
        }
        
        // Average price should be between the min and max of all OHLC values
        let all_prices: Vec<Price> = open.iter()
            .chain(high.iter())
            .chain(low.iter())
            .chain(close.iter())
            .copied()
            .collect();
        
        let min_price = all_prices.iter().fold(Price::INFINITY, |a, &b| a.min(b));
        let max_price = all_prices.iter().fold(Price::NEG_INFINITY, |a, &b| a.max(b));
        
        for &price in &result {
            assert!(price >= min_price && price <= max_price);
        }
    }

    #[test]
    fn test_avgprice_vs_ohlc_methods() {
        let open = vec![10.0, 11.0, 12.0];
        let high = vec![12.0, 13.0, 14.0];
        let low = vec![9.0, 10.0, 11.0];
        let close = vec![11.0, 12.0, 13.0];
        
        let avgprice_result = avgprice(&open, &high, &low, &close).unwrap();
        
        // Compare with OHLC struct method
        for i in 0..open.len() {
            let ohlc = OHLC::new(open[i], high[i], low[i], close[i]);
            assert!((avgprice_result[i] - ohlc.average_price()).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_avgprice_mathematical_properties() {
        let open = vec![10.0, 20.0];
        let high = vec![15.0, 25.0];
        let low = vec![8.0, 18.0];
        let close = vec![12.0, 22.0];
        
        let result = avgprice(&open, &high, &low, &close).unwrap();
        
        // Average price should be the arithmetic mean
        for i in 0..open.len() {
            let expected = (open[i] + high[i] + low[i] + close[i]) / 4.0;
            assert!((result[i] - expected).abs() < DEFAULT_TOLERANCE);
        }
        
        // For the second set of prices (all +10 from first), avgprice should also be +10
        assert!((result[1] - result[0] - 10.0).abs() < DEFAULT_TOLERANCE);
    }
}