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
/// The first value will be NaN since there's no previous close (following TA-Lib convention).
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
    
    let len = high.len();
    let mut result = Vec::with_capacity(len);
    
    // First value: High - Low (no previous close available)
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

    #[test]
    fn test_trange_basic() {
        let high = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let low = vec![9.0, 10.0, 10.5, 10.0, 11.0];
        let close = vec![9.5, 10.5, 11.5, 10.5, 12.0];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First TR = High - Low (no previous close)
        assert!((result[0] - (10.0 - 9.0)).abs() < 1e-8);
        
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
        
        // First TR = High - Low
        assert!((result[0] - (10.0 - 9.0)).abs() < 1e-8);
        
        // Second TR = max(15.0-14.0, |15.0-9.5|, |14.0-9.5|) = max(1.0, 5.5, 4.5) = 5.5
        assert!((result[1] - 5.5).abs() < 1e-8);
    }

    #[test]
    fn test_trange_single_value() {
        let high = vec![10.0];
        let low = vec![9.0];
        let close = vec![9.5];
        
        let result = trange(&high, &low, &close).unwrap();
        
        assert_eq!(result.len(), 1);
        assert!((result[0] - (10.0 - 9.0)).abs() < 1e-8);
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
}