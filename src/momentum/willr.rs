//! Williams' %R (WILLR)
//! 
//! Williams' %R is a momentum oscillator that measures overbought and oversold levels.
//! It oscillates between -100 and 0, with values above -20 considered overbought
//! and values below -80 considered oversold.

use crate::common::{TAError, validate_period};
use crate::math_operators::{max, min};

/// Validates HLC data consistency
fn validate_hlc(high: &[f64], low: &[f64], close: &[f64]) -> Result<(), TAError> {
    // Validate that high, low, close have same length
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TAError::mismatched_inputs("High, Low, and Close arrays must have the same length".to_string()));
    }
    
    // Validate HLC constraints for each bar
    for i in 0..high.len() {
        let (h, l, c) = (high[i], low[i], close[i]);
        
        if !h.is_finite() || !l.is_finite() || !c.is_finite() {
            return Err(TAError::invalid_input(format!(
                "Invalid HLC values at index {}: H={}, L={}, C={}",
                i, h, l, c
            )));
        }
        
        if h < l {
            return Err(TAError::invalid_input(format!(
                "High ({}) < Low ({}) at index {}",
                h, l, i
            )));
        }
        if c < l || c > h {
            return Err(TAError::invalid_input(format!(
                "Close ({}) is outside High-Low range [{}, {}] at index {}",
                c, l, h, i
            )));
        }
    }
    
    Ok(())
}

/// Calculates Williams' %R.
/// 
/// %R = -100 * (Highest High - Close) / (Highest High - Lowest Low)
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation (typically 14)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing Williams' %R values, or `Err(TAError)` on invalid input.
/// The first `period-1` values will be NaN as %R needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::willr;
/// 
/// let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0];
/// let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0];
/// let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0];
/// 
/// let result = willr(&high, &low, &close, 3).unwrap();
/// assert_eq!(result.len(), 7);
/// ```
pub fn willr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_hlc(high, low, close)?;
    validate_period(period, "period")?;
    
    let len = high.len();
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    
    // Calculate highest high and lowest low over period
    let highest_high = max(high, period)?;
    let lowest_low = min(low, period)?;
    
    let mut result = Vec::with_capacity(len);
    
    // Calculate Williams' %R
    for i in 0..len {
        if highest_high[i].is_nan() || lowest_low[i].is_nan() {
            result.push(f64::NAN);
        } else {
            let hh = highest_high[i];
            let ll = lowest_low[i];
            let range = hh - ll;
            
            if range == 0.0 {
                result.push(-50.0); // Midpoint when no range
            } else {
                let willr_val = -100.0 * (hh - close[i]) / range;
                result.push(willr_val);
            }
        }
    }
    
    Ok(result)
}

/// Calculates Williams' %R using OHLC data structure.
/// 
/// # Arguments
/// 
/// * `ohlc` - Slice of OHLC data
/// * `period` - Period for calculation
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing Williams' %R values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::common::OHLC;
/// use ta_rust::momentum::willr_ohlc;
/// 
/// let data = vec![
///     OHLC { open: 9.2, high: 10.0, low: 9.0, close: 9.5 },
///     OHLC { open: 9.5, high: 11.0, low: 10.0, close: 10.5 },
///     OHLC { open: 10.5, high: 12.0, low: 10.5, close: 11.5 },
///     OHLC { open: 11.5, high: 13.0, low: 11.0, close: 12.0 },
/// ];
/// 
/// let result = willr_ohlc(&data, 3).unwrap();
/// assert_eq!(result.len(), 4);
/// ```
pub fn willr_ohlc(ohlc: &[crate::common::types::OHLC], period: usize) -> Result<Vec<f64>, TAError> {
    if ohlc.is_empty() {
        return Err(TAError::invalid_input("OHLC data cannot be empty"));
    }
    
    let high: Vec<f64> = ohlc.iter().map(|x| x.high).collect();
    let low: Vec<f64> = ohlc.iter().map(|x| x.low).collect();
    let close: Vec<f64> = ohlc.iter().map(|x| x.close).collect();
    
    willr(&high, &low, &close, period)
}

/// Calculates Williams' %R with overbought/oversold levels.
/// 
/// Returns Williams' %R values along with overbought and oversold signals.
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation
/// * `overbought` - Overbought level (typically -20)
/// * `oversold` - Oversold level (typically -80)
/// 
/// # Returns
/// 
/// Returns `Ok((Vec<f64>, Vec<i8>))` containing (Williams' %R values, signals),
/// where signals are: 1 = oversold, -1 = overbought, 0 = neutral.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::willr_levels;
/// 
/// let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0];
/// let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0];
/// let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0];
/// 
/// let (willr_vals, signals) = willr_levels(&high, &low, &close, 3, -20.0, -80.0).unwrap();
/// assert_eq!(willr_vals.len(), 7);
/// assert_eq!(signals.len(), 7);
/// ```
pub fn willr_levels(high: &[f64], low: &[f64], close: &[f64], period: usize, overbought: f64, oversold: f64) -> Result<(Vec<f64>, Vec<i8>), TAError> {
    validate_hlc(high, low, close)?;
    validate_period(period, "period")?;
    
    if overbought <= oversold {
        return Err(TAError::invalid_input("Overbought level must be greater than oversold level"));
    }
    
    if overbought > 0.0 || oversold < -100.0 {
        return Err(TAError::invalid_input("Williams %R levels must be between -100 and 0"));
    }
    
    let willr_values = willr(high, low, close, period)?;
    let mut signals = vec![0i8; willr_values.len()];
    
    for (i, &willr_val) in willr_values.iter().enumerate() {
        if !willr_val.is_nan() {
            if willr_val >= overbought {
                signals[i] = -1; // Overbought
            } else if willr_val <= oversold {
                signals[i] = 1; // Oversold
            }
        }
    }
    
    Ok((willr_values, signals))
}

/// Calculates Williams' %R with smoothing.
/// 
/// This applies a simple moving average to the Williams' %R values for smoother signals.
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for Williams' %R calculation
/// * `smooth_period` - Period for smoothing (typically 3-5)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing smoothed Williams' %R values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::willr_smoothed;
/// 
/// let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0];
/// let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0, 12.5, 13.0];
/// let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0, 13.0, 14.0];
/// 
/// let result = willr_smoothed(&high, &low, &close, 3, 2).unwrap();
/// assert_eq!(result.len(), 9);
/// ```
pub fn willr_smoothed(high: &[f64], low: &[f64], close: &[f64], period: usize, smooth_period: usize) -> Result<Vec<f64>, TAError> {
    validate_hlc(high, low, close)?;
    validate_period(period, "period")?;
    validate_period(smooth_period, "smooth_period")?;
    
    let len = high.len();
    if len < period + smooth_period - 1 {
        return Err(TAError::insufficient_data(period + smooth_period - 1, len));
    }
    
    // Calculate basic Williams' %R first
    let willr_values = willr(high, low, close, period)?;
    
    let mut result = vec![f64::NAN; len];
    
    // Apply smoothing
    for i in (period + smooth_period - 2)..len {
        let start_idx = i - smooth_period + 1;
        let end_idx = i + 1;
        
        let sum: f64 = willr_values[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .sum();
        let count = willr_values[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .count();
        
        if count == smooth_period {
            result[i] = sum / smooth_period as f64;
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::OHLC;

    #[test]
    fn test_willr_basic() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0];
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(result.len(), 7);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Williams' %R values should be between -100 and 0
        for i in 2..7 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= -100.0);
            assert!(result[i] <= 0.0);
        }
    }

    #[test]
    fn test_willr_calculation() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![8.0, 9.0, 10.0];
        let close = vec![9.0, 10.0, 11.0];
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        // For the third value:
        // HH = 12.0, LL = 8.0, Close = 11.0
        // %R = -100 * (12.0 - 11.0) / (12.0 - 8.0) = -100 * 1.0 / 4.0 = -25.0
        assert!((result[2] - (-25.0)).abs() < 1e-8);
    }

    #[test]
    fn test_willr_close_at_high() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![8.0, 9.0, 10.0];
        let close = vec![10.0, 11.0, 12.0]; // Close at high
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        // When close equals highest high, %R should be 0
        assert!((result[2] - 0.0).abs() < 1e-8);
    }

    #[test]
    fn test_willr_close_at_low() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![8.0, 9.0, 8.0];  // Adjusted low to match close
        let close = vec![8.0, 9.0, 8.0]; // Close at lowest low
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        // When close equals lowest low, %R should be -100
        assert!((result[2] - (-100.0)).abs() < 1e-8);
    }

    #[test]
    fn test_willr_no_range() {
        let high = vec![10.0, 10.0, 10.0];
        let low = vec![10.0, 10.0, 10.0];
        let close = vec![10.0, 10.0, 10.0];
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        // When there's no range, %R should be -50 (midpoint)
        assert!((result[2] - (-50.0)).abs() < 1e-8);
    }

    #[test]
    fn test_willr_insufficient_data() {
        let high = vec![10.0, 11.0];
        let low = vec![9.0, 10.0];
        let close = vec![9.5, 10.5];
        
        let result = willr(&high, &low, &close, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_willr_empty_data() {
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        let result = willr(&high, &low, &close, 14);
        assert!(result.is_err());
    }

    #[test]
    fn test_willr_zero_period() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = willr(&high, &low, &close, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_willr_period_1() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = willr(&high, &low, &close, 1).unwrap();
        
        // With period 1, %R should be calculated for each individual bar
        for &willr_val in &result {
            assert!(!willr_val.is_nan());
            assert!(willr_val >= -100.0);
            assert!(willr_val <= 0.0);
        }
    }

    #[test]
    fn test_willr_ohlc() {
        let data = vec![
            OHLC { open: 9.2, high: 10.0, low: 9.0, close: 9.5 },
            OHLC { open: 9.5, high: 11.0, low: 10.0, close: 10.5 },
            OHLC { open: 10.5, high: 12.0, low: 10.5, close: 11.5 },
            OHLC { open: 11.5, high: 13.0, low: 11.0, close: 12.0 },
        ];
        
        let result = willr_ohlc(&data, 3).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(!result[2].is_nan());
        assert!(!result[3].is_nan());
    }

    #[test]
    fn test_willr_levels() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0];
        
        let (willr_vals, signals) = willr_levels(&high, &low, &close, 3, -20.0, -80.0).unwrap();
        
        assert_eq!(willr_vals.len(), 7);
        assert_eq!(signals.len(), 7);
        
        // Check signal logic
        for (_i, (&willr_val, &signal)) in willr_vals.iter().zip(signals.iter()).enumerate() {
            if !willr_val.is_nan() {
                if willr_val >= -20.0 {
                    assert_eq!(signal, -1); // Overbought
                } else if willr_val <= -80.0 {
                    assert_eq!(signal, 1); // Oversold
                } else {
                    assert_eq!(signal, 0); // Neutral
                }
            } else {
                assert_eq!(signal, 0);
            }
        }
    }

    #[test]
    fn test_willr_levels_invalid_params() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        // Overbought <= Oversold
        let result = willr_levels(&high, &low, &close, 2, -80.0, -20.0);
        assert!(result.is_err());
        
        // Invalid levels
        let result = willr_levels(&high, &low, &close, 2, 10.0, -80.0);
        assert!(result.is_err());
        
        let result = willr_levels(&high, &low, &close, 2, -20.0, -110.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_willr_smoothed() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5, 15.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0, 12.5, 13.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0, 13.0, 14.0];
        
        let result = willr_smoothed(&high, &low, &close, 3, 2).unwrap();
        
        assert_eq!(result.len(), 9);
        
        // First several values should be NaN
        for i in 0..3 {
            assert!(result[i].is_nan());
        }
        
        // Smoothed values should be valid
        for i in 3..9 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= -100.0);
            assert!(result[i] <= 0.0);
        }
    }

    #[test]
    fn test_willr_real_market_scenario() {
        // Simulate real market data
        let high = vec![
            100.0, 102.0, 101.5, 103.0, 99.0, 101.0, 104.0, 102.5,
            105.0, 103.0, 106.0, 104.5, 107.0, 105.0, 108.0
        ];
        let low = vec![
            98.0, 100.5, 99.0, 100.0, 96.0, 98.5, 101.0, 100.0,
            102.0, 101.0, 103.0, 102.0, 104.0, 103.0, 105.0
        ];
        let close = vec![
            99.0, 101.0, 100.0, 102.0, 97.0, 100.0, 103.0, 101.5,
            104.0, 102.0, 105.0, 103.5, 106.0, 104.0, 107.0
        ];
        
        let result = willr(&high, &low, &close, 14).unwrap();
        
        assert_eq!(result.len(), 15);
        
        // First 13 values should be NaN
        for i in 0..13 {
            assert!(result[i].is_nan());
        }
        
        // 14th value should be valid
        assert!(!result[13].is_nan());
        assert!(result[13] >= -100.0);
        assert!(result[13] <= 0.0);
        
        // Last value should be valid
        assert!(!result[14].is_nan());
        assert!(result[14] >= -100.0);
        assert!(result[14] <= 0.0);
    }

    #[test]
    fn test_willr_trending_market() {
        // Simulate trending market
        let high = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let low = vec![9.0, 10.0, 11.0, 12.0, 13.0, 14.0];
        let close = vec![9.5, 10.5, 11.5, 12.5, 13.5, 14.5];
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        // In a trending market, %R should show consistent patterns
        for i in 2..6 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= -100.0);
            assert!(result[i] <= 0.0);
        }
    }

    #[test]
    fn test_willr_overbought_oversold() {
        // Create scenario with overbought and oversold conditions
        let high = vec![10.0, 15.0, 12.0, 8.0, 14.0];
        let low = vec![5.0, 10.0, 7.0, 3.0, 9.0];
        let close = vec![9.5, 14.5, 7.5, 3.5, 13.5]; // Close near high/low
        
        let result = willr(&high, &low, &close, 3).unwrap();
        
        // Check that extreme values are handled correctly
        for i in 2..5 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= -100.0);
            assert!(result[i] <= 0.0);
        }
    }
}