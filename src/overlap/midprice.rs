//! Midpoint Price over period

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, validate_same_length, allocate_output, highest_in_period, lowest_in_period};

/// Calculates the Midpoint Price over a specified period
///
/// The Midpoint Price is the average of the highest high and lowest low over a given period.
/// Unlike MidPoint which uses a single price series, MidPrice uses separate high and low series.
///
/// # Formula
/// ```text
/// MidPrice = (Highest(High, period) + Lowest(Low, period)) / 2
/// ```
///
/// # Parameters
/// - `high`: Slice of high price data
/// - `low`: Slice of low price data
/// - `period`: Number of periods to look back for highest and lowest values
///
/// # Returns
/// Vector of MidPrice values. The first `period-1` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < period
/// - `MismatchedInputs` if high and low arrays have different lengths
///
/// # Example
/// ```rust
/// use ta_rust::overlap::midprice;
///
/// let high = vec![2.0, 4.0, 3.0, 6.0, 5.0];
/// let low = vec![1.0, 2.0, 1.0, 4.0, 3.0];
/// let result = midprice(&high, &low, 3).unwrap();
/// 
/// // For index 2: highest_high(2,4,3) = 4, lowest_low(1,2,1) = 1, midprice = 2.5
/// assert_eq!(result[2], 2.5);
/// ```
pub fn midprice(high: &[Price], low: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_same_length(high, low, "high", "low")?;
    validate_period(period, "period")?;
    validate_sufficient_data(high, period, "high")?;

    let mut output = allocate_output(high.len());
    
    // Calculate midprice for each position starting from period-1
    for i in (period - 1)..high.len() {
        let start_index = i + 1 - period;
        let highest_high = highest_in_period(high, start_index, period);
        let lowest_low = lowest_in_period(low, start_index, period);
        output[i] = (highest_high + lowest_low) / 2.0;
    }

    Ok(output)
}

/// Calculates MidPrice using OHLC data
///
/// Convenience function that extracts high and low from OHLC data.
///
/// # Parameters
/// - `open`: Slice of open prices
/// - `high`: Slice of high prices
/// - `low`: Slice of low prices
/// - `close`: Slice of close prices
/// - `period`: Number of periods to look back
///
/// # Returns
/// Vector of MidPrice values calculated from the high and low series.
pub fn midprice_ohlc(
    _open: &[Price], 
    high: &[Price], 
    low: &[Price], 
    _close: &[Price], 
    period: Period
) -> TAResult<Vec<Price>> {
    // We only need high and low for midprice calculation
    midprice(high, low, period)
}

/// Calculates MidPrice with custom percentile levels
///
/// Instead of using absolute highest and lowest, this version uses
/// specified percentile levels (e.g., 90th percentile high, 10th percentile low).
///
/// # Parameters
/// - `high`: Slice of high price data
/// - `low`: Slice of low price data
/// - `period`: Number of periods to look back
/// - `high_percentile`: Percentile for high values (0.0 to 1.0)
/// - `low_percentile`: Percentile for low values (0.0 to 1.0)
///
/// # Example
/// ```rust
/// use ta_rust::overlap::midprice_percentile;
///
/// let high = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let low = vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5];
/// // Use 80th percentile for high and 20th percentile for low
/// let result = midprice_percentile(&high, &low, 5, 0.8, 0.2).unwrap();
/// ```
pub fn midprice_percentile(
    high: &[Price], 
    low: &[Price], 
    period: Period,
    high_percentile: Price,
    low_percentile: Price
) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_same_length(high, low, "high", "low")?;
    validate_period(period, "period")?;
    validate_sufficient_data(high, period, "high")?;
    
    if high_percentile < 0.0 || high_percentile > 1.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "high_percentile",
            "must be between 0.0 and 1.0"
        ));
    }
    
    if low_percentile < 0.0 || low_percentile > 1.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "low_percentile",
            "must be between 0.0 and 1.0"
        ));
    }

    let mut output = allocate_output(high.len());
    
    // Calculate midprice for each position using percentiles
    for i in (period - 1)..high.len() {
        let start_index = i + 1 - period;
        
        // Get high and low windows
        let high_window = &high[start_index..=i];
        let low_window = &low[start_index..=i];
        
        // Calculate percentiles
        let high_value = calculate_percentile(high_window, high_percentile);
        let low_value = calculate_percentile(low_window, low_percentile);
        
        output[i] = (high_value + low_value) / 2.0;
    }

    Ok(output)
}

/// Helper function to calculate percentile of a slice
fn calculate_percentile(data: &[Price], percentile: Price) -> Price {
    if data.is_empty() {
        return Price::NAN;
    }
    
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    
    let index = (percentile * (sorted_data.len() - 1) as Price).round() as usize;
    sorted_data[index.min(sorted_data.len() - 1)]
}

/// Calculates MidPrice with adaptive period based on volatility
///
/// This version adjusts the lookback period based on recent volatility,
/// using shorter periods during high volatility and longer periods during low volatility.
pub fn midprice_adaptive(
    high: &[Price], 
    low: &[Price], 
    base_period: Period,
    volatility_period: Period
) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(high, "high")?;
    validate_not_empty(low, "low")?;
    validate_same_length(high, low, "high", "low")?;
    validate_period(base_period, "base_period")?;
    validate_period(volatility_period, "volatility_period")?;
    
    let min_required = base_period.max(volatility_period);
    validate_sufficient_data(high, min_required, "high")?;

    let mut output = allocate_output(high.len());
    
    // Calculate adaptive midprice
    for i in min_required..high.len() {
        // Calculate recent volatility
        let vol_start = i + 1 - volatility_period;
        let high_range = &high[vol_start..=i];
        let low_range = &low[vol_start..=i];
        
        let volatility = calculate_hl_volatility(high_range, low_range);
        
        // Adjust period based on volatility (higher volatility = shorter period)
        let adjusted_period = if volatility > 0.05 {
            (base_period as Price * 0.7).round() as Period
        } else if volatility < 0.02 {
            (base_period as Price * 1.3).round() as Period
        } else {
            base_period
        }.max(1).min(i + 1);
        
        // Calculate midprice with adjusted period
        let start_index = i + 1 - adjusted_period;
        let highest_high = highest_in_period(high, start_index, adjusted_period);
        let lowest_low = lowest_in_period(low, start_index, adjusted_period);
        output[i] = (highest_high + lowest_low) / 2.0;
    }

    Ok(output)
}

/// Helper function to calculate high-low volatility
fn calculate_hl_volatility(high: &[Price], low: &[Price]) -> Price {
    if high.len() != low.len() || high.is_empty() {
        return 0.0;
    }
    
    let hl_ratios: Vec<Price> = high.iter()
        .zip(low.iter())
        .map(|(&h, &l)| if l > 0.0 { (h - l) / l } else { 0.0 })
        .collect();
    
    crate::common::utils::std_dev(&hl_ratios)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_midprice_basic() {
        let high = vec![2.0, 4.0, 3.0, 6.0, 5.0];
        let low = vec![1.0, 2.0, 1.0, 4.0, 3.0];
        let result = midprice(&high, &low, 3).unwrap();
        
        let _expected = vec![
            Price::NAN, Price::NAN,
            2.5, // (max_high(2,4,3) + min_low(1,2,1)) / 2 = (4+1)/2 = 2.5
            3.5, // (max_high(4,3,6) + min_low(2,1,4)) / 2 = (6+1)/2 = 3.5
            4.5, // (max_high(3,6,5) + min_low(1,4,3)) / 2 = (6+1)/2 = 3.5, wait let me recalculate
        ];
        
        // Let me recalculate the expected values
        // Index 2: high[0:3] = [2,4,3], max = 4; low[0:3] = [1,2,1], min = 1; midprice = 2.5
        // Index 3: high[1:4] = [4,3,6], max = 6; low[1:4] = [2,1,4], min = 1; midprice = 3.5
        // Index 4: high[2:5] = [3,6,5], max = 6; low[2:5] = [1,4,3], min = 1; midprice = 3.5
        
        let expected = vec![
            Price::NAN, Price::NAN,
            2.5, // (4+1)/2
            3.5, // (6+1)/2
            3.5, // (6+1)/2
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midprice_period_1() {
        let high = vec![2.0, 4.0, 3.0, 6.0, 5.0];
        let low = vec![1.0, 2.0, 1.0, 4.0, 3.0];
        let result = midprice(&high, &low, 1).unwrap();
        
        // With period 1, midprice should be (high + low) / 2 for each point
        let expected: Vec<Price> = high.iter()
            .zip(low.iter())
            .map(|(&h, &l)| (h + l) / 2.0)
            .collect();
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midprice_ohlc() {
        let open = vec![1.5, 3.5, 2.5, 5.5, 4.5];
        let high = vec![2.0, 4.0, 3.0, 6.0, 5.0];
        let low = vec![1.0, 2.0, 1.0, 4.0, 3.0];
        let close = vec![1.8, 3.8, 2.8, 5.8, 4.8];
        
        let result1 = midprice_ohlc(&open, &high, &low, &close, 3).unwrap();
        let result2 = midprice(&high, &low, 3).unwrap();
        
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midprice_percentile() {
        let high = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let low = vec![0.5, 1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5];
        
        // Test with 100th and 0th percentiles (should equal regular midprice)
        let result1 = midprice_percentile(&high, &low, 5, 1.0, 0.0).unwrap();
        let result2 = midprice(&high, &low, 5).unwrap();
        
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
        
        // Test with 50th percentiles (should be median-based)
        let result_median = midprice_percentile(&high, &low, 5, 0.5, 0.5).unwrap();
        assert!(!result_median[4].is_nan());
    }

    #[test]
    fn test_midprice_adaptive() {
        let high = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let low = vec![0.9, 1.8, 2.7, 3.6, 4.5, 5.4, 6.3, 7.2, 8.1, 9.0];
        
        let result = midprice_adaptive(&high, &low, 5, 3).unwrap();
        
        // Should have valid values starting from max(5, 3) = 5
        for i in 0..5 {
            assert!(result[i].is_nan());
        }
        for i in 5..result.len() {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_midprice_mismatched_inputs() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![0.5, 1.5]; // Different length
        
        assert!(midprice(&high, &low, 2).is_err());
    }

    #[test]
    fn test_midprice_constant_data() {
        let high = vec![5.0; 10];
        let low = vec![4.0; 10];
        let result = midprice(&high, &low, 4).unwrap();
        
        // All non-NaN values should be 4.5 for constant input
        for i in 3..result.len() {
            assert!((result[i] - 4.5).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_midprice_insufficient_data() {
        let high = vec![1.0, 2.0];
        let low = vec![0.5, 1.5];
        assert!(midprice(&high, &low, 3).is_err());
    }

    #[test]
    fn test_midprice_empty_data() {
        let high = vec![];
        let low = vec![];
        assert!(midprice(&high, &low, 3).is_err());
    }

    #[test]
    fn test_midprice_zero_period() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![0.5, 1.5, 2.5];
        assert!(midprice(&high, &low, 0).is_err());
    }

    #[test]
    fn test_midprice_percentile_invalid() {
        let high = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let low = vec![0.5, 1.5, 2.5, 3.5, 4.5];
        
        // Invalid percentiles
        assert!(midprice_percentile(&high, &low, 3, -0.1, 0.5).is_err());
        assert!(midprice_percentile(&high, &low, 3, 1.1, 0.5).is_err());
        assert!(midprice_percentile(&high, &low, 3, 0.5, -0.1).is_err());
        assert!(midprice_percentile(&high, &low, 3, 0.5, 1.1).is_err());
    }

    #[test]
    fn test_calculate_percentile() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        assert_eq!(calculate_percentile(&data, 0.0), 1.0);
        assert_eq!(calculate_percentile(&data, 0.5), 3.0);
        assert_eq!(calculate_percentile(&data, 1.0), 5.0);
        
        // Empty data
        assert!(calculate_percentile(&[], 0.5).is_nan());
    }

    #[test]
    fn test_hl_volatility_calculation() {
        let high = vec![2.0, 4.0, 3.0, 6.0];
        let low = vec![1.0, 2.0, 1.0, 4.0];
        
        let volatility = calculate_hl_volatility(&high, &low);
        assert!(volatility >= 0.0);
        
        // Constant spread should have low volatility
        let high_const = vec![2.0, 2.0, 2.0, 2.0];
        let low_const = vec![1.0, 1.0, 1.0, 1.0];
        let volatility_const = calculate_hl_volatility(&high_const, &low_const);
        assert!(volatility_const < volatility);
    }

    #[test]
    fn test_midprice_real_market_data() {
        let high = vec![
            44.50, 44.25, 44.30, 43.80, 44.50, 45.00, 46.00, 47.50, 48.00, 47.20
        ];
        let low = vec![
            44.00, 43.90, 44.00, 43.40, 44.10, 44.60, 45.70, 47.00, 47.80, 46.70
        ];
        
        let result = midprice(&high, &low, 5).unwrap();
        
        // Should have valid values starting from index 4
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        for i in 4..result.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
        
        // MidPrice should be between overall min and max
        let min_low = low.iter().fold(Price::INFINITY, |a, &b| a.min(b));
        let max_high = high.iter().fold(Price::NEG_INFINITY, |a, &b| a.max(b));
        
        for i in 4..result.len() {
            assert!(result[i] >= min_low && result[i] <= max_high);
        }
    }

    #[test]
    fn test_midprice_vs_midpoint_relationship() {
        // When high and low are the same, midprice should equal midpoint
        let prices = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let midprice_result = midprice(&prices, &prices, 3).unwrap();
        let midpoint_result = crate::overlap::midpoint::midpoint(&prices, 3).unwrap();
        
        assert_arrays_approx_equal(&midprice_result, &midpoint_result, DEFAULT_TOLERANCE);
    }
}