//! Double Exponential Moving Average (DEMA)

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output};
use crate::overlap::ema::ema;

/// Calculates the Double Exponential Moving Average (DEMA)
///
/// DEMA is designed to reduce the lag inherent in moving averages by applying
/// a double smoothing technique. It's more responsive than a simple EMA.
///
/// # Formula
/// ```text
/// EMA1 = EMA(Price, period)
/// EMA2 = EMA(EMA1, period)
/// DEMA = 2 × EMA1 - EMA2
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Vector of DEMA values. The first `2×period-2` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < 2×period-1
///
/// # Example
/// ```rust
/// use ta_rust::overlap::dema;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let result = dema(&prices, 3).unwrap();
/// 
/// // DEMA will have valid values starting from index 4 (2×3-2)
/// assert!(result[3].is_nan());
/// assert!(!result[4].is_nan());
/// ```
pub fn dema(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    
    // DEMA requires at least 2×period-1 data points
    let min_required = 2 * period - 1;
    validate_sufficient_data(data, min_required, "data")?;

    // Calculate first EMA
    let ema1 = ema(data, period)?;
    
    // Extract non-NaN values from EMA1 for second EMA calculation
    let ema1_valid: Vec<Price> = ema1.iter()
        .skip(period - 1)
        .copied()
        .collect();
    
    // Calculate second EMA
    let ema2_partial = ema(&ema1_valid, period)?;
    
    // Combine results
    let mut output = allocate_output(data.len());
    
    // Calculate DEMA values
    let start_index = 2 * period - 2;
    for i in start_index..data.len() {
        let ema1_val = ema1[i];
        let ema2_index = i - (period - 1);
        if ema2_index < ema2_partial.len() && !ema2_partial[ema2_index].is_nan() {
            let ema2_val = ema2_partial[ema2_index];
            output[i] = 2.0 * ema1_val - ema2_val;
        }
    }

    Ok(output)
}

/// Calculates DEMA using a more direct approach
///
/// This version calculates both EMAs in a single pass for better performance.
pub fn dema_direct(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    
    let min_required = 2 * period - 1;
    validate_sufficient_data(data, min_required, "data")?;

    let mut output = allocate_output(data.len());
    let multiplier = 2.0 / (period as Price + 1.0);
    
    // Initialize first EMA with SMA of first 'period' values
    let initial_sum: Price = data[0..period].iter().sum();
    let mut ema1 = initial_sum / period as Price;
    
    // Track EMA1 values for EMA2 calculation
    let mut ema1_values = Vec::with_capacity(data.len());
    
    // Calculate EMA1 for all values
    for i in 0..data.len() {
        if i >= period - 1 {
            if i > period - 1 {
                ema1 = (data[i] * multiplier) + (ema1 * (1.0 - multiplier));
            }
            ema1_values.push(ema1);
        }
    }
    
    // Calculate EMA2 from EMA1 values
    if ema1_values.len() >= period {
        let initial_sum2: Price = ema1_values[0..period].iter().sum();
        let mut ema2 = initial_sum2 / period as Price;
        
        // Calculate DEMA values
        let start_index = 2 * period - 2;
        for i in start_index..data.len() {
            let ema1_val = ema1_values[i - (period - 1)];
            
            if i > start_index {
                let ema1_for_ema2 = ema1_values[i - (period - 1)];
                ema2 = (ema1_for_ema2 * multiplier) + (ema2 * (1.0 - multiplier));
            }
            
            output[i] = 2.0 * ema1_val - ema2;
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE, RELAXED_TOLERANCE};

    #[test]
    fn test_dema_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = dema(&data, 3).unwrap();
        
        // First 4 values should be NaN (2×3-2 = 4)
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        
        // Should have valid values from index 4 onwards
        for i in 4..data.len() {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_dema_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = dema(&data, 1).unwrap();
        
        // With period 1, DEMA should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_dema_known_calculation() {
        // Test with simple data where we can verify the calculation
        let data = vec![10.0, 10.0, 10.0, 10.0, 10.0, 10.0];
        let result = dema(&data, 2).unwrap();
        
        // For constant data, DEMA should equal the constant value
        for i in 2..data.len() {
            assert!((result[i] - 10.0).abs() < RELAXED_TOLERANCE);
        }
    }

    #[test]
    fn test_dema_vs_ema_responsiveness() {
        // DEMA should be more responsive than EMA
        let mut data = vec![10.0; 10];
        data.extend(vec![20.0; 10]); // Sudden jump
        
        let dema_result = dema(&data, 5).unwrap();
        let ema_result = ema(&data, 5).unwrap();
        
        // Find first valid DEMA value after the jump
        let jump_index = 10;
        let dema_after_jump = dema_result[jump_index + 2];
        let ema_after_jump = ema_result[jump_index + 2];
        
        // DEMA should react more strongly to the price change
        if !dema_after_jump.is_nan() && !ema_after_jump.is_nan() {
            assert!(dema_after_jump > ema_after_jump);
        }
    }

    #[test]
    fn test_dema_direct_equivalence() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result1 = dema(&data, 3).unwrap();
        let result2 = dema_direct(&data, 3).unwrap();
        
        // Both methods should produce similar results
        assert_arrays_approx_equal(&result1, &result2, RELAXED_TOLERANCE);
    }

    #[test]
    fn test_dema_trending_data() {
        // Test with trending data
        let data: Vec<Price> = (1..=20).map(|x| x as Price).collect();
        let result = dema(&data, 5).unwrap();
        
        // DEMA should follow the trend
        let valid_start = 2 * 5 - 2; // 8
        for i in (valid_start + 1)..data.len() {
            if !result[i].is_nan() && !result[i-1].is_nan() {
                // Should generally trend upward for increasing data
                assert!(result[i] >= result[i-1] - RELAXED_TOLERANCE);
            }
        }
    }

    #[test]
    fn test_dema_insufficient_data() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(dema(&data, 3).is_err()); // Needs at least 2×3-1 = 5 points
    }

    #[test]
    fn test_dema_empty_data() {
        let data = vec![];
        assert!(dema(&data, 3).is_err());
    }

    #[test]
    fn test_dema_zero_period() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(dema(&data, 0).is_err());
    }

    #[test]
    fn test_dema_minimum_data_requirement() {
        // Test with exactly the minimum required data
        let period = 3;
        let min_data = 2 * period - 1; // 5
        let data: Vec<Price> = (1..=min_data).map(|x| x as Price).collect();
        
        let result = dema(&data, period).unwrap();
        
        // Should have exactly one valid value at the end
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert_eq!(valid_count, 1);
    }

    #[test]
    fn test_dema_real_market_data() {
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
            46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80
        ];
        
        let result = dema(&data, 8).unwrap();
        
        // Should have valid values starting from index 14 (2×8-2)
        let start_index = 2 * 8 - 2;
        for i in 0..start_index {
            assert!(result[i].is_nan());
        }
        for i in start_index..data.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
    }

    #[test]
    fn test_dema_formula_verification() {
        // Verify DEMA formula: DEMA = 2×EMA1 - EMA2
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let period = 3;
        
        let ema1 = ema(&data, period).unwrap();
        let ema1_valid: Vec<Price> = ema1.iter().skip(period - 1).copied().collect();
        let ema2 = ema(&ema1_valid, period).unwrap();
        
        let dema_result = dema(&data, period).unwrap();
        
        // Verify formula for valid indices
        let start_index = 2 * period - 2;
        for i in start_index..data.len() {
            if !dema_result[i].is_nan() {
                let ema2_index = i - (period - 1);
                if ema2_index < ema2.len() && !ema2[ema2_index].is_nan() {
                    let expected = 2.0 * ema1[i] - ema2[ema2_index];
                    assert!((dema_result[i] - expected).abs() < DEFAULT_TOLERANCE);
                }
            }
        }
    }
}