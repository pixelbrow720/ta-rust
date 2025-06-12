//! Triangular Moving Average (TRIMA)

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output};
use crate::overlap::sma::sma;

/// Calculates the Triangular Moving Average (TRIMA)
///
/// The Triangular Moving Average is a double-smoothed moving average that gives
/// more weight to the middle portion of the data series. It's calculated by
/// applying a Simple Moving Average to another Simple Moving Average.
///
/// # Formula
/// ```text
/// If period is odd:
///     TRIMA = SMA(SMA(Price, (period+1)/2), (period+1)/2)
/// If period is even:
///     TRIMA = SMA(SMA(Price, period/2+1), period/2)
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Vector of TRIMA values. The first `period-1` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < period
///
/// # Example
/// ```rust
/// use ta_rust::overlap::trima;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let result = trima(&prices, 5).unwrap();
/// 
/// // TRIMA will have valid values starting from index 4
/// assert!(result[3].is_nan());
/// assert!(!result[4].is_nan());
/// ```
pub fn trima(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    // Calculate the periods for the two SMAs
    let (first_period, second_period) = if period % 2 == 1 {
        // Odd period
        let half_period = (period + 1) / 2;
        (half_period, half_period)
    } else {
        // Even period
        let first = period / 2 + 1;
        let second = period / 2;
        (first, second)
    };

    // Calculate first SMA
    let first_sma = sma(data, first_period)?;
    
    // Extract non-NaN values from first SMA
    let first_sma_valid: Vec<Price> = first_sma.iter()
        .skip(first_period - 1)
        .copied()
        .collect();
    
    // Calculate second SMA
    let second_sma = sma(&first_sma_valid, second_period)?;
    
    // Combine results
    let mut output = allocate_output(data.len());
    
    // Map second SMA results back to original indices
    let start_index = first_period + second_period - 2;
    for i in start_index..data.len() {
        let second_sma_index = i - (first_period - 1);
        if second_sma_index < second_sma.len() && !second_sma[second_sma_index].is_nan() {
            output[i] = second_sma[second_sma_index];
        }
    }

    Ok(output)
}

/// Calculates TRIMA using a direct calculation approach
///
/// This version calculates the triangular weights directly without
/// using intermediate SMA calculations.
pub fn trima_direct(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    // Calculate triangular weights
    let mut weights = Vec::with_capacity(period);
    let half_period = (period + 1) / 2;
    
    for i in 0..period {
        let weight = if i < half_period {
            (i + 1) as Price
        } else {
            (period - i) as Price
        };
        weights.push(weight);
    }
    
    let weight_sum: Price = weights.iter().sum();
    
    // Calculate TRIMA for each position
    for i in (period - 1)..data.len() {
        let mut weighted_sum = 0.0;
        
        for j in 0..period {
            let price_index = i + 1 - period + j;
            weighted_sum += data[price_index] * weights[j];
        }
        
        output[i] = weighted_sum / weight_sum;
    }

    Ok(output)
}

/// Calculates TRIMA with custom triangular weighting
///
/// This version allows for custom triangular weight distribution.
pub fn trima_custom_peak(data: &[Price], period: Period, peak_position: Price) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;
    
    if peak_position < 0.0 || peak_position > 1.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "peak_position",
            "must be between 0.0 and 1.0"
        ));
    }

    let mut output = allocate_output(data.len());
    
    // Calculate custom triangular weights
    let mut weights = Vec::with_capacity(period);
    let peak_index = (peak_position * (period - 1) as Price).round() as usize;
    
    for i in 0..period {
        let weight = if i <= peak_index {
            if peak_index == 0 {
                1.0
            } else {
                (i as Price / peak_index as Price) + 1.0
            }
        } else {
            if peak_index == period - 1 {
                1.0
            } else {
                ((period - 1 - i) as Price / (period - 1 - peak_index) as Price) + 1.0
            }
        };
        weights.push(weight);
    }
    
    let weight_sum: Price = weights.iter().sum();
    
    // Calculate TRIMA for each position
    for i in (period - 1)..data.len() {
        let mut weighted_sum = 0.0;
        
        for j in 0..period {
            let price_index = i + 1 - period + j;
            weighted_sum += data[price_index] * weights[j];
        }
        
        output[i] = weighted_sum / weight_sum;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE, RELAXED_TOLERANCE};

    #[test]
    fn test_trima_basic_odd_period() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = trima(&data, 5).unwrap();
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        
        // Should have valid values from index 4 onwards
        for i in 4..data.len() {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_trima_basic_even_period() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = trima(&data, 6).unwrap();
        
        // First 5 values should be NaN (first_period + second_period - 2 = 4 + 3 - 2 = 5)
        for i in 0..5 {
            assert!(result[i].is_nan());
        }
        
        // Should have valid values from index 5 onwards
        for i in 5..data.len() {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_trima_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = trima(&data, 1).unwrap();
        
        // With period 1, TRIMA should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_trima_period_2() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = trima(&data, 2).unwrap();
        
        // For period 2: first_period = 2, second_period = 1
        // Should be equivalent to SMA(2)
        let sma_result = sma(&data, 2).unwrap();
        assert_arrays_approx_equal(&result, &sma_result, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_trima_direct_equivalence() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result1 = trima(&data, 5).unwrap();
        let result2 = trima_direct(&data, 5).unwrap();
        
        // Both methods should produce similar results
        assert_arrays_approx_equal(&result1, &result2, RELAXED_TOLERANCE);
    }

    #[test]
    fn test_trima_constant_data() {
        let data = vec![7.5; 10];
        let result = trima(&data, 5).unwrap();
        
        // All non-NaN values should be 7.5 for constant input
        for i in 4..result.len() {
            assert!((result[i] - 7.5).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_trima_triangular_weights() {
        // Test that triangular weights are calculated correctly
        let period = 5;
        let _expected_weights = vec![1.0, 2.0, 3.0, 2.0, 1.0];
        
        // Verify by checking the direct calculation
        let data = vec![1.0, 0.0, 0.0, 0.0, 0.0]; // Only first element is 1
        let result = trima_direct(&data, period).unwrap();
        
        // The result should be 1.0 * weight[0] / sum(weights) = 1.0 / 9.0
        let expected = 1.0 / 9.0;
        assert!((result[4] - expected).abs() < DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_trima_custom_peak() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        
        // Test with peak at center (0.5)
        let result_center = trima_custom_peak(&data, 5, 0.5).unwrap();
        let result_normal = trima(&data, 5).unwrap();
        
        // Should be similar to normal TRIMA
        assert_arrays_approx_equal(&result_center, &result_normal, RELAXED_TOLERANCE);
        
        // Test with peak at beginning (0.0)
        let result_begin = trima_custom_peak(&data, 5, 0.0).unwrap();
        assert!(!result_begin[4].is_nan());
        
        // Test with peak at end (1.0)
        let result_end = trima_custom_peak(&data, 5, 1.0).unwrap();
        assert!(!result_end[4].is_nan());
    }

    #[test]
    fn test_trima_smoothing_property() {
        // TRIMA should be smoother than SMA - test with constant data
        let data = vec![10.0; 15];
        
        let trima_result = trima(&data, 5).unwrap();
        let sma_result = sma(&data, 5).unwrap();
        
        // For constant data, both should be constant after initial period
        let start_trima = if 5 % 2 == 1 {
            let half = (5 + 1) / 2;
            half + half - 2
        } else {
            let first = 5 / 2 + 1;
            let second = 5 / 2;
            first + second - 2
        };
        
        for i in start_trima..trima_result.len() {
            assert!((trima_result[i] - 10.0).abs() < DEFAULT_TOLERANCE);
        }
        
        for i in 4..sma_result.len() {
            assert!((sma_result[i] - 10.0).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_trima_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(trima(&data, 5).is_err());
    }

    #[test]
    fn test_trima_empty_data() {
        let data = vec![];
        assert!(trima(&data, 3).is_err());
    }

    #[test]
    fn test_trima_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(trima(&data, 0).is_err());
    }

    #[test]
    fn test_trima_custom_peak_invalid() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        // Invalid peak positions
        assert!(trima_custom_peak(&data, 3, -0.1).is_err());
        assert!(trima_custom_peak(&data, 3, 1.1).is_err());
    }

    #[test]
    fn test_trima_period_calculation() {
        // Test period calculations for odd and even periods
        
        // Odd period (5): first_period = 3, second_period = 3
        let data = vec![1.0; 10];
        let result_odd = trima(&data, 5).unwrap();
        let start_odd = 3 + 3 - 2; // 4
        assert!(result_odd[start_odd - 1].is_nan());
        assert!(!result_odd[start_odd].is_nan());
        
        // Even period (6): first_period = 4, second_period = 3
        let result_even = trima(&data, 6).unwrap();
        let start_even = 4 + 3 - 2; // 5
        assert!(result_even[start_even - 1].is_nan());
        assert!(!result_even[start_even].is_nan());
    }

    #[test]
    fn test_trima_real_market_data() {
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
            46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80
        ];
        
        let result = trima(&data, 7).unwrap();
        
        // Should have valid values starting from appropriate index
        let start_index = if 7 % 2 == 1 {
            let half = (7 + 1) / 2; // 4
            half + half - 2 // 6
        } else {
            let first = 7 / 2 + 1; // 4
            let second = 7 / 2; // 3
            first + second - 2 // 5
        };
        
        for i in 0..start_index {
            assert!(result[i].is_nan());
        }
        for i in start_index..data.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
        
        // TRIMA should be between min and max of the data
        let min_price = data.iter().fold(Price::INFINITY, |a, &b| a.min(b));
        let max_price = data.iter().fold(Price::NEG_INFINITY, |a, &b| a.max(b));
        
        for i in start_index..data.len() {
            assert!(result[i] >= min_price && result[i] <= max_price);
        }
    }
}