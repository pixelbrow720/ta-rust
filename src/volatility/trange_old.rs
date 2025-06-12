//! True Range (TRANGE)
//! 
//! True Range is a measure of volatility that captures the full range of price movement
//! for a given period, including gaps. It's the foundation for calculating ATR.

use crate::common::TAError;

/// Calculates True Range for each period.
/// 
/// True Range is the maximum of:
/// - High - Low
/// - |High - Previous Close|
/// - |Low - Previous Close|
/// 
/// # Arguments
/// 
/// * `high` - High prices
/// * `low` - Low prices  
/// * `close` - Close prices
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing True Range values, or `Err(TAError)` on invalid input.
/// The first value will be High[0] - Low[0] since there's no previous close.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::volatility::trange;
/// 
/// let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
/// let low = vec![9.0, 10.0, 10.5, 10.0, 11.0];
/// let close = vec![9.5, 10.5, 11.5, 10.5, 12.0];
/// 
/// let result = trange(&high, &low, &close).unwrap();
/// assert_eq!(result.len(), 5);
/// ```
pub fn trange(high: &[f64], low: &[f64], close: &[f64]) -> Result<Vec<f64>, TAError> {
    // Validate that arrays have same length and are not empty
    if high.is_empty() || low.is_empty() || close.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if high.len() != low.len() || high.len() != close.len() {
        return Err(TAError::mismatched_inputs("High, Low, and Close arrays must have the same length"));
    }
    
    // Validate OHLC constraints (we'll use close as open for first bar)
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
        if h < c || l > c {
            return Err(TAError::invalid_input(format!(
                "Close ({}) is outside High-Low range [{}, {}] at index {}",
                c, l, h, i
            )));
        }
    }
    
    let len = high.len();
    let mut result = Vec::with_capacity(len);
    
    // First value: High[0] - Low[0] (no previous close available)
    result.push(high[0] - low[0]);
    
    // Calculate True Range for remaining periods
    for i in 1..len {
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        
        let tr = hl.max(hc).max(lc);
        result.push(tr);
    }
    
    Ok(result)
}

/// Calculates True Range using OHLC data structure.
/// 
/// # Arguments
/// 
/// * `ohlc` - Slice of OHLC data
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing True Range values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::common::OHLC;
/// use ta_rust::volatility::trange_ohlc;
/// 
/// let data = vec![
///     OHLC { open: 9.2, high: 10.0, low: 9.0, close: 9.5 },
///     OHLC { open: 9.5, high: 11.0, low: 10.0, close: 10.5 },
///     OHLC { open: 10.5, high: 12.0, low: 10.5, close: 11.5 },
/// ];
/// 
/// let result = trange_ohlc(&data).unwrap();
/// assert_eq!(result.len(), 3);
/// ```
pub fn trange_ohlc(ohlc: &[crate::common::types::OHLC]) -> Result<Vec<f64>, TAError> {
    if ohlc.is_empty() {
        return Err(TAError::invalid_input("OHLC data cannot be empty"));
    }
    
    let high: Vec<f64> = ohlc.iter().map(|x| x.high).collect();
    let low: Vec<f64> = ohlc.iter().map(|x| x.low).collect();
    let close: Vec<f64> = ohlc.iter().map(|x| x.close).collect();
    
    trange(&high, &low, &close)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::types::OHLC;

    #[test]
    fn test_trange_basic() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First TR = High[0] - Low[0] = 10.0 - 9.0 = 1.0
        assert!((result[0] - 1.0).abs() < 1e-8);
        
        // Second TR = max(11.0-10.0, |11.0-9.5|, |10.0-9.5|) = max(1.0, 1.5, 0.5) = 1.5
        assert!((result[1] - 1.5).abs() < 1e-8);
        
        // Third TR = max(12.0-10.5, |12.0-10.5|, |10.5-10.5|) = max(1.5, 1.5, 0.0) = 1.5
        assert!((result[2] - 1.5).abs() < 1e-8);
    }

    #[test]
    fn test_trange_gap_up() {
        // Test gap up scenario
        let high = vec![10.0, 15.0];
        let low = vec![9.0, 14.0];
        let close = vec![9.5, 14.5];
        
        let result = trange(&high, &low, &close).unwrap();
        
        // First TR = 10.0 - 9.0 = 1.0
        assert!((result[0] - 1.0).abs() < 1e-8);
        
        // Second TR = max(15.0-14.0, |15.0-9.5|, |14.0-9.5|) = max(1.0, 5.5, 4.5) = 5.5
        assert!((result[1] - 5.5).abs() < 1e-8);
    }

    #[test]
    fn test_trange_gap_down() {
        // Test gap down scenario
        let high = vec![10.0, 6.0];
        let low = vec![9.0, 5.0];
        let close = vec![9.5, 5.5];
        
        let result = trange(&high, &low, &close).unwrap();
        
        // First TR = 10.0 - 9.0 = 1.0
        assert!((result[0] - 1.0).abs() < 1e-8);
        
        // Second TR = max(6.0-5.0, |6.0-9.5|, |5.0-9.5|) = max(1.0, 3.5, 4.5) = 4.5
        assert!((result[1] - 4.5).abs() < 1e-8);
    }

    #[test]
    fn test_trange_single_value() {
        let high = vec![10.0];
        let low = vec![9.0];
        let close = vec![9.5];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 1);
        assert!((result[0] - 1.0).abs() < 1e-8);
    }

    #[test]
    fn test_trange_empty_input() {
        let high = vec![];
        let low = vec![];
        let close = vec![];
        
        let result = trange(&high, &low, &close);
        assert!(result.is_err());
    }

    #[test]
    fn test_trange_mismatched_lengths() {
        let high = vec![10.0, 11.0];
        let low = vec![9.0];
        let close = vec![9.5, 10.5];
        
        let result = trange(&high, &low, &close);
        assert!(result.is_err());
    }

    #[test]
    fn test_trange_ohlc() {
        let data = vec![
            OHLC { open: 9.2, high: 10.0, low: 9.0, close: 9.5 },
            OHLC { open: 9.5, high: 11.0, low: 10.0, close: 10.5 },
            OHLC { open: 10.5, high: 12.0, low: 10.5, close: 11.5 },
        ];
        
        let result = trange_ohlc(&data).unwrap();
        
        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.0).abs() < 1e-8);
        assert!((result[1] - 1.5).abs() < 1e-8);
        assert!((result[2] - 1.5).abs() < 1e-8);
    }

    #[test]
    fn test_trange_ohlc_empty() {
        let data = vec![];
        let result = trange_ohlc(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_trange_real_market_data() {
        // Real market scenario with various price movements
        let high = vec![100.0, 102.0, 101.5, 103.0, 99.0, 101.0];
        let low = vec![98.0, 100.5, 99.0, 100.0, 96.0, 98.5];
        let close = vec![99.0, 101.0, 100.0, 102.0, 97.0, 100.0];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 6);
        
        // Verify each calculation
        assert!((result[0] - 2.0).abs() < 1e-8); // 100.0 - 98.0
        
        // Second: max(102.0-100.5, |102.0-99.0|, |100.5-99.0|) = max(1.5, 3.0, 1.5) = 3.0
        assert!((result[1] - 3.0).abs() < 1e-8);
        
        // Third: max(101.5-99.0, |101.5-101.0|, |99.0-101.0|) = max(2.5, 0.5, 2.0) = 2.5
        assert!((result[2] - 2.5).abs() < 1e-8);
    }

    #[test]
    fn test_trange_constant_prices() {
        let high = vec![10.0, 10.0, 10.0];
        let low = vec![10.0, 10.0, 10.0];
        let close = vec![10.0, 10.0, 10.0];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 3);
        for &tr in &result {
            assert!((tr - 0.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_trange_extreme_volatility() {
        // Test with extreme price movements
        let high = vec![100.0, 200.0, 50.0];
        let low = vec![90.0, 150.0, 40.0];
        let close = vec![95.0, 180.0, 45.0];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 3);
        
        // First: 100.0 - 90.0 = 10.0
        assert!((result[0] - 10.0).abs() < 1e-8);
        
        // Second: max(200.0-150.0, |200.0-95.0|, |150.0-95.0|) = max(50.0, 105.0, 55.0) = 105.0
        assert!((result[1] - 105.0).abs() < 1e-8);
        
        // Third: max(50.0-40.0, |50.0-180.0|, |40.0-180.0|) = max(10.0, 130.0, 140.0) = 140.0
        assert!((result[2] - 140.0).abs() < 1e-8);
    }
}