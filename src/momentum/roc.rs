//! Rate of Change (ROC)
//! 
//! ROC measures the percentage change in price over a specified period.
//! It's calculated as ((Price[today] / Price[n periods ago]) - 1) * 100.

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Rate of Change as a percentage.
/// 
/// ROC = ((Price[today] / Price[n periods ago]) - 1) * 100
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ROC percentage values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as ROC needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::roc;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let result = roc(&prices, 2).unwrap();
/// assert_eq!(result.len(), 5);
/// // result[2] = ((120.0 / 100.0) - 1) * 100 = 20.0%
/// ```
pub fn roc(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate ROC starting from period index
    for i in period..len {
        if prices[i - period] == 0.0 {
            result[i] = f64::NAN;
        } else {
            result[i] = ((prices[i] / prices[i - period]) - 1.0) * 100.0;
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roc_basic() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let result = roc(&prices, 2).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Check calculations
        assert!((result[2] - 20.0).abs() < 1e-8); // (120/100 - 1) * 100 = 20%
        assert!((result[3] - 4.545454545454546).abs() < 1e-8); // (115/110 - 1) * 100 ≈ 4.55%
        assert!((result[4] - 8.333333333333334).abs() < 1e-8); // (130/120 - 1) * 100 ≈ 8.33%
    }

    #[test]
    fn test_roc_zero_price() {
        let prices = vec![0.0, 110.0, 120.0];
        let result = roc(&prices, 1).unwrap();
        
        // Should handle zero price gracefully
        assert!(result[1].is_nan()); // Division by zero
    }

    #[test]
    fn test_roc_negative_change() {
        let prices = vec![100.0, 90.0, 80.0];
        let result = roc(&prices, 1).unwrap();
        
        assert!((result[1] - (-10.0)).abs() < 1e-8); // (90/100 - 1) * 100 = -10%
        assert!((result[2] - (-11.111111111111114)).abs() < 1e-8); // (80/90 - 1) * 100 ≈ -11.11%
    }
}