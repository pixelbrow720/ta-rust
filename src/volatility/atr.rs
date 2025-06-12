//! Average True Range (ATR)
//! 
//! ATR is a volatility indicator that measures the average of true ranges over a specified period.
//! It uses Wilder's smoothing method (exponential moving average with alpha = 1/period).

use crate::common::{TAError, validate_period};
use crate::volatility::trange;

/// Calculates Average True Range using Wilder's smoothing method.
/// 
/// ATR = EMA(True Range, period) using Wilder's smoothing (alpha = 1/period)
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
/// Returns `Ok(Vec<f64>)` containing ATR values, or `Err(TAError)` on invalid input.
/// The first `period-1` values will be NaN as ATR needs time to stabilize.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::atr;
/// 
/// let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0];
/// let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5, 12.0];
/// let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0, 13.0];
/// 
/// let result = atr(&high, &low, &close, 3).unwrap();
/// assert_eq!(result.len(), 7);
/// ```
pub fn atr(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    // Validate that arrays have same length and are not empty
    if high.is_empty() || low.is_empty() || close.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TAError::mismatched_inputs("High, Low, and Close arrays must have the same length"));
    }
    
    validate_period(period, "period")?;
    
    let len = high.len();
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    
    // Calculate True Range first
    let tr_values = trange(high, low, close)?;
    
    let mut result = vec![f64::NAN; len];
    let alpha = 1.0 / period as f64;
    
    // Initialize ATR with SMA of first 'period' TR values
    let initial_atr: f64 = tr_values[0..period].iter().sum::<f64>() / period as f64;
    result[period - 1] = initial_atr;
    
    // Apply Wilder's smoothing for remaining values
    for i in period..len {
        result[i] = alpha * tr_values[i] + (1.0 - alpha) * result[i - 1];
    }
    
    Ok(result)
}

/// Calculates ATR using OHLC data structure.
/// 
/// # Arguments
/// 
/// * `ohlc` - Slice of OHLC data
/// * `period` - Period for ATR calculation
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ATR values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::common::OHLC;
/// use ta_rust::volatility::atr_ohlc;
/// 
/// let data = vec![
///     OHLC { open: 9.2, high: 10.0, low: 9.0, close: 9.5 },
///     OHLC { open: 9.5, high: 11.0, low: 10.0, close: 10.5 },
///     OHLC { open: 10.5, high: 12.0, low: 10.5, close: 11.5 },
///     OHLC { open: 11.5, high: 13.0, low: 11.0, close: 12.0 },
/// ];
/// 
/// let result = atr_ohlc(&data, 3).unwrap();
/// assert_eq!(result.len(), 4);
/// ```
pub fn atr_ohlc(ohlc: &[crate::common::types::OHLC], period: usize) -> Result<Vec<f64>, TAError> {
    if ohlc.is_empty() {
        return Err(TAError::invalid_input("OHLC data cannot be empty"));
    }
    
    let high: Vec<f64> = ohlc.iter().map(|x| x.high).collect();
    let low: Vec<f64> = ohlc.iter().map(|x| x.low).collect();
    let close: Vec<f64> = ohlc.iter().map(|x| x.close).collect();
    
    atr(&high, &low, &close, period)
}

/// Calculates ATR with custom smoothing factor.
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
/// Returns `Ok(Vec<f64>)` containing ATR values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::atr_custom;
/// 
/// let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
/// let low = vec![9.0, 10.0, 10.5, 10.0, 11.0];
/// let close = vec![9.5, 10.5, 11.5, 10.5, 12.0];
/// 
/// // Use faster smoothing (alpha = 0.2 instead of 1/14 ≈ 0.071)
/// let result = atr_custom(&high, &low, &close, 3, 0.2).unwrap();
/// assert_eq!(result.len(), 5);
/// ```
pub fn atr_custom(high: &[f64], low: &[f64], close: &[f64], period: usize, alpha: f64) -> Result<Vec<f64>, TAError> {
    // Validate that arrays have same length and are not empty
    if high.is_empty() || low.is_empty() || close.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TAError::mismatched_inputs("High, Low, and Close arrays must have the same length"));
    }
    
    validate_period(period, "period")?;
    
    if alpha <= 0.0 || alpha > 1.0 {
        return Err(TAError::invalid_input("Alpha must be between 0 and 1"));
    }
    
    let len = high.len();
    if len < period {
        return Err(TAError::insufficient_data(period, len));
    }
    
    // Calculate True Range first
    let tr_values = trange(high, low, close)?;
    
    let mut result = vec![f64::NAN; len];
    
    // Initialize ATR with SMA of first 'period' TR values
    let initial_atr: f64 = tr_values[0..period].iter().sum::<f64>() / period as f64;
    result[period - 1] = initial_atr;
    
    // Apply custom smoothing for remaining values
    for i in period..len {
        result[i] = alpha * tr_values[i] + (1.0 - alpha) * result[i - 1];
    }
    
    Ok(result)
}

/// Calculates ATR percentage relative to price.
/// 
/// This normalizes ATR by dividing by the close price, making it easier to compare
/// volatility across different price levels.
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for ATR calculation
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ATR percentage values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::atr_percent;
/// 
/// let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
/// let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
/// 
/// let result = atr_percent(&high, &low, &close, 3).unwrap();
/// assert_eq!(result.len(), 5);
/// ```
pub fn atr_percent(high: &[f64], low: &[f64], close: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    let atr_values = atr(high, low, close, period)?;
    
    let mut result = Vec::with_capacity(atr_values.len());
    
    for (i, &atr_val) in atr_values.iter().enumerate() {
        if atr_val.is_nan() || close[i] == 0.0 {
            result.push(f64::NAN);
        } else {
            result.push(100.0 * atr_val / close[i]);
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::OHLC;

    #[test]
    fn test_atr_basic() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0];
        
        let result = atr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First two values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Third value should be SMA of first 3 TR values
        let tr_values = trange(&high, &low, &close).unwrap();
        let expected_initial = (tr_values[0] + tr_values[1] + tr_values[2]) / 3.0;
        assert!((result[2] - expected_initial).abs() < 1e-8);
        
        // Subsequent values should use Wilder's smoothing
        assert!(!result[3].is_nan());
        assert!(!result[4].is_nan());
    }

    #[test]
    fn test_atr_wilder_smoothing() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0, 11.5];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0, 12.0];
        
        let result = atr(&high, &low, &close, 3).unwrap();
        let tr_values = trange(&high, &low, &close).unwrap();
        
        let alpha = 1.0 / 3.0;
        
        // Verify Wilder's smoothing calculation
        let initial_atr = (tr_values[0] + tr_values[1] + tr_values[2]) / 3.0;
        assert!((result[2] - initial_atr).abs() < 1e-8);
        
        let expected_atr_3 = alpha * tr_values[3] + (1.0 - alpha) * result[2];
        assert!((result[3] - expected_atr_3).abs() < 1e-8);
        
        let expected_atr_4 = alpha * tr_values[4] + (1.0 - alpha) * result[3];
        assert!((result[4] - expected_atr_4).abs() < 1e-8);
    }

    #[test]
    fn test_atr_insufficient_data() {
        let high = vec![10.0, 11.0];
        let low = vec![9.0, 10.0];
        let close = vec![9.5, 10.5];
        
        let result = atr(&high, &low, &close, 5);
        assert!(result.is_err());
        
        if let Err(TAError::InsufficientData { required, provided }) = result {
            assert_eq!(required, 5);
            assert_eq!(provided, 2);
        } else {
            panic!("Expected InsufficientData error");
        }
    }

    #[test]
    fn test_atr_empty_data() {
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        let result = atr(&high, &low, &close, 14);
        assert!(result.is_err());
    }

    #[test]
    fn test_atr_zero_period() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = atr(&high, &low, &close, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_atr_period_1() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = atr(&high, &low, &close, 1).unwrap();
        let tr_values = trange(&high, &low, &close).unwrap();
        
        // With period 1, ATR should equal TR values
        for (i, &atr_val) in result.iter().enumerate() {
            assert!((atr_val - tr_values[i]).abs() < 1e-8);
        }
    }

    #[test]
    fn test_atr_ohlc() {
        let data = vec![
            OHLC { open: 9.2, high: 10.0, low: 9.0, close: 9.5 },
            OHLC { open: 9.5, high: 11.0, low: 10.0, close: 10.5 },
            OHLC { open: 10.5, high: 12.0, low: 10.5, close: 11.5 },
            OHLC { open: 11.5, high: 13.0, low: 11.0, close: 12.0 },
        ];
        
        let result = atr_ohlc(&data, 3).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(!result[2].is_nan());
        assert!(!result[3].is_nan());
    }

    #[test]
    fn test_atr_custom_alpha() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0];
        
        let result = atr_custom(&high, &low, &close, 3, 0.5).unwrap();
        
        assert_eq!(result.len(), 5);
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!(!result[2].is_nan());
    }

    #[test]
    fn test_atr_custom_invalid_alpha() {
        let high = vec![10.0, 11.0, 12.0];
        let low = vec![9.0, 10.0, 10.5];
        let close = vec![9.5, 10.5, 11.5];
        
        let result = atr_custom(&high, &low, &close, 2, 0.0);
        assert!(result.is_err());
        
        let result = atr_custom(&high, &low, &close, 2, 1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_atr_percent() {
        let high = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let low = vec![90.0, 100.0, 105.0, 100.0, 110.0];
        let close = vec![95.0, 105.0, 115.0, 105.0, 120.0];
        
        let result = atr_percent(&high, &low, &close, 3).unwrap();
        let atr_values = atr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // Check percentage calculation
        for (i, (&atr_pct, &atr_val)) in result.iter().zip(atr_values.iter()).enumerate() {
            if atr_val.is_nan() {
                assert!(atr_pct.is_nan());
            } else {
                let expected = 100.0 * atr_val / close[i];
                assert!((atr_pct - expected).abs() < 1e-8);
            }
        }
    }

    #[test]
    fn test_atr_real_market_data() {
        // Simulate real market data with typical ATR period of 14
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
        
        let result = atr(&high, &low, &close, 14).unwrap();
        
        assert_eq!(result.len(), 16);
        
        // First 13 values should be NaN
        for i in 0..13 {
            assert!(result[i].is_nan());
        }
        
        // 14th value should be valid
        assert!(!result[13].is_nan());
        assert!(result[13] > 0.0);
        
        // Last value should be valid
        assert!(!result[15].is_nan());
        assert!(result[15] > 0.0);
    }

    #[test]
    fn test_atr_constant_prices() {
        let high = vec![10.0; 10];
        let low = vec![10.0; 10];
        let close = vec![10.0; 10];
        
        let result = atr(&high, &low, &close, 5).unwrap();
        
        // ATR should be 0 for constant prices
        for i in 4..10 {
            assert!((result[i] - 0.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_atr_trending_market() {
        // Simulate trending market with increasing volatility
        let high = vec![10.0, 11.0, 13.0, 16.0, 20.0, 25.0];
        let low = vec![9.0, 10.0, 11.0, 13.0, 16.0, 20.0];
        let close = vec![9.5, 10.5, 12.0, 15.0, 18.0, 23.0];
        
        let result = atr(&high, &low, &close, 3).unwrap();
        
        assert_eq!(result.len(), 6);
        
        // ATR should generally increase with increasing volatility
        assert!(!result[2].is_nan());
        assert!(!result[3].is_nan());
        assert!(!result[4].is_nan());
        assert!(!result[5].is_nan());
        
        // Later ATR values should be higher due to increasing volatility
        assert!(result[5] > result[2]);
    }
}