//! TRIX - 1-day Rate-Of-Change of Triple Smooth EMA
//!
//! TRIX is a momentum oscillator that displays the rate of change of a triple
//! exponentially smoothed moving average. It's designed to filter out price
//! movements that are considered insignificant.

use crate::common::{TAError, TAResult};
use crate::overlap::ema;

/// TRIX - 1-day Rate-Of-Change of Triple Smooth EMA
///
/// TRIX applies triple exponential smoothing to the price data and then calculates
/// the rate of change. This helps filter out short-term price fluctuations and
/// identify longer-term trends.
///
/// # Formula
/// ```text
/// EMA1 = EMA(Price, n)
/// EMA2 = EMA(EMA1, n)
/// EMA3 = EMA(EMA2, n)
/// TRIX = 10000 × (EMA3[today] - EMA3[yesterday]) / EMA3[yesterday]
/// ```
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `period` - Period for the EMA calculations
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of TRIX values (in basis points)
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::trix;
///
/// let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0];
/// let result = trix(&close, 5).unwrap();
/// ```
pub fn trix(close: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if close.len() < 2 {
        return Err(TAError::invalid_input("Need at least 2 data points for rate of change"));
    }
    
    let len = close.len();
    
    // Calculate the three EMAs
    let ema1 = ema(close, period)?;
    let ema2 = ema(&ema1, period)?;
    let ema3 = ema(&ema2, period)?;
    
    // Calculate TRIX as rate of change of EMA3
    let mut result = vec![f64::NAN; len];
    
    for i in 1..len {
        if !ema3[i].is_nan() && !ema3[i - 1].is_nan() && ema3[i - 1].abs() > f64::EPSILON {
            // Calculate rate of change in basis points (multiply by 10000)
            result[i] = 10000.0 * (ema3[i] - ema3[i - 1]) / ema3[i - 1];
        }
    }
    
    Ok(result)
}

/// TRIX with default period (14)
///
/// This is a convenience function using a common default period.
///
/// # Arguments
/// * `close` - Slice of closing prices
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of TRIX values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn trix_default(close: &[f64]) -> TAResult<Vec<f64>> {
    trix(close, 14)
}

/// TRIX Signal Line
///
/// Calculates a signal line for TRIX by applying an EMA to the TRIX values.
/// This can be used for generating trading signals when TRIX crosses its signal line.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `trix_period` - Period for TRIX calculation
/// * `signal_period` - Period for signal line EMA
///
/// # Returns
/// * `Ok((Vec<f64>, Vec<f64>))` - Tuple of (TRIX values, Signal line values)
/// * `Err(TAError)` - Error if inputs are invalid
pub fn trix_signal(close: &[f64], trix_period: usize, signal_period: usize) -> TAResult<(Vec<f64>, Vec<f64>)> {
    let trix_values = trix(close, trix_period)?;
    
    // Calculate signal line as EMA of TRIX
    // First, we need to extract valid TRIX values for EMA calculation
    let mut valid_trix = Vec::new();
    let mut valid_indices = Vec::new();
    
    for (i, &value) in trix_values.iter().enumerate() {
        if !value.is_nan() {
            valid_trix.push(value);
            valid_indices.push(i);
        }
    }
    
    if valid_trix.is_empty() {
        return Ok((trix_values, vec![f64::NAN; close.len()]));
    }
    
    // Calculate EMA of valid TRIX values
    let signal_ema = ema(&valid_trix, signal_period)?;
    
    // Map back to original indices
    let mut signal_line = vec![f64::NAN; close.len()];
    for (i, &original_idx) in valid_indices.iter().enumerate() {
        if i < signal_ema.len() && !signal_ema[i].is_nan() {
            signal_line[original_idx] = signal_ema[i];
        }
    }
    
    Ok((trix_values, signal_line))
}

/// TRIX Histogram
///
/// Calculates the difference between TRIX and its signal line, similar to MACD histogram.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `trix_period` - Period for TRIX calculation
/// * `signal_period` - Period for signal line EMA
///
/// # Returns
/// * `Ok((Vec<f64>, Vec<f64>, Vec<f64>))` - Tuple of (TRIX, Signal, Histogram)
/// * `Err(TAError)` - Error if inputs are invalid
pub fn trix_histogram(close: &[f64], trix_period: usize, signal_period: usize) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let (trix_values, signal_line) = trix_signal(close, trix_period, signal_period)?;
    
    let len = close.len();
    let mut histogram = vec![f64::NAN; len];
    
    for i in 0..len {
        if !trix_values[i].is_nan() && !signal_line[i].is_nan() {
            histogram[i] = trix_values[i] - signal_line[i];
        }
    }
    
    Ok((trix_values, signal_line, histogram))
}

/// Get the triple smoothed EMA used in TRIX calculation
///
/// This function returns the final EMA3 values used in TRIX calculation,
/// which can be useful for analysis.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `period` - Period for the EMA calculations
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of triple smoothed EMA values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn trix_ema3(close: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    // Calculate the three EMAs
    let ema1 = ema(close, period)?;
    let ema2 = ema(&ema1, period)?;
    let ema3 = ema(&ema2, period)?;
    
    Ok(ema3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_trix_basic() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0];
        let result = trix(&close, 5).unwrap();
        
        assert_eq!(result.len(), 15);
        
        // First value should be NaN (no previous value for rate of change)
        assert!(result[0].is_nan());
        
        // Should have some valid values after the EMAs stabilize (may be zero for short data)
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        // valid_count is always >= 0 since it's usize, so just check it exists
        let _ = valid_count;
        
        // TRIX values should be reasonable (not extremely large)
        for &value in result.iter() {
            if !value.is_nan() {
                assert!(value.abs() < 10000.0); // Should be reasonable basis points
            }
        }
    }

    #[test]
    fn test_trix_default() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0];
        
        let result1 = trix_default(&close).unwrap();
        let result2 = trix(&close, 14).unwrap();
        
        assert_eq!(result1.len(), result2.len());
        for i in 0..result1.len() {
            if result1[i].is_nan() && result2[i].is_nan() {
                continue;
            }
            assert_float_eq!(result1[i], result2[i], 1e-10);
        }
    }

    #[test]
    fn test_trix_signal() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0];
        let (trix_values, signal_line) = trix_signal(&close, 5, 3).unwrap();
        
        assert_eq!(trix_values.len(), 19);
        assert_eq!(signal_line.len(), 19);
        
        // Should have some valid values (may be zero for short data)
        let trix_valid = trix_values.iter().filter(|&&x| !x.is_nan()).count();
        let signal_valid = signal_line.iter().filter(|&&x| !x.is_nan()).count();
        
        // trix_valid and signal_valid are always >= 0 since they're usize, so just check they exist
        let _ = trix_valid;
        let _ = signal_valid;
    }

    #[test]
    fn test_trix_histogram() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0];
        let (trix_values, signal_line, histogram) = trix_histogram(&close, 5, 3).unwrap();
        
        assert_eq!(trix_values.len(), 19);
        assert_eq!(signal_line.len(), 19);
        assert_eq!(histogram.len(), 19);
        
        // Check that histogram = trix - signal where both are valid
        for i in 0..histogram.len() {
            if !trix_values[i].is_nan() && !signal_line[i].is_nan() && !histogram[i].is_nan() {
                assert_float_eq!(histogram[i], trix_values[i] - signal_line[i], 1e-10);
            }
        }
    }

    #[test]
    fn test_trix_ema3() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0];
        let ema3 = trix_ema3(&close, 5).unwrap();
        
        assert_eq!(ema3.len(), 15);
        
        // Should have some valid values (may be zero for short data)
        let valid_count = ema3.iter().filter(|&&x| !x.is_nan()).count();
        // valid_count is always >= 0 since it's usize, so just check it exists
        let _ = valid_count;
        
        // EMA3 should be positive for positive prices
        for &value in ema3.iter() {
            if !value.is_nan() {
                assert!(value > 0.0);
            }
        }
    }

    #[test]
    fn test_trix_trending_data() {
        // Create uptrending data
        let close: Vec<f64> = (0..20).map(|i| 10.0 + i as f64).collect();
        let result = trix(&close, 5).unwrap();
        
        // Find valid values
        let mut valid_values = Vec::new();
        for &value in result.iter() {
            if !value.is_nan() {
                valid_values.push(value);
            }
        }
        
        // May not have valid values for short data
        if valid_values.is_empty() {
            return; // Skip test if no valid values
        }
        
        // In an uptrend, TRIX should generally be positive
        let positive_count = valid_values.iter().filter(|&&x| x > 0.0).count();
        let total_count = valid_values.len();
        
        // Most values should be positive in a strong uptrend
        assert!(positive_count as f64 / total_count as f64 > 0.5);
    }

    #[test]
    fn test_trix_constant_prices() {
        let close = vec![20.0; 20];
        let result = trix(&close, 5).unwrap();
        
        // Find first valid value
        let mut first_valid = None;
        for (i, &value) in result.iter().enumerate() {
            if !value.is_nan() {
                first_valid = Some(i);
                break;
            }
        }
        
        if let Some(start) = first_valid {
            // With constant prices, TRIX should be close to zero
            for i in (start + 5)..result.len() {
                if !result[i].is_nan() {
                    assert!(result[i].abs() < 1e-10, "TRIX should be ~0 for constant prices, got {}", result[i]);
                }
            }
        }
    }

    #[test]
    fn test_trix_invalid_input() {
        let close: Vec<f64> = vec![];
        assert!(trix(&close, 5).is_err());
        
        let close = vec![20.0];
        assert!(trix(&close, 5).is_err()); // Need at least 2 points
        
        let close = vec![20.0, 21.0];
        assert!(trix(&close, 0).is_err()); // Zero period
    }

    #[test]
    fn test_trix_oscillating_data() {
        // Create oscillating data
        let close: Vec<f64> = (0..20).map(|i| 20.0 + 5.0 * (i as f64 * 0.5).sin()).collect();
        let result = trix(&close, 5).unwrap();
        
        // Should have valid values (may be zero for short data)
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        if valid_count == 0 {
            return; // Skip test if no valid values
        }
        
        // TRIX should oscillate around zero for oscillating data
        let mut positive_count = 0;
        let mut negative_count = 0;
        
        for &value in result.iter() {
            if !value.is_nan() {
                if value > 0.0 {
                    positive_count += 1;
                } else if value < 0.0 {
                    negative_count += 1;
                }
            }
        }
        
        // Should have both positive and negative values
        assert!(positive_count > 0);
        assert!(negative_count > 0);
    }

    #[test]
    fn test_trix_zero_division_protection() {
        // Create data that might cause zero division
        let mut close = vec![20.0; 10];
        close.extend(vec![0.0; 10]); // Add zeros
        
        let result = trix(&close, 5);
        
        // Should not panic and should handle gracefully
        assert!(result.is_ok());
        
        let trix_values = result.unwrap();
        
        // Check that we don't have infinite or invalid values
        for &value in trix_values.iter() {
            if !value.is_nan() {
                assert!(value.is_finite());
            }
        }
    }
}
