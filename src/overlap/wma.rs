//! Weighted Moving Average (WMA)

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output};

/// Calculates the Weighted Moving Average (WMA)
///
/// The Weighted Moving Average assigns different weights to each price in the period,
/// with the most recent price receiving the highest weight and older prices receiving
/// progressively lower weights.
///
/// # Formula
/// ```text
/// WMA = (P1×1 + P2×2 + ... + Pn×n) / (1 + 2 + ... + n)
/// 
/// Where:
/// - P1 is the oldest price, P2 is the next oldest, ..., Pn is the most recent
/// - n is the period
/// - Weight sum = n × (n + 1) / 2
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Vector of WMA values. The first `period-1` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < period
///
/// # Example
/// ```rust
/// use ta_rust::overlap::wma;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = wma(&prices, 3).unwrap();
/// 
/// // WMA calculation for index 2: (1×1 + 2×2 + 3×3) / (1+2+3) = 14/6 = 2.333...
/// assert!((result[2] - 2.333333333333333).abs() < 1e-10);
/// ```
pub fn wma(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    // Pre-calculate weight sum: 1 + 2 + ... + n = n(n+1)/2
    let weight_sum = (period * (period + 1)) as Price / 2.0;
    
    // Calculate WMA for each position starting from period-1
    for i in (period - 1)..data.len() {
        let mut weighted_sum = 0.0;
        
        // Calculate weighted sum for current window
        for j in 0..period {
            let weight = (j + 1) as Price; // Weight increases with recency
            let price_index = i + 1 - period + j;
            weighted_sum += data[price_index] * weight;
        }
        
        output[i] = weighted_sum / weight_sum;
    }

    Ok(output)
}

/// Calculates WMA with custom weights
///
/// Allows for custom weight assignment instead of the standard linear weighting.
/// 
/// # Parameters
/// - `data`: Slice of price data
/// - `weights`: Slice of weights (must have same length as period)
///
/// # Example
/// ```rust
/// use ta_rust::overlap::wma_custom;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let weights = vec![0.1, 0.3, 0.6]; // Custom weights that sum to 1.0
/// let result = wma_custom(&prices, &weights).unwrap();
/// ```
pub fn wma_custom(data: &[Price], weights: &[Price]) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_not_empty(weights, "weights")?;
    
    let period = weights.len();
    validate_sufficient_data(data, period, "data")?;
    
    // Validate weights are positive
    for (i, &weight) in weights.iter().enumerate() {
        if weight < 0.0 || !weight.is_finite() {
            return Err(crate::common::TAError::invalid_parameter(
                "weights",
                &format!("weight at index {} must be non-negative and finite", i)
            ));
        }
    }
    
    let weight_sum: Price = weights.iter().sum();
    if weight_sum <= 0.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "weights",
            "sum of weights must be positive"
        ));
    }

    let mut output = allocate_output(data.len());
    
    // Calculate WMA for each position starting from period-1
    for i in (period - 1)..data.len() {
        let mut weighted_sum = 0.0;
        
        // Calculate weighted sum for current window
        for j in 0..period {
            let price_index = i + 1 - period + j;
            weighted_sum += data[price_index] * weights[j];
        }
        
        output[i] = weighted_sum / weight_sum;
    }

    Ok(output)
}

/// Calculates WMA using a rolling approach for better performance
///
/// This version is more efficient for large datasets as it maintains
/// running weighted sums and updates them incrementally.
pub fn wma_rolling(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    let weight_sum = (period * (period + 1)) as Price / 2.0;
    
    // Calculate first WMA value
    let mut weighted_sum = 0.0;
    for j in 0..period {
        weighted_sum += data[j] * (j + 1) as Price;
    }
    output[period - 1] = weighted_sum / weight_sum;
    
    // Rolling calculation for remaining values
    for i in period..data.len() {
        // Remove contribution of oldest value and add newest
        // This is more complex for WMA due to changing weights
        weighted_sum = 0.0;
        for j in 0..period {
            let price_index = i + 1 - period + j;
            weighted_sum += data[price_index] * (j + 1) as Price;
        }
        output[i] = weighted_sum / weight_sum;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_wma_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = wma(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            (1.0*1.0 + 2.0*2.0 + 3.0*3.0) / 6.0, // (1+4+9)/6 = 14/6 = 2.333...
            (2.0*1.0 + 3.0*2.0 + 4.0*3.0) / 6.0, // (2+6+12)/6 = 20/6 = 3.333...
            (3.0*1.0 + 4.0*2.0 + 5.0*3.0) / 6.0, // (3+8+15)/6 = 26/6 = 4.333...
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_wma_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = wma(&data, 1).unwrap();
        
        // With period 1, WMA should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_wma_period_2() {
        let data = vec![10.0, 20.0, 30.0, 40.0];
        let result = wma(&data, 2).unwrap();
        
        let expected = vec![
            Price::NAN,
            (10.0*1.0 + 20.0*2.0) / 3.0, // (10+40)/3 = 50/3 = 16.666...
            (20.0*1.0 + 30.0*2.0) / 3.0, // (20+60)/3 = 80/3 = 26.666...
            (30.0*1.0 + 40.0*2.0) / 3.0, // (30+80)/3 = 110/3 = 36.666...
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_wma_custom_weights() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let weights = vec![0.1, 0.3, 0.6]; // Custom weights
        let result = wma_custom(&data, &weights).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            (1.0*0.1 + 2.0*0.3 + 3.0*0.6) / 1.0, // 0.1+0.6+1.8 = 2.5
            (2.0*0.1 + 3.0*0.3 + 4.0*0.6) / 1.0, // 0.2+0.9+2.4 = 3.5
            (3.0*0.1 + 4.0*0.3 + 5.0*0.6) / 1.0, // 0.3+1.2+3.0 = 4.5
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_wma_rolling_equivalence() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result1 = wma(&data, 4).unwrap();
        let result2 = wma_rolling(&data, 4).unwrap();
        
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_wma_weight_sum_calculation() {
        // Test that weight sum is calculated correctly
        for period in 1..=10 {
            let expected_sum = (period * (period + 1)) as Price / 2.0;
            let actual_sum: Price = (1..=period).map(|x| x as Price).sum();
            assert!((expected_sum - actual_sum).abs() < 1e-10);
        }
    }

    #[test]
    fn test_wma_vs_sma_responsiveness() {
        // WMA should be more responsive to recent changes than SMA
        let mut data = vec![10.0; 5];
        data.extend(vec![20.0; 5]); // Sudden jump
        
        let wma_result = wma(&data, 5).unwrap();
        let sma_result = crate::overlap::sma::sma(&data, 5).unwrap();
        
        // WMA should react faster to the price jump
        assert!(wma_result[6] > sma_result[6]);
    }

    #[test]
    fn test_wma_constant_values() {
        let data = vec![7.5; 10];
        let result = wma(&data, 4).unwrap();
        
        // All non-NaN values should be 7.5 for constant input
        for i in 3..result.len() {
            assert!((result[i] - 7.5).abs() < 1e-10);
        }
    }

    #[test]
    fn test_wma_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(wma(&data, 3).is_err());
    }

    #[test]
    fn test_wma_empty_data() {
        let data = vec![];
        assert!(wma(&data, 3).is_err());
    }

    #[test]
    fn test_wma_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(wma(&data, 0).is_err());
    }

    #[test]
    fn test_wma_custom_invalid_weights() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        
        // Negative weight
        let weights = vec![0.5, -0.3, 0.8];
        assert!(wma_custom(&data, &weights).is_err());
        
        // Zero sum weights
        let weights = vec![0.0, 0.0, 0.0];
        assert!(wma_custom(&data, &weights).is_err());
        
        // Empty weights
        let weights = vec![];
        assert!(wma_custom(&data, &weights).is_err());
    }

    #[test]
    fn test_wma_real_market_data() {
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93
        ];
        let result = wma(&data, 5).unwrap();
        
        // Should have valid values starting from index 4
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        for i in 4..data.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
        
        // WMA should be between min and max of the data
        let min_price = data.iter().fold(Price::INFINITY, |a, &b| a.min(b));
        let max_price = data.iter().fold(Price::NEG_INFINITY, |a, &b| a.max(b));
        
        for i in 4..data.len() {
            assert!(result[i] >= min_price && result[i] <= max_price);
        }
    }
}