//! Simple Moving Average (SMA)

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output};

/// Calculates the Simple Moving Average (SMA)
///
/// The Simple Moving Average is the arithmetic mean of a given set of prices
/// over a specific number of periods.
///
/// # Formula
/// ```text
/// SMA = (P1 + P2 + ... + Pn) / n
/// ```
/// Where:
/// - P1, P2, ..., Pn are the prices for the last n periods
/// - n is the period
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Vector of SMA values. The first `period-1` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < period
///
/// # Example
/// ```rust
/// use ta_rust::overlap::sma;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = sma(&prices, 3).unwrap();
/// 
/// // First 2 values are NaN, then [2.0, 3.0, 4.0]
/// assert!(result[0].is_nan());
/// assert!(result[1].is_nan());
/// assert_eq!(result[2], 2.0); // (1+2+3)/3
/// assert_eq!(result[3], 3.0); // (2+3+4)/3
/// assert_eq!(result[4], 4.0); // (3+4+5)/3
/// ```
pub fn sma(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    
    // Calculate SMA for each position starting from period-1
    for i in (period - 1)..data.len() {
        let sum: Price = data[(i + 1 - period)..=i].iter().sum();
        output[i] = sum / period as Price;
    }

    Ok(output)
}

/// Calculates SMA with a rolling window approach (more memory efficient for large datasets)
///
/// This version maintains a running sum and updates it incrementally,
/// which can be more efficient for very large datasets.
pub fn sma_rolling(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    let mut sum = 0.0;
    
    // Initialize sum for first period
    for i in 0..period {
        sum += data[i];
    }
    output[period - 1] = sum / period as Price;
    
    // Rolling calculation for remaining values
    for i in period..data.len() {
        sum = sum - data[i - period] + data[i];
        output[i] = sum / period as Price;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_sma_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 3).unwrap();
        let expected = vec![Price::NAN, Price::NAN, 2.0, 3.0, 4.0];
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sma_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 1).unwrap();
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sma_period_equals_length() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 5).unwrap();
        let expected = vec![Price::NAN, Price::NAN, Price::NAN, Price::NAN, 3.0];
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sma_rolling_equivalence() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result1 = sma(&data, 4).unwrap();
        let result2 = sma_rolling(&data, 4).unwrap();
        assert_arrays_approx_equal(&result1, &result2, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_sma_real_data() {
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93
        ];
        let result = sma(&data, 5).unwrap();
        
        // Check that we get valid results
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        for i in 4..result.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 40.0 && result[i] < 50.0);
        }
        
        // Test specific calculation for index 4
        let expected_4 = (44.34 + 44.09 + 44.15 + 43.61 + 44.33) / 5.0;
        assert!((result[4] - expected_4).abs() < 1e-10);
    }

    #[test]
    fn test_sma_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(sma(&data, 3).is_err());
    }

    #[test]
    fn test_sma_empty_data() {
        let data = vec![];
        assert!(sma(&data, 3).is_err());
    }

    #[test]
    fn test_sma_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(sma(&data, 0).is_err());
    }

    #[test]
    fn test_sma_large_dataset() {
        let data: Vec<Price> = (1..=1000).map(|x| x as Price).collect();
        let result = sma(&data, 50).unwrap();
        
        // Check that we get the right number of valid values
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert_eq!(valid_count, 1000 - 50 + 1);
        
        // Check a specific value
        let expected_at_50 = (1.0 + 50.0) / 2.0; // Average of 1..50
        assert!((result[49] - expected_at_50).abs() < 1e-10);
    }

    #[test]
    fn test_sma_constant_values() {
        let data = vec![5.0; 10];
        let result = sma(&data, 3).unwrap();
        
        // All non-NaN values should be 5.0
        for i in 2..result.len() {
            assert!((result[i] - 5.0).abs() < 1e-10);
        }
    }
}