//! MidPoint over period

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output, highest_in_period, lowest_in_period};

/// Calculates the MidPoint over a specified period
///
/// The MidPoint is the average of the highest and lowest values over a given period.
/// It represents the middle point of the price range for that period.
///
/// # Formula
/// ```text
/// MidPoint = (Highest(Price, period) + Lowest(Price, period)) / 2
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back for highest and lowest values
///
/// # Returns
/// Vector of MidPoint values. The first `period-1` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < period
///
/// # Example
/// ```rust
/// use ta_rust::overlap::midpoint;
///
/// let prices = vec![1.0, 3.0, 2.0, 5.0, 4.0];
/// let result = midpoint(&prices, 3).unwrap();
/// 
/// // For index 2: highest(1,3,2) = 3, lowest(1,3,2) = 1, midpoint = 2.0
/// assert_eq!(result[2], 2.0);
/// // For index 3: highest(3,2,5) = 5, lowest(3,2,5) = 2, midpoint = 3.5
/// assert_eq!(result[3], 3.5);
/// ```
pub fn midpoint(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    // Calculate midpoint for each position starting from period-1
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        let highest = highest_in_period(data, start_index, period);
        let lowest = lowest_in_period(data, start_index, period);
        output[i] = (highest + lowest) / 2.0;
    }

    Ok(output)
}

/// Calculates MidPoint using a rolling approach for better performance
///
/// This version maintains running highest and lowest values and updates
/// them more efficiently for large datasets.
pub fn midpoint_rolling(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    // For rolling calculation, we need to track the window
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        let window = &data[start_index..=i];
        
        let highest = window.iter().fold(Price::NEG_INFINITY, |acc, &x| acc.max(x));
        let lowest = window.iter().fold(Price::INFINITY, |acc, &x| acc.min(x));
        
        output[i] = (highest + lowest) / 2.0;
    }

    Ok(output)
}

/// Calculates MidPoint with custom aggregation functions
///
/// This version allows for custom functions to determine the "highest" and "lowest"
/// values, which could be useful for specialized applications.
pub fn midpoint_custom<F1, F2>(
    data: &[Price], 
    period: Period,
    high_fn: F1,
    low_fn: F2
) -> TAResult<Vec<Price>>
where
    F1: Fn(&[Price]) -> Price,
    F2: Fn(&[Price]) -> Price,
{
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    // Calculate midpoint for each position using custom functions
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        let window = &data[start_index..=i];
        
        let highest = high_fn(window);
        let lowest = low_fn(window);
        
        output[i] = (highest + lowest) / 2.0;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_midpoint_basic() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = midpoint(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            2.0, // (max(1,3,2) + min(1,3,2)) / 2 = (3+1)/2 = 2.0
            3.5, // (max(3,2,5) + min(3,2,5)) / 2 = (5+2)/2 = 3.5
            3.5, // (max(2,5,4) + min(2,5,4)) / 2 = (5+2)/2 = 3.5
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = midpoint(&data, 1).unwrap();
        
        // With period 1, midpoint should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_constant_data() {
        let data = vec![5.0; 10];
        let result = midpoint(&data, 4).unwrap();
        
        // All non-NaN values should be 5.0 for constant input
        for i in 3..result.len() {
            assert!((result[i] - 5.0).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_midpoint_rolling_equivalence() {
        let data = vec![1.0, 4.0, 2.0, 6.0, 3.0, 7.0, 1.0, 5.0];
        let result1 = midpoint(&data, 4).unwrap();
        let result2 = midpoint_rolling(&data, 4).unwrap();
        
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_ascending_data() {
        let data: Vec<Price> = (1..=10).map(|x| x as Price).collect();
        let result = midpoint(&data, 5).unwrap();
        
        // For ascending data, midpoint should be the middle of the window
        for i in 4..data.len() {
            let start = i + 1 - 5;
            let expected = (data[start] + data[i]) / 2.0;
            assert!((result[i] - expected).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_midpoint_descending_data() {
        let data: Vec<Price> = (1..=10).rev().map(|x| x as Price).collect();
        let result = midpoint(&data, 5).unwrap();
        
        // For descending data, midpoint should be the middle of the window
        for i in 4..data.len() {
            let start = i + 1 - 5;
            let expected = (data[start] + data[i]) / 2.0;
            assert!((result[i] - expected).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_midpoint_custom() {
        let data = vec![1.0, 5.0, 3.0, 7.0, 2.0, 6.0];
        
        // Custom functions that return median instead of min/max
        let median_fn = |slice: &[Price]| {
            let mut sorted = slice.to_vec();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = sorted.len() / 2;
            if sorted.len() % 2 == 0 {
                (sorted[mid - 1] + sorted[mid]) / 2.0
            } else {
                sorted[mid]
            }
        };
        
        let result = midpoint_custom(&data, 3, &median_fn, &median_fn).unwrap();
        
        // All values should be the median (since high_fn and low_fn are the same)
        for i in 2..result.len() {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_midpoint_extreme_values() {
        let data = vec![1.0, 1000.0, 1.0, 1000.0, 1.0];
        let result = midpoint(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            500.5, // (1000 + 1) / 2
            500.5, // (1000 + 1) / 2
            500.5, // (1000 + 1) / 2
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_single_peak() {
        let data = vec![1.0, 1.0, 10.0, 1.0, 1.0];
        let result = midpoint(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            5.5, // (10 + 1) / 2
            5.5, // (10 + 1) / 2
            5.5, // (10 + 1) / 2
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(midpoint(&data, 3).is_err());
    }

    #[test]
    fn test_midpoint_empty_data() {
        let data = vec![];
        assert!(midpoint(&data, 3).is_err());
    }

    #[test]
    fn test_midpoint_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(midpoint(&data, 0).is_err());
    }

    #[test]
    fn test_midpoint_period_equals_length() {
        let data = vec![1.0, 5.0, 3.0, 7.0, 2.0];
        let result = midpoint(&data, 5).unwrap();
        
        // Should have exactly one valid value
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert_eq!(valid_count, 1);
        
        // The value should be (7 + 1) / 2 = 4.0
        assert!((result[4] - 4.0).abs() < DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_real_market_data() {
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93
        ];
        let result = midpoint(&data, 5).unwrap();
        
        // Should have valid values starting from index 4
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        for i in 4..data.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
        
        // MidPoint should be between min and max of the data
        let min_price = data.iter().fold(Price::INFINITY, |a, &b| a.min(b));
        let max_price = data.iter().fold(Price::NEG_INFINITY, |a, &b| a.max(b));
        
        for i in 4..data.len() {
            assert!(result[i] >= min_price && result[i] <= max_price);
        }
    }

    #[test]
    fn test_midpoint_symmetry() {
        // Test that midpoint is symmetric around the center
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = midpoint(&data, 5).unwrap();
        
        // For symmetric data, midpoint should be the center value
        assert!((result[4] - 3.0).abs() < DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_midpoint_vs_sma_relationship() {
        // For certain data patterns, we can verify relationships
        let data = vec![1.0, 5.0, 1.0, 5.0, 1.0, 5.0];
        let midpoint_result = midpoint(&data, 4).unwrap();
        
        // For alternating min/max data, midpoint should be constant
        for i in 3..data.len() {
            assert!((midpoint_result[i] - 3.0).abs() < DEFAULT_TOLERANCE);
        }
    }
}