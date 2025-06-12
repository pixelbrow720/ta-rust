//! Kaufman Adaptive Moving Average (KAMA)
//!
//! KAMA is a moving average designed to account for market noise or volatility.
//! It adapts to price movements by speeding up when prices are trending and
//! slowing down when prices are moving sideways.

use crate::common::{TAError, TAResult};

/// Kaufman Adaptive Moving Average (KAMA)
///
/// KAMA adjusts its smoothing constant based on the efficiency ratio, which measures
/// the directional movement relative to the volatility. When prices are trending,
/// KAMA acts more like a fast EMA. When prices are choppy, it acts more like a slow EMA.
///
/// # Formula
/// ```text
/// Direction = |Close[today] - Close[n periods ago]|
/// Volatility = Σ|Close[i] - Close[i-1]| for n periods
/// Efficiency Ratio (ER) = Direction / Volatility
/// 
/// Fast SC = 2 / (fast_period + 1)
/// Slow SC = 2 / (slow_period + 1)
/// SC = (ER × (Fast SC - Slow SC) + Slow SC)²
/// 
/// KAMA[today] = KAMA[yesterday] + SC × (Price[today] - KAMA[yesterday])
/// ```
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `period` - Period for efficiency ratio calculation (typically 10)
/// * `fast_period` - Fast EMA period for smoothing constant (typically 2)
/// * `slow_period` - Slow EMA period for smoothing constant (typically 30)
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of KAMA values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::kama;
///
/// let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0];
/// let result = kama(&close, 10, 2, 30).unwrap();
/// ```
pub fn kama(close: &[f64], period: usize, fast_period: usize, slow_period: usize) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if period == 0 || fast_period == 0 || slow_period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if period >= close.len() {
        return Err(TAError::insufficient_data(period, close.len()));
    }
    
    if fast_period >= slow_period {
        return Err(TAError::invalid_parameter("period", "less than slow period"));
    }
    
    let len = close.len();
    let mut result = vec![f64::NAN; len];
    
    // Calculate smoothing constants
    let fast_sc = 2.0 / (fast_period as f64 + 1.0);
    let slow_sc = 2.0 / (slow_period as f64 + 1.0);
    let sc_diff = fast_sc - slow_sc;
    
    // Initialize KAMA with first valid value (SMA of first period+1 values)
    let start_idx = period;
    if start_idx >= len {
        return Ok(result);
    }
    
    // Initialize KAMA with simple average of first period+1 values
    let mut kama = close[0..=start_idx].iter().sum::<f64>() / (start_idx + 1) as f64;
    result[start_idx] = kama;
    
    // Calculate KAMA for remaining values
    for i in (start_idx + 1)..len {
        // Calculate direction (change over period)
        let direction = (close[i] - close[i - period]).abs();
        
        // Calculate volatility (sum of absolute changes over period)
        let mut volatility = 0.0;
        for j in (i - period + 1)..=i {
            volatility += (close[j] - close[j - 1]).abs();
        }
        
        // Calculate efficiency ratio
        let er = if volatility.abs() < f64::EPSILON {
            0.0  // No volatility means no efficiency
        } else {
            direction / volatility
        };
        
        // Calculate smoothing constant
        let sc = (er * sc_diff + slow_sc).powi(2);
        
        // Update KAMA
        kama = kama + sc * (close[i] - kama);
        result[i] = kama;
    }
    
    Ok(result)
}

/// KAMA with default parameters (10, 2, 30)
///
/// This is a convenience function using the standard default parameters.
///
/// # Arguments
/// * `close` - Slice of closing prices
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of KAMA values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn kama_default(close: &[f64]) -> TAResult<Vec<f64>> {
    kama(close, 10, 2, 30)
}

/// Calculate KAMA Efficiency Ratio
///
/// Returns the efficiency ratio used in KAMA calculation, which can be useful
/// for understanding market conditions.
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `period` - Period for efficiency ratio calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of efficiency ratio values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn kama_efficiency_ratio(close: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if period >= close.len() {
        return Err(TAError::insufficient_data(period, close.len()));
    }
    
    let len = close.len();
    let mut result = vec![f64::NAN; len];
    
    for i in period..len {
        // Calculate direction (change over period)
        let direction = (close[i] - close[i - period]).abs();
        
        // Calculate volatility (sum of absolute changes over period)
        let mut volatility = 0.0;
        for j in (i - period + 1)..=i {
            volatility += (close[j] - close[j - 1]).abs();
        }
        
        // Calculate efficiency ratio
        result[i] = if volatility.abs() < f64::EPSILON {
            0.0
        } else {
            direction / volatility
        };
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_kama_basic() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0];
        let result = kama(&close, 10, 2, 30).unwrap();
        
        assert_eq!(result.len(), 12);
        
        // First 10 values should be NaN
        for i in 0..10 {
            assert!(result[i].is_nan());
        }
        
        // Values from index 10 onwards should be valid
        for i in 10..12 {
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_kama_default() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0];
        
        let result1 = kama_default(&close).unwrap();
        let result2 = kama(&close, 10, 2, 30).unwrap();
        
        assert_eq!(result1.len(), result2.len());
        for i in 0..result1.len() {
            if result1[i].is_nan() && result2[i].is_nan() {
                continue;
            }
            assert_float_eq!(result1[i], result2[i], 1e-10);
        }
    }

    #[test]
    fn test_kama_trending_market() {
        // Strong uptrend - KAMA should adapt quickly
        let close = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0];
        let result = kama(&close, 10, 2, 30).unwrap();
        
        assert_eq!(result.len(), 12);
        
        // In a trending market, KAMA should follow the trend relatively closely
        for i in 10..12 {
            assert!(!result[i].is_nan());
            // KAMA should be between the start and end of the trend
            assert!(result[i] >= 10.0 && result[i] <= close[i]);
        }
    }

    #[test]
    fn test_kama_sideways_market() {
        // Sideways market - KAMA should be less responsive
        let close = vec![20.0, 19.0, 21.0, 20.0, 19.0, 21.0, 20.0, 19.0, 21.0, 20.0, 19.0, 21.0];
        let result = kama(&close, 10, 2, 30).unwrap();
        
        assert_eq!(result.len(), 12);
        
        // In a sideways market, KAMA should be relatively stable
        for i in 10..12 {
            assert!(!result[i].is_nan());
            // KAMA should be around the average price
            assert!(result[i] >= 18.0 && result[i] <= 22.0);
        }
    }

    #[test]
    fn test_kama_efficiency_ratio() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0];
        let er = kama_efficiency_ratio(&close, 10).unwrap();
        
        assert_eq!(er.len(), 12);
        
        // First 10 values should be NaN
        for i in 0..10 {
            assert!(er[i].is_nan());
        }
        
        // Valid efficiency ratios should be between 0 and 1
        for i in 10..12 {
            assert!(!er[i].is_nan());
            assert!(er[i] >= 0.0 && er[i] <= 1.0);
        }
    }

    #[test]
    fn test_kama_constant_prices() {
        // Constant prices should result in zero efficiency ratio
        let close = vec![20.0; 15];
        let result = kama(&close, 10, 2, 30).unwrap();
        let er = kama_efficiency_ratio(&close, 10).unwrap();
        
        // Efficiency ratio should be 0 for constant prices
        for i in 10..15 {
            assert_float_eq!(er[i], 0.0, 1e-10);
        }
        
        // KAMA should equal the constant price
        for i in 10..15 {
            assert_float_eq!(result[i], 20.0, 1e-10);
        }
    }

    #[test]
    fn test_kama_invalid_input() {
        let close: Vec<f64> = vec![];
        assert!(kama(&close, 10, 2, 30).is_err());
        
        let close = vec![20.0, 21.0];
        assert!(kama(&close, 0, 2, 30).is_err());  // Zero period
        assert!(kama(&close, 10, 2, 30).is_err()); // Period >= data length
        assert!(kama(&close, 1, 0, 30).is_err());  // Zero fast period
        assert!(kama(&close, 1, 2, 0).is_err());   // Zero slow period
        assert!(kama(&close, 1, 30, 2).is_err());  // Fast >= slow period
    }

    #[test]
    fn test_kama_minimal_data() {
        let close = vec![20.0, 21.0, 22.0];
        let result = kama(&close, 2, 1, 3).unwrap();
        
        assert_eq!(result.len(), 3);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Last value should be valid
        assert!(!result[2].is_nan());
    }

    #[test]
    fn test_kama_adaptive_behavior() {
        // Create data with both trending and sideways periods
        let mut close = vec![];
        
        // Trending period
        for i in 0..10 {
            close.push(10.0 + i as f64);
        }
        
        // Sideways period
        for i in 0..10 {
            close.push(19.0 + (i % 2) as f64);
        }
        
        let result = kama(&close, 5, 2, 30).unwrap();
        let er = kama_efficiency_ratio(&close, 5).unwrap();
        
        // Efficiency ratio should be higher during trending period
        // and lower during sideways period
        assert_eq!(result.len(), 20);
        
        // Check that we have valid results
        for i in 5..20 {
            assert!(!result[i].is_nan());
            assert!(!er[i].is_nan());
            assert!(er[i] >= 0.0 && er[i] <= 1.0);
        }
    }
}