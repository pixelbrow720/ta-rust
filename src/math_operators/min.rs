//! Minimum value over a specified period

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output, lowest_in_period, lowest_index, highest_in_period, highest_index};

/// Finds the lowest value over a specified period
///
/// # Formula
/// ```text
/// MIN[i] = min(data[i-period+1], data[i-period+2], ..., data[i])
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back for minimum value
///
/// # Returns
/// Vector of minimum values. The first `period-1` values will be NaN.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::min;
///
/// let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
/// let result = min(&data, 3).unwrap();
/// // result[2] = min(3,1,4) = 1.0
/// // result[3] = min(1,4,2) = 1.0
/// ```
pub fn min(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        output[i] = lowest_in_period(data, start_index, period);
    }

    Ok(output)
}

/// Finds the index of the lowest value over a specified period
///
/// Returns the absolute index (not relative to the window) of the minimum value.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back for minimum value
///
/// # Returns
/// Vector of indices where minimum values occur. The first `period-1` values will be 0.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::minindex;
///
/// let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
/// let result = minindex(&data, 3).unwrap();
/// // result[2] = 1 (index of value 1.0)
/// // result[3] = 1 (index of value 1.0)
/// ```
pub fn minindex(data: &[Price], period: Period) -> TAResult<Vec<usize>> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = vec![0; data.len()];
    
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        let window = &data[start_index..=i];
        let relative_index = lowest_index(window);
        output[i] = start_index + relative_index;
    }

    Ok(output)
}

/// Finds both the lowest and highest values over a specified period
///
/// This is more efficient than calling min() and max() separately.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back
///
/// # Returns
/// Tuple of (min_values, max_values) vectors. The first `period-1` values will be NaN.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::minmax;
///
/// let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
/// let (min_result, max_result) = minmax(&data, 3).unwrap();
/// // min_result[2] = 1.0, max_result[2] = 4.0
/// ```
pub fn minmax(data: &[Price], period: Period) -> TAResult<(Vec<Price>, Vec<Price>)> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut min_output = allocate_output(data.len());
    let mut max_output = allocate_output(data.len());
    
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        min_output[i] = lowest_in_period(data, start_index, period);
        max_output[i] = highest_in_period(data, start_index, period);
    }

    Ok((min_output, max_output))
}

/// Finds the indexes of both the lowest and highest values over a specified period
///
/// This is more efficient than calling minindex() and maxindex() separately.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back
///
/// # Returns
/// Tuple of (min_indices, max_indices) vectors. The first `period-1` values will be 0.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::minmaxindex;
///
/// let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
/// let (min_indices, max_indices) = minmaxindex(&data, 3).unwrap();
/// // min_indices[2] = 1, max_indices[2] = 2
/// ```
pub fn minmaxindex(data: &[Price], period: Period) -> TAResult<(Vec<usize>, Vec<usize>)> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut min_output = vec![0; data.len()];
    let mut max_output = vec![0; data.len()];
    
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        let window = &data[start_index..=i];
        
        let min_relative_index = lowest_index(window);
        let max_relative_index = highest_index(window);
        
        min_output[i] = start_index + min_relative_index;
        max_output[i] = start_index + max_relative_index;
    }

    Ok((min_output, max_output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_min_basic() {
        let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let result = min(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            1.0, // min(3,1,4)
            1.0, // min(1,4,2)
            2.0, // min(4,2,5)
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_minindex_basic() {
        let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let result = minindex(&data, 3).unwrap();
        
        let expected = vec![0, 0, 1, 1, 3]; // indices of min values
        assert_eq!(result, expected);
    }

    #[test]
    fn test_minmax_basic() {
        let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let (min_result, max_result) = minmax(&data, 3).unwrap();
        
        let expected_min = vec![
            Price::NAN, Price::NAN,
            1.0, // min(3,1,4)
            1.0, // min(1,4,2)
            2.0, // min(4,2,5)
        ];
        
        let expected_max = vec![
            Price::NAN, Price::NAN,
            4.0, // max(3,1,4)
            4.0, // max(1,4,2)
            5.0, // max(4,2,5)
        ];
        
        assert_arrays_approx_equal(&min_result, &expected_min, DEFAULT_TOLERANCE);
        assert_arrays_approx_equal(&max_result, &expected_max, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_minmaxindex_basic() {
        let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let (min_indices, max_indices) = minmaxindex(&data, 3).unwrap();
        
        let expected_min = vec![0, 0, 1, 1, 3]; // indices of min values
        let expected_max = vec![0, 0, 2, 2, 4]; // indices of max values
        
        assert_eq!(min_indices, expected_min);
        assert_eq!(max_indices, expected_max);
    }

    #[test]
    fn test_min_period_1() {
        let data = vec![3.0, 1.0, 4.0, 2.0, 5.0];
        let result = min(&data, 1).unwrap();
        
        // With period 1, min should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_min_constant_data() {
        let data = vec![5.0; 10];
        let result = min(&data, 4).unwrap();
        
        // All non-NaN values should be 5.0
        for i in 3..result.len() {
            assert!((result[i] - 5.0).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_minmax_efficiency() {
        // Test that minmax produces same results as separate min/max calls
        let data = vec![3.0, 1.0, 4.0, 2.0, 5.0, 0.5, 6.0];
        let period = 4;
        
        let min_separate = min(&data, period).unwrap();
        let max_separate = super::super::max::max(&data, period).unwrap();
        let (min_combined, max_combined) = minmax(&data, period).unwrap();
        
        assert_arrays_approx_equal(&min_separate, &min_combined, DEFAULT_TOLERANCE);
        assert_arrays_approx_equal(&max_separate, &max_combined, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_min_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(min(&data, 3).is_err());
    }

    #[test]
    fn test_min_empty_data() {
        let data = vec![];
        assert!(min(&data, 3).is_err());
    }

    #[test]
    fn test_min_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(min(&data, 0).is_err());
    }

    #[test]
    fn test_minmax_edge_cases() {
        // Test with duplicate values
        let data = vec![2.0, 2.0, 2.0, 1.0, 1.0];
        let (min_result, max_result) = minmax(&data, 3).unwrap();
        
        // Should handle duplicates correctly
        assert!(!min_result[2].is_nan());
        assert!(!max_result[2].is_nan());
        assert_eq!(min_result[2], 2.0);
        assert_eq!(max_result[2], 2.0);
    }
}