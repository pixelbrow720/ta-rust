//! Chaikin A/D Oscillator (ADOSC)
//!
//! The A/D Oscillator is the difference between the fast and slow EMA of the A/D Line.
//! It's used to identify momentum changes in the accumulation/distribution pattern.

use crate::common::{TAError, TAResult};
use crate::volume::ad;
use crate::overlap::ema;

/// Chaikin A/D Oscillator (ADOSC)
///
/// The A/D Oscillator measures the momentum of the A/D Line by calculating the difference
/// between a fast EMA and slow EMA of the A/D Line.
///
/// # Formula
/// ```text
/// AD = Chaikin A/D Line
/// ADOSC = EMA(AD, fast_period) - EMA(AD, slow_period)
/// ```
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
/// * `close` - Slice of closing prices
/// * `volume` - Slice of volume data
/// * `fast_period` - Fast EMA period (default: 3)
/// * `slow_period` - Slow EMA period (default: 10)
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of A/D Oscillator values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::volume::adosc;
///
/// let high = vec![12.0, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0, 15.5, 17.0];
/// let low = vec![10.0, 11.0, 10.5, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0];
/// let close = vec![11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0];
/// let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0, 1800.0, 900.0, 2200.0, 1100.0, 1900.0];
/// let result = adosc(&high, &low, &close, &volume, 3, 10).unwrap();
/// ```
pub fn adosc(
    high: &[f64], 
    low: &[f64], 
    close: &[f64], 
    volume: &[f64], 
    fast_period: usize, 
    slow_period: usize
) -> TAResult<Vec<f64>> {
    if high.is_empty() || low.is_empty() || close.is_empty() || volume.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    let len = high.len();
    if len != low.len() || len != close.len() || len != volume.len() {
        return Err(TAError::mismatched_inputs("All input arrays must have the same length"));
    }
    
    if fast_period == 0 || slow_period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if fast_period >= slow_period {
        return Err(TAError::invalid_parameter("period", "less than slow period"));
    }
    
    // Calculate A/D Line
    let ad_line = ad(high, low, close, volume)?;
    
    // Calculate fast and slow EMAs of A/D Line
    let fast_ema = ema(&ad_line, fast_period)?;
    let slow_ema = ema(&ad_line, slow_period)?;
    
    // Calculate oscillator as difference
    let mut result = vec![f64::NAN; len];
    
    // The oscillator is valid when both EMAs are valid
    let start_idx = slow_period - 1;  // Slow EMA determines when we have valid values
    
    for i in start_idx..len {
        if !fast_ema[i].is_nan() && !slow_ema[i].is_nan() {
            result[i] = fast_ema[i] - slow_ema[i];
        }
    }
    
    Ok(result)
}

/// Chaikin A/D Oscillator with default periods (3, 10)
///
/// This is a convenience function that uses the standard default periods.
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
/// * `close` - Slice of closing prices
/// * `volume` - Slice of volume data
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of A/D Oscillator values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn adosc_default(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> TAResult<Vec<f64>> {
    adosc(high, low, close, volume, 3, 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_adosc_basic() {
        let high = vec![12.0, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0, 15.5, 17.0, 16.5, 18.0];
        let low = vec![10.0, 11.0, 10.5, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0];
        let close = vec![11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0, 15.5, 17.0];
        let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0, 1800.0, 900.0, 2200.0, 1100.0, 1900.0, 1300.0, 2100.0];
        
        let result = adosc(&high, &low, &close, &volume, 3, 10).unwrap();
        assert_eq!(result.len(), 12);
        
        // First 9 values should be NaN (slow_period - 1 = 9)
        for i in 0..9 {
            assert!(result[i].is_nan());
        }
        
        // Values from index 9 onwards should be valid
        for i in 9..12 {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_adosc_default() {
        let high = vec![12.0, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0, 15.5, 17.0, 16.5, 18.0];
        let low = vec![10.0, 11.0, 10.5, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0];
        let close = vec![11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0, 14.5, 16.0, 15.5, 17.0];
        let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0, 1800.0, 900.0, 2200.0, 1100.0, 1900.0, 1300.0, 2100.0];
        
        let result1 = adosc_default(&high, &low, &close, &volume).unwrap();
        let result2 = adosc(&high, &low, &close, &volume, 3, 10).unwrap();
        
        assert_eq!(result1.len(), result2.len());
        for i in 0..result1.len() {
            if result1[i].is_nan() && result2[i].is_nan() {
                continue;
            }
            assert_float_eq!(result1[i], result2[i], 1e-10);
        }
    }

    #[test]
    fn test_adosc_invalid_periods() {
        let high = vec![12.0, 13.0];
        let low = vec![10.0, 11.0];
        let close = vec![11.0, 12.0];
        let volume = vec![1000.0, 1500.0];
        
        // Fast period >= slow period
        assert!(adosc(&high, &low, &close, &volume, 10, 3).is_err());
        assert!(adosc(&high, &low, &close, &volume, 5, 5).is_err());
        
        // Zero periods
        assert!(adosc(&high, &low, &close, &volume, 0, 10).is_err());
        assert!(adosc(&high, &low, &close, &volume, 3, 0).is_err());
    }

    #[test]
    fn test_adosc_empty_input() {
        let high: Vec<f64> = vec![];
        let low: Vec<f64> = vec![];
        let close: Vec<f64> = vec![];
        let volume: Vec<f64> = vec![];
        assert!(adosc(&high, &low, &close, &volume, 3, 10).is_err());
    }

    #[test]
    fn test_adosc_mismatched_lengths() {
        let high = vec![10.0, 11.0];
        let low = vec![8.0];
        let close = vec![9.0, 10.0];
        let volume = vec![1000.0, 1500.0];
        assert!(adosc(&high, &low, &close, &volume, 3, 10).is_err());
    }

    #[test]
    fn test_adosc_insufficient_data() {
        // Test with data length less than slow period
        let high = vec![12.0, 13.0, 12.5];
        let low = vec![10.0, 11.0, 10.5];
        let close = vec![11.0, 12.0, 11.5];
        let volume = vec![1000.0, 1500.0, 800.0];
        
        // This should fail because EMA requires at least 'period' data points
        // and we only have 3 data points but need 10 for slow period
        let result = adosc(&high, &low, &close, &volume, 3, 10);
        assert!(result.is_err());
    }
}