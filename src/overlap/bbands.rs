//! Bollinger Bands (BBANDS)
//!
//! Bollinger Bands consist of a middle band (SMA) and two outer bands that are standard
//! deviations away from the middle band. They are used to measure volatility and identify
//! overbought/oversold conditions.

use crate::common::{TAError, TAResult};
use crate::overlap::sma;

/// Bollinger Bands result structure
#[derive(Debug, Clone)]
pub struct BollingerBands {
    /// Upper band values
    pub upper: Vec<f64>,
    /// Middle band values (SMA)
    pub middle: Vec<f64>,
    /// Lower band values
    pub lower: Vec<f64>,
}

/// Bollinger Bands (BBANDS)
///
/// Bollinger Bands are volatility bands placed above and below a moving average.
/// The bands automatically widen when volatility increases and narrow when volatility decreases.
///
/// # Formula
/// ```text
/// Middle Band = SMA(Close, period)
/// Standard Deviation = √(Σ(Close[i] - Middle Band)² / period)
/// Upper Band = Middle Band + (std_dev_multiplier × Standard Deviation)
/// Lower Band = Middle Band - (std_dev_multiplier × Standard Deviation)
/// ```
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `period` - Period for the moving average and standard deviation calculation
/// * `std_dev_multiplier` - Number of standard deviations for the bands (typically 2.0)
///
/// # Returns
/// * `Ok(BollingerBands)` - Structure containing upper, middle, and lower bands
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::overlap::bbands;
///
/// let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
/// let result = bbands(&close, 5, 2.0).unwrap();
/// // result.middle contains the 5-period SMA
/// // result.upper contains middle + 2 * standard deviation
/// // result.lower contains middle - 2 * standard deviation
/// ```
pub fn bbands(close: &[f64], period: usize, std_dev_multiplier: f64) -> TAResult<BollingerBands> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if period > close.len() {
        return Err(TAError::insufficient_data(period, close.len()));
    }
    
    if std_dev_multiplier < 0.0 {
        return Err(TAError::invalid_parameter("std_dev_multiplier", "must be non-negative"));
    }
    
    let len = close.len();
    
    // Calculate middle band (SMA)
    let middle = sma(close, period)?;
    
    // Initialize result vectors
    let mut upper = vec![f64::NAN; len];
    let mut lower = vec![f64::NAN; len];
    
    // Calculate standard deviation and bands for each valid point
    for i in (period - 1)..len {
        if !middle[i].is_nan() {
            // Calculate standard deviation for the current window
            let start_idx = i + 1 - period;
            let window = &close[start_idx..=i];
            let mean = middle[i];
            
            // Calculate variance
            let variance = window.iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>() / period as f64;
            
            let std_dev = variance.sqrt();
            
            // Calculate bands
            upper[i] = mean + std_dev_multiplier * std_dev;
            lower[i] = mean - std_dev_multiplier * std_dev;
        }
    }
    
    Ok(BollingerBands {
        upper,
        middle,
        lower,
    })
}

/// Bollinger Bands with default parameters (20 period, 2.0 standard deviations)
///
/// This is a convenience function using the most common Bollinger Bands settings.
///
/// # Arguments
/// * `close` - Slice of closing prices
///
/// # Returns
/// * `Ok(BollingerBands)` - Structure containing upper, middle, and lower bands
/// * `Err(TAError)` - Error if inputs are invalid
pub fn bbands_default(close: &[f64]) -> TAResult<BollingerBands> {
    bbands(close, 20, 2.0)
}

/// Calculate Bollinger Band %B
///
/// %B indicates where the price is in relation to the bands.
/// %B = (Close - Lower Band) / (Upper Band - Lower Band)
///
/// # Arguments
/// * `close` - Slice of closing prices
/// * `bands` - Bollinger Bands structure
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of %B values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn bbands_percent_b(close: &[f64], bands: &BollingerBands) -> TAResult<Vec<f64>> {
    if close.is_empty() {
        return Err(TAError::invalid_input("Close prices cannot be empty"));
    }
    
    if close.len() != bands.upper.len() || close.len() != bands.lower.len() {
        return Err(TAError::mismatched_inputs("Close prices and bands must have the same length"));
    }
    
    let len = close.len();
    let mut result = vec![f64::NAN; len];
    
    for i in 0..len {
        if !bands.upper[i].is_nan() && !bands.lower[i].is_nan() {
            let band_width = bands.upper[i] - bands.lower[i];
            if band_width.abs() > f64::EPSILON {
                result[i] = (close[i] - bands.lower[i]) / band_width;
            }
        }
    }
    
    Ok(result)
}

/// Calculate Bollinger Band Width
///
/// Band Width measures the width of the bands relative to the middle band.
/// Width = (Upper Band - Lower Band) / Middle Band
///
/// # Arguments
/// * `bands` - Bollinger Bands structure
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of band width values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn bbands_width(bands: &BollingerBands) -> TAResult<Vec<f64>> {
    let len = bands.upper.len();
    let mut result = vec![f64::NAN; len];
    
    for i in 0..len {
        if !bands.upper[i].is_nan() && !bands.lower[i].is_nan() && !bands.middle[i].is_nan() {
            if bands.middle[i].abs() > f64::EPSILON {
                result[i] = (bands.upper[i] - bands.lower[i]) / bands.middle[i];
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
    fn test_bbands_basic() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let result = bbands(&close, 5, 2.0).unwrap();
        
        assert_eq!(result.upper.len(), 10);
        assert_eq!(result.middle.len(), 10);
        assert_eq!(result.lower.len(), 10);
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(result.upper[i].is_nan());
            assert!(result.middle[i].is_nan());
            assert!(result.lower[i].is_nan());
        }
        
        // Values from index 4 onwards should be valid
        for i in 4..10 {
            assert!(!result.upper[i].is_nan());
            assert!(!result.middle[i].is_nan());
            assert!(!result.lower[i].is_nan());
            
            // Upper should be greater than middle, middle greater than lower
            assert!(result.upper[i] > result.middle[i]);
            assert!(result.middle[i] > result.lower[i]);
        }
    }

    #[test]
    fn test_bbands_default() {
        let close = vec![20.0; 25];  // Need at least 20 values for default period
        let result = bbands_default(&close).unwrap();
        
        assert_eq!(result.upper.len(), 25);
        assert_eq!(result.middle.len(), 25);
        assert_eq!(result.lower.len(), 25);
        
        // With constant prices, middle should equal close price, and bands should be equal to middle
        for i in 19..25 {
            assert_float_eq!(result.middle[i], 20.0, 1e-10);
            assert_float_eq!(result.upper[i], 20.0, 1e-10);  // No volatility
            assert_float_eq!(result.lower[i], 20.0, 1e-10);  // No volatility
        }
    }

    #[test]
    fn test_bbands_percent_b() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let bands = bbands(&close, 5, 2.0).unwrap();
        let percent_b = bbands_percent_b(&close, &bands).unwrap();
        
        assert_eq!(percent_b.len(), 10);
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(percent_b[i].is_nan());
        }
        
        // Valid values should be between 0 and 1 for prices within bands
        for i in 4..10 {
            assert!(!percent_b[i].is_nan());
        }
    }

    #[test]
    fn test_bbands_width() {
        let close = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let bands = bbands(&close, 5, 2.0).unwrap();
        let width = bbands_width(&bands).unwrap();
        
        assert_eq!(width.len(), 10);
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(width[i].is_nan());
        }
        
        // Valid values should be positive
        for i in 4..10 {
            assert!(!width[i].is_nan());
            assert!(width[i] >= 0.0);
        }
    }

    #[test]
    fn test_bbands_zero_volatility() {
        let close = vec![20.0; 10];  // Constant prices
        let result = bbands(&close, 5, 2.0).unwrap();
        
        // With zero volatility, all bands should be equal to the middle band
        for i in 4..10 {
            assert_float_eq!(result.upper[i], result.middle[i], 1e-10);
            assert_float_eq!(result.lower[i], result.middle[i], 1e-10);
            assert_float_eq!(result.middle[i], 20.0, 1e-10);
        }
    }

    #[test]
    fn test_bbands_invalid_input() {
        let close: Vec<f64> = vec![];
        assert!(bbands(&close, 5, 2.0).is_err());
        
        let close = vec![20.0, 21.0];
        assert!(bbands(&close, 0, 2.0).is_err());  // Zero period
        assert!(bbands(&close, 5, 2.0).is_err());  // Period > data length
        assert!(bbands(&close, 2, -1.0).is_err()); // Negative multiplier
    }

    #[test]
    fn test_bbands_single_period() {
        let close = vec![20.0, 21.0, 22.0];
        let result = bbands(&close, 1, 2.0).unwrap();
        
        // With period 1, middle band equals close price, and bands equal middle (no variance)
        for i in 0..3 {
            assert_float_eq!(result.middle[i], close[i], 1e-10);
            assert_float_eq!(result.upper[i], close[i], 1e-10);
            assert_float_eq!(result.lower[i], close[i], 1e-10);
        }
    }
}