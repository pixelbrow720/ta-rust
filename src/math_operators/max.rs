//! Maximum value over a specified period

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output, highest_in_period, highest_index};

/// Finds the highest value over a specified period
///
/// # Formula
/// ```text
/// MAX[i] = max(data[i-period+1], data[i-period+2], ..., data[i])
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back for maximum value
///
/// # Returns
/// Vector of maximum values. The first `period-1` values will be NaN.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::max;
///
/// let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
/// let result = max(&data, 3).unwrap();
/// // result[2] = max(1,3,2) = 3.0
/// // result[3] = max(3,2,5) = 5.0
/// ```
pub fn max(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        output[i] = highest_in_period(data, start_index, period);
    }

    Ok(output)
}

/// Finds the index of the highest value over a specified period
///
/// Returns the absolute index (not relative to the window) of the maximum value.
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to look back for maximum value
///
/// # Returns
/// Vector of indices where maximum values occur. The first `period-1` values will be 0.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::maxindex;
///
/// let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
/// let result = maxindex(&data, 3).unwrap();
/// // result[2] = 1 (index of value 3.0)
/// // result[3] = 3 (index of value 5.0)
/// ```
pub fn maxindex(data: &[Price], period: Period) -> TAResult<Vec<usize>> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = vec![0; data.len()];
    
    for i in (period - 1)..data.len() {
        let start_index = i + 1 - period;
        let window = &data[start_index..=i];
        let relative_index = highest_index(window);
        output[i] = start_index + relative_index;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_max_basic() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = max(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            3.0, // max(1,3,2)
            5.0, // max(3,2,5)
            5.0, // max(2,5,4)
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_maxindex_basic() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = maxindex(&data, 3).unwrap();
        
        let expected = vec![0, 0, 1, 3, 3]; // indices of max values
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_period_1() {
        let data = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        let result = max(&data, 1).unwrap();
        
        // With period 1, max should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_max_constant_data() {
        let data = vec![5.0; 10];
        let result = max(&data, 4).unwrap();
        
        // All non-NaN values should be 5.0
        for i in 3..result.len() {
            assert!((result[i] - 5.0).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_max_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(max(&data, 3).is_err());
    }

    #[test]
    fn test_max_empty_data() {
        let data = vec![];
        assert!(max(&data, 3).is_err());
    }

    #[test]
    fn test_max_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(max(&data, 0).is_err());
    }
}