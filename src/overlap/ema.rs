//! Exponential Moving Average (EMA)

use crate::common::{TAResult, Price, Period};
use crate::common::utils::{validate_not_empty, validate_period, validate_sufficient_data, allocate_output, ema_multiplier};

/// Calculates the Exponential Moving Average (EMA)
///
/// The Exponential Moving Average gives more weight to recent prices,
/// making it more responsive to new information than the Simple Moving Average.
///
/// # Formula
/// ```text
/// Multiplier (α) = 2 / (period + 1)
/// EMA[today] = (Price[today] × α) + (EMA[yesterday] × (1 - α))
/// 
/// Initial EMA = SMA of first 'period' values
/// ```
///
/// # Parameters
/// - `data`: Slice of price data
/// - `period`: Number of periods for the moving average
///
/// # Returns
/// Vector of EMA values. The first `period-1` values will be NaN.
///
/// # Errors
/// - `EmptyInput` if data is empty
/// - `InvalidParameter` if period is 0
/// - `InsufficientData` if data length < period
///
/// # Example
/// ```rust
/// use ta_rust::overlap::ema;
///
/// let prices = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29];
/// let result = ema(&prices, 10).unwrap();
/// 
/// // First 9 values are NaN, then EMA starts
/// assert!(result[8].is_nan());
/// assert!(!result[9].is_nan());
/// ```
pub fn ema(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;

    let mut output = allocate_output(data.len());
    let multiplier = ema_multiplier(period);
    
    // Initialize EMA with SMA of first 'period' values
    let initial_sum: Price = data[0..period].iter().sum();
    let mut ema_value = initial_sum / period as Price;
    output[period - 1] = ema_value;
    
    // Calculate EMA for remaining values
    for i in period..data.len() {
        ema_value = (data[i] * multiplier) + (ema_value * (1.0 - multiplier));
        output[i] = ema_value;
    }

    Ok(output)
}

/// Calculates EMA starting from the first data point (alternative initialization)
///
/// This version uses the first price as the initial EMA value instead of SMA,
/// which is sometimes preferred for shorter datasets.
pub fn ema_from_first(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;

    let mut output = allocate_output(data.len());
    let multiplier = ema_multiplier(period);
    
    // Initialize with first price
    let mut ema_value = data[0];
    output[0] = ema_value;
    
    // Calculate EMA for all subsequent values
    for i in 1..data.len() {
        ema_value = (data[i] * multiplier) + (ema_value * (1.0 - multiplier));
        output[i] = ema_value;
    }

    Ok(output)
}

/// Calculates EMA with custom smoothing factor
///
/// Allows for custom smoothing factor instead of the standard 2/(period+1).
/// 
/// # Parameters
/// - `data`: Slice of price data
/// - `smoothing_factor`: Custom smoothing factor (0 < factor < 1)
///
/// # Example
/// ```rust
/// use ta_rust::overlap::ema_custom;
///
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = ema_custom(&prices, 0.5).unwrap(); // 50% smoothing
/// ```
pub fn ema_custom(data: &[Price], smoothing_factor: Price) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(data, "data")?;
    
    if smoothing_factor <= 0.0 || smoothing_factor >= 1.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "smoothing_factor",
            "must be between 0 and 1 (exclusive)"
        ));
    }

    let mut output = allocate_output(data.len());
    
    // Initialize with first price
    let mut ema_value = data[0];
    output[0] = ema_value;
    
    // Calculate EMA for all subsequent values
    for i in 1..data.len() {
        ema_value = (data[i] * smoothing_factor) + (ema_value * (1.0 - smoothing_factor));
        output[i] = ema_value;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_ema_basic() {
        let data = vec![22.27, 22.19, 22.08, 22.17, 22.18, 22.13, 22.23, 22.43, 22.24, 22.29];
        let result = ema(&data, 10).unwrap();
        
        // First 9 values should be NaN
        for i in 0..9 {
            assert!(result[i].is_nan());
        }
        
        // 10th value should be the SMA of first 10 values
        let expected_sma = data.iter().sum::<Price>() / 10.0;
        assert!((result[9] - expected_sma).abs() < DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ema_period_1() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(&data, 1).unwrap();
        
        // With period 1, EMA should equal the input data
        assert_arrays_approx_equal(&result, &data, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ema_known_values() {
        // Test with known EMA calculation
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema(&data, 3).unwrap();
        
        let expected = vec![
            Price::NAN, Price::NAN,
            2.0,  // SMA of first 3: (1+2+3)/3 = 2.0
            3.0,  // EMA: (4 * 0.5) + (2.0 * 0.5) = 3.0
            4.0,  // EMA: (5 * 0.5) + (3.0 * 0.5) = 4.0
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ema_from_first() {
        let data = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        let result = ema_from_first(&data, 3).unwrap();
        
        // Should start from first value, no NaN values
        assert!(!result[0].is_nan());
        assert_eq!(result[0], 10.0);
        
        // Check that all values are calculated
        for value in &result {
            assert!(!value.is_nan());
        }
    }

    #[test]
    fn test_ema_custom_smoothing() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema_custom(&data, 0.5).unwrap();
        
        // With 0.5 smoothing factor
        let expected = vec![
            1.0,   // First value
            1.5,   // (2.0 * 0.5) + (1.0 * 0.5) = 1.5
            2.25,  // (3.0 * 0.5) + (1.5 * 0.5) = 2.25
            3.125, // (4.0 * 0.5) + (2.25 * 0.5) = 3.125
            4.0625,// (5.0 * 0.5) + (3.125 * 0.5) = 4.0625
        ];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_ema_multiplier_calculation() {
        // Test that multiplier is calculated correctly
        assert!((ema_multiplier(10) - 2.0/11.0).abs() < 1e-10);
        assert!((ema_multiplier(20) - 2.0/21.0).abs() < 1e-10);
    }

    #[test]
    fn test_ema_convergence() {
        // EMA should converge to the constant value for constant input
        let data = vec![5.0; 20];
        let result = ema(&data, 10).unwrap();
        
        // Last few values should be very close to 5.0
        for i in 15..20 {
            assert!((result[i] - 5.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_ema_responsiveness() {
        // EMA should be more responsive than SMA to recent changes
        let mut data = vec![10.0; 10];
        data.extend(vec![20.0; 10]); // Sudden jump
        
        let ema_result = ema(&data, 10).unwrap();
        
        // EMA should react faster to the price jump
        // (This is more of a behavioral test)
        assert!(ema_result[15] > ema_result[10]);
    }

    #[test]
    fn test_ema_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(ema(&data, 3).is_err());
    }

    #[test]
    fn test_ema_empty_data() {
        let data = vec![];
        assert!(ema(&data, 3).is_err());
    }

    #[test]
    fn test_ema_zero_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(ema(&data, 0).is_err());
    }

    #[test]
    fn test_ema_custom_invalid_smoothing() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(ema_custom(&data, 0.0).is_err());
        assert!(ema_custom(&data, 1.0).is_err());
        assert!(ema_custom(&data, -0.1).is_err());
        assert!(ema_custom(&data, 1.1).is_err());
    }

    #[test]
    fn test_ema_real_market_data() {
        // Test with realistic market data
        let data = vec![
            44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 47.25, 47.92, 46.93,
            46.85, 46.80, 46.80, 46.85, 46.85, 47.92, 47.25, 46.93, 46.85, 46.80
        ];
        
        let result = ema(&data, 12).unwrap();
        
        // Should have valid values starting from index 11
        for i in 0..11 {
            assert!(result[i].is_nan());
        }
        for i in 11..data.len() {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Prices should be positive
        }
    }
}