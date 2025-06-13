//! On Balance Volume (OBV)
//!
//! OBV is a momentum indicator that uses volume flow to predict changes in stock price.
//! It adds volume on up days and subtracts volume on down days.

use crate::common::{TAError, TAResult};

/// On Balance Volume (OBV)
///
/// OBV is calculated by adding volume on days when the closing price is higher than the previous
/// closing price and subtracting volume on days when the closing price is lower.
///
/// # Formula
/// ```text
/// If Close > Close[prev]: OBV = OBV[prev] + Volume
/// If Close < Close[prev]: OBV = OBV[prev] - Volume  
/// If Close = Close[prev]: OBV = OBV[prev]
/// ```
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `volume` - Slice of volume data
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of OBV values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::volume::obv;
///
/// let close = vec![10.0, 11.0, 10.5, 12.0, 11.5];
/// let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];
/// let result = obv(&close, &volume).unwrap();
/// // result[0] = 0.0 (initial)
/// // result[1] = 1500.0 (close up, add volume)
/// // result[2] = 700.0 (close down, subtract volume)
/// // result[3] = 2700.0 (close up, add volume)
/// // result[4] = 1500.0 (close down, subtract volume)
/// ```
pub fn obv(close: &[f64], volume: &[f64]) -> TAResult<Vec<f64>> {
    if close.is_empty() || volume.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if close.len() != volume.len() {
        return Err(TAError::mismatched_inputs("Close and volume arrays must have the same length"));
    }
    
    let len = close.len();
    let mut result = vec![0.0; len];
    
    // First value is the first volume value (TA-Lib compatible)
    result[0] = volume[0];
    
    for i in 1..len {
        if close[i] > close[i - 1] {
            // Price up: add volume
            result[i] = result[i - 1] + volume[i];
        } else if close[i] < close[i - 1] {
            // Price down: subtract volume
            result[i] = result[i - 1] - volume[i];
        } else {
            // Price unchanged: keep same OBV
            result[i] = result[i - 1];
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_obv_basic() {
        let close = vec![10.0, 11.0, 10.5, 12.0, 11.5];
        let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];
        let result = obv(&close, &volume).unwrap();
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1000.0);  // First volume value
        assert_eq!(result[1], 2500.0);  // 1000 + 1500 (price up)
        assert_eq!(result[2], 1700.0);  // 2500 - 800 (price down)
        assert_eq!(result[3], 3700.0);  // 1700 + 2000 (price up)
        assert_eq!(result[4], 2500.0);  // 3700 - 1200 (price down)
    }

    #[test]
    fn test_obv_unchanged_price() {
        let close = vec![10.0, 10.0, 10.0];
        let volume = vec![1000.0, 1500.0, 800.0];
        let result = obv(&close, &volume).unwrap();
        
        assert_eq!(result[0], 1000.0);  // First volume value
        assert_eq!(result[1], 1000.0);  // Price unchanged
        assert_eq!(result[2], 1000.0);  // Price unchanged
    }

    #[test]
    fn test_obv_empty_input() {
        let close: Vec<f64> = vec![];
        let volume: Vec<f64> = vec![];
        assert!(obv(&close, &volume).is_err());
    }

    #[test]
    fn test_obv_mismatched_lengths() {
        let close = vec![10.0, 11.0];
        let volume = vec![1000.0];
        assert!(obv(&close, &volume).is_err());
    }

    #[test]
    fn test_obv_single_value() {
        let close = vec![10.0];
        let volume = vec![1000.0];
        let result = obv(&close, &volume).unwrap();
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 1000.0);  // First volume value
    }
}