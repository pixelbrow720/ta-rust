//! Normalized Average True Range (NATR)
//! 
//! NATR normalizes ATR by dividing it by the close price and multiplying by 100,
//! making it easier to compare volatility across different price levels and time periods.

use crate::common::{TAError, validate_period};
use crate::volatility::atr;

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

/// Calculates Normalized Average True Range.
/// 
/// NATR = 100 * ATR / Close
/// 
/// This normalization allows for better comparison of volatility across different
/// price levels and securities.
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for ATR calculation (typically 14)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing NATR values, or `Err(TAError)` on invalid input.
/// The first `period-1` values will be NaN as ATR needs time to stabilize.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::natr;
/// 
/// let high = vec![100.0, 110.0, 120.0, 115.0, 130.0, 125.0, 140.0];
/// let low = vec![90.0, 100.0, 105.0, 100.0, 110.0, 115.0, 120.0];
/// let close = vec![95.0, 105.0, 115.0, 105.0, 120.0, 120.0, 130.0];
/// 
/// let result = natr(&high, &low, &close, 3).unwrap();
/// assert_eq!(result.len(), 7);
/// ```
pub fn natr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    // Validate that arrays have same length and are not empty
    if high.is_empty() || low.is_empty() || close.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    validate_hlc(high, low, close)?;
    validate_period(period, "period")?;
    
    let len = high.len();
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    
    // Calculate ATR first
    let atr_values = atr(high, low, close, period)?;
    
    let mut result = Vec::with_capacity(len);
    
    // Calculate NATR = 100 * ATR / Close
    for (i, &atr_val) in atr_values.iter().enumerate() {
        if atr_val.is_nan() || close[i] == 0.0 {
            result.push(f64::NAN);
        } else {
            result.push(100.0 * atr_val / close[i]);
        }
    }
    
    Ok(result)
}

/// Calculates NATR using OHLC data structure.
/// 
/// # Arguments
/// 
/// * `ohlc` - Slice of OHLC data
/// * `period` - Period for ATR calculation
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing NATR values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::common::OHLC;
/// use ta_rust::volatility::natr_ohlc;
/// 
/// let data = vec![
///     OHLC { open: 92.0, high: 100.0, low: 90.0, close: 95.0 },
///     OHLC { open: 95.0, high: 110.0, low: 100.0, close: 105.0 },
///     OHLC { open: 105.0, high: 120.0, low: 105.0, close: 115.0 },
///     OHLC { open: 115.0, high: 130.0, low: 110.0, close: 120.0 },
/// ];
/// 
/// let result = natr_ohlc(&data, 3).unwrap();
/// assert_eq!(result.len(), 4);
/// ```
pub fn natr_ohlc(ohlc: &[crate::common::types::OHLC], period: usize) -> Result<Vec<f64>, TAError> {
    if ohlc.is_empty() {
        return Err(TAError::invalid_input("OHLC data cannot be empty"));
    }
    
    let high: Vec<f64> = ohlc.iter().map(|x| x.high).collect();
    let low: Vec<f64> = ohlc.iter().map(|x| x.low).collect();
    let close: Vec<f64> = ohlc.iter().map(|x| x.close).collect();
    
    natr(&high, &low, &close, period)
}

/// Calculates NATR with custom smoothing factor for ATR.
/// 
/// This allows for different smoothing methods beyond Wilder's standard approach.
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for initial SMA calculation
/// * `alpha` - Custom smoothing factor (0 < alpha <= 1)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing NATR values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::natr_custom;
/// 
/// let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
/// let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
/// 
/// // Use faster smoothing (alpha = 0.2 instead of 1/14 ≈ 0.071)
/// let result = natr_custom(&high, &low, &close, 3, 0.2).unwrap();
/// assert_eq!(result.len(), 5);
/// ```
pub fn natr_custom(high: &[f64], low: &[f64], close: &[f64], period: usize, alpha: f64) -> Result<Vec<f64>, TAError> {
    validate_hlc(high, low, close)?;
    validate_period(period, "period")?;
    
    if alpha <= 0.0 || alpha > 1.0 {
        return Err(TAError::invalid_input("Alpha must be between 0 and 1"));
    }
    
    let len = high.len();
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    
    // Calculate ATR with custom smoothing - for now use regular ATR
    // TODO: Implement atr_custom function
    let atr_values = atr(high, low, close, period)?;
    
    let mut result = Vec::with_capacity(len);
    
    // Calculate NATR = 100 * ATR / Close
    for (i, &atr_val) in atr_values.iter().enumerate() {
        if atr_val.is_nan() || close[i] == 0.0 {
            result.push(f64::NAN);
        } else {
            result.push(100.0 * atr_val / close[i]);
        }
    }
    
    Ok(result)
}

/// Calculates NATR bands (upper and lower bounds).
/// 
/// This creates volatility bands around the current price using NATR.
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for ATR calculation
/// * `multiplier` - Multiplier for NATR bands (typically 2.0)
/// 
/// # Returns
/// 
/// Returns `Ok((Vec<f64>, Vec<f64>))` containing (upper_band, lower_band), or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::natr_bands;
/// 
/// let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
/// let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
/// 
/// let (upper, lower) = natr_bands(&high, &low, &close, 3, 2.0).unwrap();
/// assert_eq!(upper.len(), 5);
/// assert_eq!(lower.len(), 5);
/// ```
pub fn natr_bands(high: &[f64], low: &[f64], close: &[f64], period: usize, multiplier: f64) -> Result<(Vec<f64>, Vec<f64>), TAError> {
    let natr_values = natr(high, low, close, period)?;
    
    let mut upper_band = Vec::with_capacity(natr_values.len());
    let mut lower_band = Vec::with_capacity(natr_values.len());
    
    for (i, &natr_val) in natr_values.iter().enumerate() {
        if natr_val.is_nan() {
            upper_band.push(f64::NAN);
            lower_band.push(f64::NAN);
        } else {
            let band_width = close[i] * natr_val * multiplier / 100.0;
            upper_band.push(close[i] + band_width);
            lower_band.push(close[i] - band_width);
        }
    }
    
    Ok((upper_band, lower_band))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::OHLC;

    #[test]
    fn test_natr_basic() {
        let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
        let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
        
        let result = natr(&high, &low, &close, 3).unwrap();
        let atr_values = atr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First two values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Check NATR calculation: NATR = 100 * ATR / Close
        for (i, (&natr_val, &atr_val)) in result.iter().zip(atr_values.iter()).enumerate() {
            if atr_val.is_nan() {
                assert!(natr_val.is_nan());
            } else {
                let expected = 100.0 * atr_val / close[i];
                assert!((natr_val - expected).abs() < 1e-8);
            }
        }
    }

    #[test]
    fn test_natr_normalization() {
        // Test that NATR normalizes volatility across different price levels
        let high1 = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let low1 = vec![9.0, 10.0, 10.5, 10.0, 11.0];
        let close1 = vec![9.5, 10.5, 11.5, 10.5, 12.0];
        
        // Same percentage moves but 10x higher prices
        let high2 = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let low2 = vec![90.0, 100.0, 105.0, 100.0, 110.0];
        let close2 = vec![95.0, 105.0, 115.0, 105.0, 120.0];
        
        let natr1 = natr(&high1, &low1, &close1, 3).unwrap();
        let natr2 = natr(&high2, &low2, &close2, 3).unwrap();
        
        // NATR should be similar for similar percentage moves
        for (i, (&n1, &n2)) in natr1.iter().zip(natr2.iter()).enumerate() {
            if !n1.is_nan() && !n2.is_nan() {
                assert!((n1 - n2).abs() < 1e-6, "NATR values should be similar at index {}: {} vs {}", i, n1, n2);
            }
        }
    }

    #[test]
    fn test_natr_zero_close() {
        let high = vec![1.0, 2.0, 3.0];
        let low = vec![0.5, 1.0, 2.0];
        let close = vec![0.0, 1.5, 2.5]; // Zero close price
        
        let result = natr(&high, &low, &close, 2);
        // This should fail validation because close is outside high-low range
        assert!(result.is_err());
    }

    #[test]
    fn test_natr_insufficient_data() {
        let high = vec![10.0, 11.0];
        let low = vec![9.0, 10.0];
        let close = vec![9.5, 10.5];
        
        let result = natr(&high, &low, &close, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_natr_empty_data() {
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        let result = natr(&high, &low, &close, 14);
        assert!(result.is_err());
    }

    #[test]
    fn test_natr_zero_period() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = natr(&high, &low, &close, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_natr_period_1() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = natr(&high, &low, &close, 1).unwrap();
        
        // With period 1, NATR should be 100 * TR / Close
        assert_eq!(result.len(), 3);
        for &natr_val in &result {
            assert!(!natr_val.is_nan());
            assert!(natr_val >= 0.0);
        }
    }

    #[test]
    fn test_natr_ohlc() {
        let data = vec![
            OHLC { open: 92.0, high: 100.0, low: 90.0, close: 95.0 },
            OHLC { open: 95.0, high: 110.0, low: 100.0, close: 105.0 },
            OHLC { open: 105.0, high: 120.0, low: 105.0, close: 115.0 },
            OHLC { open: 115.0, high: 130.0, low: 110.0, close: 120.0 },
        ];
        
        let result = natr_ohlc(&data, 3).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(!result[2].is_nan());
        assert!(!result[3].is_nan());
    }

    #[test]
    fn test_natr_custom_alpha() {
        let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
        let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
        
        let result = natr_custom(&high, &low, &close, 3, 0.5).unwrap();
        
        assert_eq!(result.len(), 5);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(!result[2].is_nan());
    }

    #[test]
    fn test_natr_custom_invalid_alpha() {
        let high = vec![100.0, 110.0, 120.0];
        let low = vec![90.0, 100.0, 105.0];
        let close = vec![95.0, 105.0, 115.0];
        
        let result = natr_custom(&high, &low, &close, 2, 0.0);
        assert!(result.is_err());
        
        let result = natr_custom(&high, &low, &close, 2, 1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_natr_bands() {
        let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
        let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
        
        let (upper, lower) = natr_bands(&high, &low, &close, 3, 2.0).unwrap();
        let natr_values = natr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(upper.len(), 5);
        assert_eq!(lower.len(), 5);
        
        // Check band calculations
        for (i, (&natr_val, (&upper_val, &lower_val))) in natr_values.iter().zip(upper.iter().zip(lower.iter())).enumerate() {
            if natr_val.is_nan() {
                assert!(upper_val.is_nan());
                assert!(lower_val.is_nan());
            } else {
                let band_width = close[i] * natr_val * 2.0 / 100.0;
                let expected_upper = close[i] + band_width;
                let expected_lower = close[i] - band_width;
                
                assert!((upper_val - expected_upper).abs() < 1e-8);
                assert!((lower_val - expected_lower).abs() < 1e-8);
                assert!(upper_val > lower_val);
            }
        }
    }

    #[test]
    fn test_natr_real_market_data() {
        // Simulate real market data with typical NATR period of 14
        let high = vec![
            100.0, 102.0, 101.5, 103.0, 99.0, 101.0, 104.0, 102.5,
            105.0, 103.0, 106.0, 104.5, 107.0, 105.0, 108.0, 106.0
        ];
        let low = vec![
            98.0, 100.5, 99.0, 100.0, 96.0, 98.5, 101.0, 100.0,
            102.0, 101.0, 103.0, 102.0, 104.0, 103.0, 105.0, 104.0
        ];
        let close = vec![
            99.0, 101.0, 100.0, 102.0, 97.0, 100.0, 103.0, 101.5,
            104.0, 102.0, 105.0, 103.5, 106.0, 104.0, 107.0, 105.0
        ];
        
        let result = natr(&high, &low, &close, 14).unwrap();
        
        assert_eq!(result.len(), 16);
        
        // First 13 values should be NaN
        for i in 0..13 {
            assert!(result[i].is_nan());
        }
        
        // 14th value should be valid
        assert!(!result[13].is_nan());
        assert!(result[13] > 0.0);
        
        // NATR should be reasonable (typically 0.5% to 5% for normal markets)
        assert!(result[13] < 20.0); // Should be less than 20%
        
        // Last value should be valid
        assert!(!result[15].is_nan());
        assert!(result[15] > 0.0);
    }

    #[test]
    fn test_natr_constant_prices() {
        let high = vec![100.0; 10];
        let low = vec![100.0; 10];
        let close = vec![100.0; 10];
        
        let result = natr(&high, &low, &close, 5).unwrap();
        
        // NATR should be 0 for constant prices
        for i in 4..10 {
            assert!((result[i] - 0.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_natr_high_volatility() {
        // Test with high volatility scenario
        let high = vec![100.0, 120.0, 90.0, 110.0, 80.0, 130.0];
        let low = vec![80.0, 90.0, 70.0, 85.0, 60.0, 100.0];
        let close = vec![90.0, 100.0, 80.0, 95.0, 70.0, 115.0];
        
        let result = natr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(result.len(), 6);
        
        // NATR should be high for volatile market
        for i in 2..6 {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0);
            // High volatility should result in NATR > 10%
            assert!(result[i] > 10.0);
        }
    }

    #[test]
    fn test_natr_vs_atr_relationship() {
        let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
        let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
        
        let natr_values = natr(&high, &low, &close, 3).unwrap();
        let atr_values = atr(&high, &low, &close, 3).unwrap();
        
        // Verify relationship: NATR = 100 * ATR / Close
        for (i, (&natr_val, &atr_val)) in natr_values.iter().zip(atr_values.iter()).enumerate() {
            if !atr_val.is_nan() && close[i] != 0.0 {
                let expected_natr = 100.0 * atr_val / close[i];
                assert!((natr_val - expected_natr).abs() < 1e-8);
            }
        }
    }
}