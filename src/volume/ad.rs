//! Chaikin A/D Line (Accumulation/Distribution Line)
//!
//! The A/D Line is a volume-based indicator designed to measure the cumulative flow of money
//! into and out of a security. It uses the relationship between closing price and the trading range.

use crate::common::{TAError, TAResult};

/// Chaikin A/D Line (Accumulation/Distribution Line)
///
/// The A/D Line combines price and volume to show how much of the volume is associated with
/// rises or falls in price. It's calculated using the Close Location Value (CLV).
///
/// # Formula
/// ```text
/// CLV = ((Close - Low) - (High - Close)) / (High - Low)
/// AD = AD[prev] + (CLV × Volume)
/// ```
///
/// # Arguments
/// * `high` - Slice of high prices
/// * `low` - Slice of low prices  
/// * `close` - Slice of closing prices
/// * `volume` - Slice of volume data
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of A/D Line values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::volume::ad;
///
/// let high = vec![12.0, 13.0, 12.5, 14.0, 13.5];
/// let low = vec![10.0, 11.0, 10.5, 12.0, 11.5];
/// let close = vec![11.0, 12.0, 11.5, 13.0, 12.5];
/// let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];
/// let result = ad(&high, &low, &close, &volume).unwrap();
/// ```
pub fn ad(high: &[f64], low: &[f64], close: &[f64], volume: &[f64]) -> TAResult<Vec<f64>> {
    if high.is_empty() || low.is_empty() || close.is_empty() || volume.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    let len = high.len();
    if len != low.len() || len != close.len() || len != volume.len() {
        return Err(TAError::mismatched_inputs("All input arrays must have the same length"));
    }
    
    let mut result = vec![0.0; len];
    let mut ad_value = 0.0;
    
    for i in 0..len {
        let h = high[i];
        let l = low[i];
        let c = close[i];
        let v = volume[i];
        
        // Calculate Close Location Value (CLV)
        let clv = if (h - l).abs() < f64::EPSILON {
            // If high equals low (no range), CLV is 0
            0.0
        } else {
            ((c - l) - (h - c)) / (h - l)
        };
        
        // Update A/D Line
        ad_value += clv * v;
        result[i] = ad_value;
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ad_basic() {
        let high = vec![12.0, 13.0, 12.5, 14.0, 13.5];
        let low = vec![10.0, 11.0, 10.5, 12.0, 11.5];
        let close = vec![11.0, 12.0, 11.5, 13.0, 12.5];
        let volume = vec![1000.0, 1500.0, 800.0, 2000.0, 1200.0];
        
        let result = ad(&high, &low, &close, &volume).unwrap();
        assert_eq!(result.len(), 5);
        
        // First value: CLV = ((11-10) - (12-11)) / (12-10) = (1-1)/2 = 0
        // AD[0] = 0 + 0 * 1000 = 0
        assert_eq!(result[0], 0.0);
        
        // Second value: CLV = ((12-11) - (13-12)) / (13-11) = (1-1)/2 = 0
        // AD[1] = 0 + 0 * 1500 = 0
        assert_eq!(result[1], 0.0);
    }

    #[test]
    fn test_ad_close_at_high() {
        // When close equals high, CLV should be 1
        let high = vec![10.0];
        let low = vec![8.0];
        let close = vec![10.0];  // Close at high
        let volume = vec![1000.0];
        
        let result = ad(&high, &low, &close, &volume).unwrap();
        
        // CLV = ((10-8) - (10-10)) / (10-8) = (2-0)/2 = 1
        // AD = 0 + 1 * 1000 = 1000
        assert_eq!(result[0], 1000.0);
    }

    #[test]
    fn test_ad_close_at_low() {
        // When close equals low, CLV should be -1
        let high = vec![10.0];
        let low = vec![8.0];
        let close = vec![8.0];  // Close at low
        let volume = vec![1000.0];
        
        let result = ad(&high, &low, &close, &volume).unwrap();
        
        // CLV = ((8-8) - (10-8)) / (10-8) = (0-2)/2 = -1
        // AD = 0 + (-1) * 1000 = -1000
        assert_eq!(result[0], -1000.0);
    }

    #[test]
    fn test_ad_no_range() {
        // When high equals low, CLV should be 0
        let high = vec![10.0];
        let low = vec![10.0];
        let close = vec![10.0];
        let volume = vec![1000.0];
        
        let result = ad(&high, &low, &close, &volume).unwrap();
        assert_eq!(result[0], 0.0);
    }

    #[test]
    fn test_ad_empty_input() {
        let high: Vec<f64> = vec![];
        let low: Vec<f64> = vec![];
        let close: Vec<f64> = vec![];
        let volume: Vec<f64> = vec![];
        assert!(ad(&high, &low, &close, &volume).is_err());
    }

    #[test]
    fn test_ad_mismatched_lengths() {
        let high = vec![10.0, 11.0];
        let low = vec![8.0];
        let close = vec![9.0, 10.0];
        let volume = vec![1000.0, 1500.0];
        assert!(ad(&high, &low, &close, &volume).is_err());
    }

    #[test]
    fn test_ad_accumulation() {
        // Test that A/D Line accumulates over time
        let high = vec![10.0, 11.0];
        let low = vec![8.0, 9.0];
        let close = vec![10.0, 11.0];  // Both close at high
        let volume = vec![1000.0, 1000.0];
        
        let result = ad(&high, &low, &close, &volume).unwrap();
        
        // First: CLV = 1, AD = 1000
        // Second: CLV = 1, AD = 1000 + 1000 = 2000
        assert_eq!(result[0], 1000.0);
        assert_eq!(result[1], 2000.0);
    }
}