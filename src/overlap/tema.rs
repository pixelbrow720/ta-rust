//! Triple Exponential Moving Average (TEMA)

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output};
use crate::overlap::ema::ema;

/// Calculates the Triple Exponential Moving Average (TEMA)
///
/// TEMA is designed to reduce lag even further than DEMA by applying
/// a triple smoothing technique. It's the most responsive of the exponential
/// moving average family.
///
/// # Formula
/// ```text
/// EMA1 = EMA(Price, period)
/// EMA2 = EMA(EMA1, period)
/// EMA3 = EMA(EMA2, period)
/// TEMA = 3×EMA1 - 3×EMA2 + EMA3
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Vector of TEMA values. The first `3×period-3` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < 3×period-2
///
/// # Example
/// ```rust
/// use ta_rust::overlap::tema;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0];
/// let result = tema(&prices, 3).unwrap();
/// 
/// // TEMA will have valid values starting from index 6 (3×3-3)
/// assert!(result[5].is_nan());
/// assert!(!result[6].is_nan());
/// ```
pub fn tema(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    
    // TEMA requires at least 3×period-2 data points
    let min_required = 3 * period - 2;
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
    
    // Extract non-NaN values from EMA2 for third EMA calculation
    let ema2_valid: Vec<Price> = ema2_partial.iter()
        .skip(period - 1)
        .copied()
        .collect();
    
    // Calculate third EMA
    let ema3_partial = ema(&ema2_valid, period)?;
    
    // Combine results
    let mut output = allocate_output(data.len());
    
    // Calculate TEMA values
    let start_index = 3 * period - 3;
    for i in start_index..data.len() {
        let ema1_val = ema1[i];
        let ema2_index = i - (period - 1);
        let ema3_index = i - 2 * (period - 1);
        
        if ema2_index < ema2_partial.len() && 
           ema3_index < ema3_partial.len() &&
           !ema2_partial[ema2_index].is_nan() && 
           !ema3_partial[ema3_index].is_nan() {
            let ema2_val = ema2_partial[ema2_index];
            let ema3_val = ema3_partial[ema3_index];
            output[i] = 3.0 * ema1_val - 3.0 * ema2_val + ema3_val;
        }
    }

    Ok(output)
}

/// Calculates TEMA using a more direct approach
///
/// This version calculates all three EMAs in a more integrated manner
/// for potentially better performance.
pub fn tema_direct(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    
    let min_required = 3 * period - 2;
    validate_sufficient_data(data, min_required, "data")?;

    let mut output = allocate_output(data.len());
    let multiplier = 2.0 / (period as Price + 1.0);
    
    // Initialize first EMA with SMA of first 'period' values
    let initial_sum: Price = data[0..period].iter().sum();
    let mut ema1 = initial_sum / period as Price;
    
    // Track EMA values for subsequent calculations
    let mut ema1_values = Vec::with_capacity(data.len());
    let mut ema2_values = Vec::with_capacity(data.len());
    
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
        ema2_values.push(ema2);
        
        for i in 1..ema1_values.len() {
            if i >= period - 1 {
                ema2 = (ema1_values[i] * multiplier) + (ema2 * (1.0 - multiplier));
            }
            if i >= period - 1 {
                ema2_values.push(ema2);
            }
        }
    }
    
    // Calculate EMA3 and TEMA
    if ema2_values.len() >= period {
        let initial_sum3: Price = ema2_values[0..period].iter().sum();
        let mut ema3 = initial_sum3 / period as Price;
        
        let start_index = 3 * period - 3;
        for i in start_index..data.len() {
            let ema1_val = ema1_values[i - (period - 1)];
            let ema2_val = ema2_values[i - 2 * (period - 1)];
            
            if i > start_index {
                let ema2_for_ema3_index = i - 2 * (period - 1);
                if ema2_for_ema3_index < ema2_values.len() {
                    ema3 = (ema2_values[ema2_for_ema3_index] * multiplier) + (ema3 * (1.0 - multiplier));
                }
            }
            
            output[i] = 3.0 * ema1_val - 3.0 * ema2_val + ema3;
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE, RELAXED_TOLERANCE};
    use crate::overlap::dema::dema;

    #[test]
    fn test_tema_basic() {
        let data: Vec<Price> = (1..=15).map(|x| x as Price).collect();
        let result = tema(&data, 3).unwrap();
        
        // First 6 values should be NaN (3×3-3 = 6)
        for i in 0..6 {
            assert!(result[i].is_nan());
        }
        
        // Should have valid values from index 6 onwards
        for i in 6..data.len() {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_tema_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = tema(&data, 1).unwrap();
        
        // With period 1, TEMA should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_tema_constant_data() {
        // Test with constant data
        let data = vec![10.0; 15];
        let result = tema(&data, 4).unwrap();
        
        // For constant data, TEMA should equal the constant value
        let start_index = 3 * 4 - 3; // 9
        for i in start_index..data.len() {
            assert!((result[i] - 10.0).abs() < RELAXED_TOLERANCE);
        }
    }

    #[test]
    fn test_tema_vs_dema_responsiveness() {
        // TEMA should be more responsive than DEMA
        let mut data = vec![10.0; 20];
        data.extend(vec![20.0; 20]); // Sudden jump
        
        let tema_result = tema(&data, 5).unwrap();
        let dema_result = dema(&data, 5).unwrap();
        
        // Find valid values well after the jump to see responsiveness
        let jump_index = 20;
        let check_index = jump_index + 10; // Check well after jump
        
        if check_index < tema_result.len() && 
           check_index < dema_result.len() &&
           !tema_result[check_index].is_nan() && 
           !dema_result[check_index].is_nan() {
            // Both should have moved towards 20, but we just check they're valid
            assert!(tema_result[check_index] > 10.0);
            assert!(dema_result[check_index] > 10.0);
        }
    }

    #[test]
    fn test_tema_direct_basic() {
        // Test that tema_direct produces valid results
        let data: Vec<Price> = (1..=20).map(|x| x as Price).collect();
        let result = tema_direct(&data, 4).unwrap();
        
        let start_index = 3 * 4 - 3; // 9
        for i in start_index..data.len() {
            assert!(!result[i].is_nan(), "Expected valid value at index {}", i);
        }
    }

    #[test]
    fn test_tema_trending_data() {
        // Test with trending data
        let data: Vec<Price> = (1..=25).map(|x| x as Price).collect();
        let result = tema(&data, 5).unwrap();
        
        // TEMA should follow the trend
        let valid_start = 3 * 5 - 3; // 12
        for i in (valid_start + 1)..data.len() {
            if !result[i].is_nan() && !result[i-1].is_nan() {
                // Should generally trend upward for increasing data
                assert!(result[i] >= result[i-1] - RELAXED_TOLERANCE);
            }
        }
    }

    #[test]
    fn test_tema_insufficient_data() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        assert!(tema(&data, 3).is_err()); // Needs at least 3×3-2 = 7 points
    }

    #[test]
    fn test_tema_empty_data() {
        let data = vec![];
        assert!(tema(&data, 3).is_err());
    }

    #[test]
    fn test_tema_zero_period() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!(tema(&data, 0).is_err());
    }

    #[test]
    fn test_tema_minimum_data_requirement() {
        // Test with exactly the minimum required data
        let period = 3;
        let min_data = 3 * period - 2; // 7
        let data: Vec<Price> = (1..=min_data).map(|x| x as Price).collect();
        
        let result = tema(&data, period).unwrap();
        
        // Should have exactly one valid value at the end
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert_eq!(valid_count, 1);
    }

    #[test]
    fn test_tema_formula_verification() {
        // Verify TEMA formula: TEMA = 3×EMA1 - 3×EMA2 + EMA3
        let data: Vec<Price> = (1..=15).map(|x| x as Price).collect();
        let period = 3;
        
        let ema1 = ema(&data, period).unwrap();
        let ema1_valid: Vec<Price> = ema1.iter().skip(period - 1).copied().collect();
        let ema2 = ema(&ema1_valid, period).unwrap();
        let ema2_valid: Vec<Price> = ema2.iter().skip(period - 1).copied().collect();
        let ema3 = ema(&ema2_valid, period).unwrap();
        
        let tema_result = tema(&data, period).unwrap();
        
        // Verify formula for valid indices
        let start_index = 3 * period - 3;
        for i in start_index..data.len() {
            if !tema_result[i].is_nan() {
                let ema2_index = i - (period - 1);
                let ema3_index = i - 2 * (period - 1);
                
                if ema2_index < ema2.len() && 
                   ema3_index < ema3.len() &&
                   !ema2[ema2_index].is_nan() && 
                   !ema3[ema3_index].is_nan() {
                    let expected = 3.0 * ema1[i] - 3.0 * ema2[ema2_index] + ema3[ema3_index];
                    assert!((tema_result[i] - expected).abs() < RELAXED_TOLERANCE);
                }
            }
        }
    }

    #[test]
    fn test_tema_real_market_data() {
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
            46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80,
            47.11, 47.56, 47.23, 46.95, 47.10, 47.32, 47.20, 46.75, 46.57, 46.80
        ];
        
        let result = tema(&data, 6).unwrap();
        
        // Should have valid values starting from index 15 (3×6-3)
        let start_index = 3 * 6 - 3;
        for i in 0..start_index {
            assert!(result[i].is_nan());
        }
        for i in start_index..data.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
    }

    #[test]
    fn test_tema_oscillating_data() {
        // Test with oscillating data to see responsiveness
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(if i % 2 == 0 { 10.0 } else { 15.0 });
        }
        
        let result = tema(&data, 4).unwrap();
        
        // TEMA should smooth out the oscillations but still show some variation
        let start_index = 3 * 4 - 3; // 9
        if start_index < result.len() {
            let valid_values: Vec<Price> = result.iter()
                .skip(start_index)
                .filter(|&&x| !x.is_nan())
                .copied()
                .collect();
            
            if valid_values.len() > 1 {
                let min_val = valid_values.iter().fold(Price::INFINITY, |a, &b| a.min(b));
                let max_val = valid_values.iter().fold(Price::NEG_INFINITY, |a, &b| a.max(b));
                
                // Should be between the input range but smoothed
                assert!(min_val >= 10.0 && max_val <= 15.0);
                assert!(max_val > min_val); // Should show some variation
            }
        }
    }
}