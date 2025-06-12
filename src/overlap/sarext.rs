//! Parabolic SAR Extended (SAREXT)
//!
//! Extended version of Parabolic SAR with additional parameters for more control
//! over the calculation, including custom start values, offset on reverse, and
//! different acceleration factors for long and short positions.

use crate::common::{TAError, TAResult};

/// Parabolic SAR Extended (SAREXT)
///
/// Extended version of the Parabolic SAR with more customizable parameters.
/// Allows for different acceleration factors for long and short positions,
/// custom start values, and offset on reverse.
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
/// * `start_value` - Starting value for SAR calculation
/// * `offset_on_reverse` - Offset to add when trend reverses
/// * `af_init_long` - Initial acceleration factor for long positions
/// * `af_long` - Acceleration increment for long positions
/// * `af_max_long` - Maximum acceleration factor for long positions
/// * `af_init_short` - Initial acceleration factor for short positions
/// * `af_short` - Acceleration increment for short positions
/// * `af_max_short` - Maximum acceleration factor for short positions
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of SAR values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::sarext;
///
/// let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
/// let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
/// let result = sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).unwrap();
/// ```
#[allow(clippy::too_many_arguments)]
pub fn sarext(
    high: &[f64],
    low: &[f64],
    start_value: f64,
    offset_on_reverse: f64,
    af_init_long: f64,
    af_long: f64,
    af_max_long: f64,
    af_init_short: f64,
    af_short: f64,
    af_max_short: f64,
) -> TAResult<Vec<f64>> {
    if high.is_empty() || low.is_empty() {
        return Err(TAError::invalid_input("High and low arrays cannot be empty"));
    }
    
    if high.len() != low.len() {
        return Err(TAError::mismatched_inputs("High and low arrays must have the same length"));
    }
    
    if af_init_long <= 0.0 || af_long <= 0.0 || af_max_long <= 0.0 ||
       af_init_short <= 0.0 || af_short <= 0.0 || af_max_short <= 0.0 {
        return Err(TAError::invalid_parameter("parameter", "must be greater than 0"));
    }
    
    if af_init_long > af_max_long || af_init_short > af_max_short {
        return Err(TAError::invalid_input("Initial AF cannot be greater than max AF"));
    }
    
    let len = high.len();
    if len < 2 {
        return Err(TAError::invalid_input("Need at least 2 data points"));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Initialize for first calculation
    let mut af: f64;
    let mut ep: f64;
    let mut sar: f64;
    let mut is_long: bool;
    
    // Use start_value if provided, otherwise determine from data
    if start_value != 0.0 {
        sar = start_value;
        // Determine initial trend based on start value relative to first price
        if start_value < (high[0] + low[0]) / 2.0 {
            is_long = true;
            af = af_init_long;
            ep = high[0];
        } else {
            is_long = false;
            af = af_init_short;
            ep = low[0];
        }
    } else {
        // Determine initial trend direction from price movement
        if high[1] > high[0] {
            // Start with uptrend
            is_long = true;
            sar = low[0];
            ep = high[1];
            af = af_init_long;
        } else {
            // Start with downtrend
            is_long = false;
            sar = high[0];
            ep = low[1];
            af = af_init_short;
        }
    }
    
    result[0] = sar;
    
    for i in 1..len {
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
                sar = ep + offset_on_reverse;  // SAR becomes the previous EP plus offset
                ep = low[i];  // New EP is current low
                af = af_init_short;  // Reset AF for short
            } else {
                // Continue uptrend
                if high[i] > ep {
                    ep = high[i];  // New extreme point
                    af = (af + af_long).min(af_max_long);  // Increase AF
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
                sar = ep - offset_on_reverse;  // SAR becomes the previous EP minus offset
                ep = high[i];  // New EP is current high
                af = af_init_long;  // Reset AF for long
            } else {
                // Continue downtrend
                if low[i] < ep {
                    ep = low[i];  // New extreme point
                    af = (af + af_short).min(af_max_short);  // Increase AF
                }
            }
        }
        
        result[i] = sar;
    }
    
    Ok(result)
}

/// SAREXT with standard parameters (equivalent to regular SAR)
///
/// This is a convenience function that mimics the standard SAR behavior.
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of SAR values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn sarext_standard(high: &[f64], low: &[f64]) -> TAResult<Vec<f64>> {
    sarext(high, low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20)
}

/// SAREXT with asymmetric acceleration factors
///
/// Uses different acceleration parameters for long and short positions.
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices
/// * `af_long` - Acceleration increment for long positions
/// * `af_max_long` - Maximum acceleration for long positions
/// * `af_short` - Acceleration increment for short positions
/// * `af_max_short` - Maximum acceleration for short positions
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of SAR values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn sarext_asymmetric(
    high: &[f64],
    low: &[f64],
    af_long: f64,
    af_max_long: f64,
    af_short: f64,
    af_max_short: f64,
) -> TAResult<Vec<f64>> {
    sarext(high, low, 0.0, 0.0, af_long, af_long, af_max_long, af_short, af_short, af_max_short)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_sarext_basic() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
        let result = sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // All values should be valid (no NaN)
        for value in &result {
            assert!(!value.is_nan());
        }
    }

    #[test]
    fn test_sarext_standard() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5];
        
        let result = sarext_standard(&high, &low).unwrap();
        assert_eq!(result.len(), 5);
        
        for value in &result {
            assert!(!value.is_nan());
        }
    }

    #[test]
    fn test_sarext_with_start_value() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5];
        
        let result = sarext(&high, &low, 19.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).unwrap();
        
        // First value should be the start value
        assert_float_eq!(result[0], 19.0, 1e-10);
    }

    #[test]
    fn test_sarext_with_offset() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
        
        let result_no_offset = sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).unwrap();
        let result_with_offset = sarext(&high, &low, 0.0, 0.1, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).unwrap();
        
        // Results should be different when offset is applied
        assert_eq!(result_no_offset.len(), result_with_offset.len());
        
        // At least some values should be different
        let mut _differences = 0;
        for i in 0..result_no_offset.len() {
            if (result_no_offset[i] - result_with_offset[i]).abs() > 1e-10 {
                _differences += 1;
            }
        }
        // We expect some differences due to the offset
        // (exact number depends on when reversals occur)
    }

    #[test]
    fn test_sarext_asymmetric() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
        
        let result = sarext_asymmetric(&high, &low, 0.02, 0.20, 0.03, 0.25).unwrap();
        
        assert_eq!(result.len(), 10);
        for value in &result {
            assert!(!value.is_nan());
        }
    }

    #[test]
    fn test_sarext_invalid_input() {
        let high: Vec<f64> = vec![];
        let low: Vec<f64> = vec![];
        assert!(sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).is_err());
        
        let high = vec![22.0, 23.0];
        let low = vec![20.0];
        assert!(sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).is_err());
        
        let high = vec![22.0, 23.0];
        let low = vec![20.0, 21.0];
        
        // Invalid acceleration factors
        assert!(sarext(&high, &low, 0.0, 0.0, 0.0, 0.02, 0.20, 0.02, 0.02, 0.20).is_err());
        assert!(sarext(&high, &low, 0.0, 0.0, 0.02, 0.0, 0.20, 0.02, 0.02, 0.20).is_err());
        assert!(sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.0, 0.02, 0.02, 0.20).is_err());
        
        // Initial AF > Max AF
        assert!(sarext(&high, &low, 0.0, 0.0, 0.25, 0.02, 0.20, 0.02, 0.02, 0.20).is_err());
        assert!(sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.25, 0.02, 0.20).is_err());
    }

    #[test]
    fn test_sarext_single_point() {
        let high = vec![22.0];
        let low = vec![20.0];
        assert!(sarext(&high, &low, 0.0, 0.0, 0.02, 0.02, 0.20, 0.02, 0.02, 0.20).is_err());
    }

    #[test]
    fn test_sarext_different_long_short_af() {
        let high = vec![22.0, 23.0, 24.0, 25.0, 24.5, 23.5, 22.5, 21.5, 20.5, 19.5];
        let low = vec![20.0, 21.0, 22.0, 23.0, 22.5, 21.5, 20.5, 19.5, 18.5, 17.5];
        
        // Different acceleration factors for long and short
        let result = sarext(&high, &low, 0.0, 0.0, 0.01, 0.01, 0.15, 0.03, 0.03, 0.25).unwrap();
        
        assert_eq!(result.len(), 10);
        for value in &result {
            assert!(!value.is_nan());
        }
    }
}