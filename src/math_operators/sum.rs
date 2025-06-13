//! Summation over a specified period

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output};

/// Calculates the sum over a specified period
///
/// # Formula
/// ```text
/// SUM[i] = data[i-period+1] + data[i-period+2] + ... + data[i]
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods to sum over
///
/// # Returns
/// Vector of sum values. The first `period-1` values will be NaN.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::sum;
///
/// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = sum(&data, 3).unwrap();
/// // result[2] = 1+2+3 = 6.0
/// // result[3] = 2+3+4 = 9.0
/// ```
pub fn sum(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    let mut rolling_sum = 0.0;
    
    // Initialize sum for first period
    for i in 0..period {
        rolling_sum += data[i];
    }
    output[period - 1] = rolling_sum;
    
    // Rolling calculation for remaining values
    for i in period..data.len() {
        rolling_sum = rolling_sum - data[i - period] + data[i];
        output[i] = rolling_sum;
    }

    Ok(output)
}

/// Calculates the sum using a rolling approach for better performance
///
/// This version maintains a running sum and updates it incrementally.
pub fn sum_rolling(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    let mut rolling_sum = 0.0;
    
    // Initialize sum for first period
    for i in 0..period {
        rolling_sum += data[i];
    }
    output[period - 1] = rolling_sum;
    
    // Rolling calculation for remaining values
    for i in period..data.len() {
        rolling_sum = rolling_sum - data[i - period] + data[i];
        output[i] = rolling_sum;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_sum_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sum(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            6.0,  // 1+2+3
            9.0,  // 2+3+4
            12.0, // 3+4+5
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sum_rolling_equivalence() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result1 = sum(&data, 4).unwrap();
        let result2 = sum_rolling(&data, 4).unwrap();
        
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sum_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sum(&data, 1).unwrap();
        
        // With period 1, sum should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sum_vs_sma_relationship() {
        // SUM = SMA * period
        let data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let period = 3;
        
        let sum_result = sum(&data, period).unwrap();
        let sma_result = crate::overlap::sma::sma(&data, period).unwrap();
        
        for i in (period - 1)..data.len() {
            let expected_sum = sma_result[i] * period as Price;
            assert!((sum_result[i] - expected_sum).abs() < DEFAULT_TOLERANCE);
        }
    }

    #[test]
    fn test_sum_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(sum(&data, 3).is_err());
    }

    #[test]
    fn test_sum_empty_data() {
        let data = vec![];
        assert!(sum(&data, 3).is_err());
    }

    #[test]
    fn test_sum_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(sum(&data, 0).is_err());
    }
}
