//! Parabolic SAR (Stop and Reverse)
//!
//! The Parabolic SAR is a trend-following indicator that provides potential reversal points.
//! It appears as dots above or below the price, indicating the direction of the trend.

use crate::common::{TAError, TAResult};

/// Parabolic SAR (Stop and Reverse)
///
/// The Parabolic SAR is designed to give exit points for long or short positions.
/// When the price crosses the SAR, it signals a potential trend reversal.
///
/// # Formula
/// ```text
/// Initial SAR = Low[0] for uptrend, High[0] for downtrend
/// EP (Extreme Point) = highest high in uptrend, lowest low in downtrend
/// SAR[tomorrow] = SAR[today] + AF × (EP - SAR[today])
/// 
/// AF starts at acceleration, increases by acceleration when new EP, max at max_acceleration
/// Reverse when price crosses SAR
/// ```
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
/// * `acceleration` - Acceleration factor (typically 0.02)
/// * `max_acceleration` - Maximum acceleration factor (typically 0.20)
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of SAR values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::sar;
///
/// let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
/// let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
/// let result = sar(&high, &low, 0.02, 0.20).unwrap();
/// ```
pub fn sar(high: &[f64], low: &[f64], acceleration: f64, max_acceleration: f64) -> TAResult<Vec<f64>> {
    if high.is_empty() || low.is_empty() {
        return Err(TAError::invalid_input("High and low arrays cannot be empty"));
    }
    
    if high.len() != low.len() {
        return Err(TAError::mismatched_inputs("High and low arrays must have the same length"));
    }
    
    if acceleration <= 0.0 || max_acceleration <= 0.0 {
        return Err(TAError::invalid_parameter("parameter", "must be greater than 0"));
    }
    
    if acceleration > max_acceleration {
        return Err(TAError::invalid_input("Acceleration cannot be greater than max acceleration"));
    }
    
    let len = high.len();
    if len < 2 {
        return Err(TAError::invalid_input("Need at least 2 data points"));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Initialize for first calculation
    let mut af = acceleration;
    let mut ep: f64;
    let mut sar: f64;
    let mut is_long: bool;
    
    // Determine initial trend direction based on first two periods
    if high[1] > high[0] {
        // Start with uptrend
        is_long = true;
        sar = low[0];
        ep = high[1];
    } else {
        // Start with downtrend
        is_long = false;
        sar = high[0];
        ep = low[1];
    }
    
    // Set initial SAR values
    result[0] = sar;
    result[1] = sar;
    
    for i in 2..len {
        // Update extreme point before calculating new SAR
        if is_long {
            if high[i - 1] > ep {
                ep = high[i - 1];
                af = (af + acceleration).min(max_acceleration);
            }
        } else {
            if low[i - 1] < ep {
                ep = low[i - 1];
                af = (af + acceleration).min(max_acceleration);
            }
        }
        
        let prev_sar = sar;
        
        // Calculate new SAR
        sar = prev_sar + af * (ep - prev_sar);
        
        if is_long {
            // In uptrend
            // SAR cannot be above the low of current or previous period
            sar = sar.min(low[i]).min(low[i - 1]);
            
            // Check for trend reversal
            if low[i] <= sar {
                // Trend reversal to downtrend
                is_long = false;
                sar = ep;  // SAR becomes the previous EP
                ep = low[i];  // New EP is current low
                af = acceleration;  // Reset AF
            } else {
                // Continue uptrend - check current period for new EP
                if high[i] > ep {
                    ep = high[i];  // New extreme point
                    af = (af + acceleration).min(max_acceleration);  // Increase AF
                }
            }
        } else {
            // In downtrend
            // SAR cannot be below the high of current or previous period
            sar = sar.max(high[i]).max(high[i - 1]);
            
            // Check for trend reversal
            if high[i] >= sar {
                // Trend reversal to uptrend
                is_long = true;
                sar = ep;  // SAR becomes the previous EP
                ep = high[i];  // New EP is current high
                af = acceleration;  // Reset AF
            } else {
                // Continue downtrend - check current period for new EP
                if low[i] < ep {
                    ep = low[i];  // New extreme point
                    af = (af + acceleration).min(max_acceleration);  // Increase AF
                }
            }
        }
        
        result[i] = sar;
    }
    
    Ok(result)
}

/// Parabolic SAR with default parameters (0.02, 0.20)
///
/// This is a convenience function using the standard default parameters.
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of SAR values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn sar_default(high: &[f64], low: &[f64]) -> TAResult<Vec<f64>> {
    sar(high, low, 0.02, 0.20)
}

/// Get SAR trend direction
///
/// Returns a vector indicating the trend direction at each point.
/// 1.0 = uptrend (SAR below price), -1.0 = downtrend (SAR above price)
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
/// * `sar_values` - SAR values from sar() function
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of trend direction values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn sar_trend(high: &[f64], low: &[f64], sar_values: &[f64]) -> TAResult<Vec<f64>> {
    if high.len() != low.len() || high.len() != sar_values.len() {
        return Err(TAError::mismatched_inputs("All arrays must have the same length"));
    }
    
    let len = high.len();
    let mut result = vec![f64::NAN; len];
    
    for i in 0..len {
        if !sar_values[i].is_nan() {
            // If SAR is below the low, it's an uptrend
            // If SAR is above the high, it's a downtrend
            if sar_values[i] < low[i] {
                result[i] = 1.0;  // Uptrend
            } else if sar_values[i] > high[i] {
                result[i] = -1.0;  // Downtrend
            } else {
                // SAR is between high and low (transition)
                result[i] = 0.0;
            }
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_sar_basic() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
        let result = sar(&high, &low, 0.02, 0.20).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // All values should be valid (no NaN)
        for value in &result {
            assert!(!value.is_nan());
        }
        
        // First SAR should be reasonable
        assert!(result[0] >= low[0] && result[0] <= high[0]);
    }

    #[test]
    fn test_sar_default() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5];
        
        let result1 = sar_default(&high, &low).unwrap();
        let result2 = sar(&high, &low, 0.02, 0.20).unwrap();
        
        assert_eq!(result1.len(), result2.len());
        for i in 0..result1.len() {
            assert_float_eq!(result1[i], result2[i], 1e-10);
        }
    }

    #[test]
    fn test_sar_trend() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
        let sar_values = sar(&high, &low, 0.02, 0.20).unwrap();
        let trend = sar_trend(&high, &low, &sar_values).unwrap();
        
        assert_eq!(trend.len(), 10);
        
        // All values should be valid
        for value in &trend {
            assert!(!value.is_nan());
            assert!(value == &1.0 || value == &-1.0 || value == &0.0);
        }
    }

    #[test]
    fn test_sar_uptrend() {
        // Create a clear uptrend
        let high = vec![21.0, 22.0, 23.0, 24.0, 25.0];
        let low = vec![20.0, 21.0, 22.0, 23.0, 24.0];
        let result = sar(&high, &low, 0.02, 0.20).unwrap();
        
        // In a clear uptrend, SAR should generally be below the lows
        for i in 1..result.len() {
            assert!(result[i] <= low[i], "SAR should be below low in uptrend at index {}", i);
        }
    }

    #[test]
    fn test_sar_downtrend() {
        // Create a clear downtrend
        let high = vec![25.0, 24.0, 23.0, 22.0, 21.0];
        let low = vec![24.0, 23.0, 22.0, 21.0, 20.0];
        let result = sar(&high, &low, 0.02, 0.20).unwrap();
        
        // In a clear downtrend, SAR should generally be above the highs
        // (after the initial period)
        for i in 2..result.len() {
            assert!(result[i] >= high[i], "SAR should be above high in downtrend at index {}", i);
        }
    }

    #[test]
    fn test_sar_invalid_input() {
        let high: Vec<f64> = vec![];
        let low: Vec<f64> = vec![];
        assert!(sar(&high, &low, 0.02, 0.20).is_err());
        
        let high = vec![22.0, 23.0];
        let low = vec![20.0];
        assert!(sar(&high, &low, 0.02, 0.20).is_err());  // Mismatched lengths
        
        let high = vec![22.0, 23.0];
        let low = vec![20.0, 21.0];
        assert!(sar(&high, &low, 0.0, 0.20).is_err());   // Zero acceleration
        assert!(sar(&high, &low, -0.02, 0.20).is_err()); // Negative acceleration
        assert!(sar(&high, &low, 0.25, 0.20).is_err());  // AF > max AF
    }

    #[test]
    fn test_sar_single_point() {
        let high = vec![22.0];
        let low = vec![20.0];
        assert!(sar(&high, &low, 0.02, 0.20).is_err());  // Need at least 2 points
    }

    #[test]
    fn test_sar_acceleration_increase() {
        // Test that acceleration factor increases with new extreme points
        let high = vec![20.0, 21.0, 22.0, 23.0, 24.0, 25.0];
        let low = vec![19.0, 20.0, 21.0, 22.0, 23.0, 24.0];
        let result = sar(&high, &low, 0.02, 0.20).unwrap();
        
        // Should have valid results
        assert_eq!(result.len(), 6);
        for value in &result {
            assert!(!value.is_nan());
        }
    }
}
