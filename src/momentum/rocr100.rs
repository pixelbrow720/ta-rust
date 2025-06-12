//! Rate of Change Ratio 100 scale (ROCR100)
//! 
//! ROCR100 measures the ratio of current price to price n periods ago, scaled by 100.
//! It's calculated as (Price[today] / Price[n periods ago]) * 100.

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Rate of Change Ratio on 100 scale.
/// 
/// ROCR100 = (Price[today] / Price[n periods ago]) * 100
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ROCR100 values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as ROCR100 needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rocr100;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let result = rocr100(&prices, 2).unwrap();
/// assert_eq!(result.len(), 5);
/// // result[2] = (120.0 / 100.0) * 100 = 120.0
/// ```
pub fn rocr100(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate ROCR100 starting from period index
    for i in period..len {
        if prices[i - period] == 0.0 {
            result[i] = f64::NAN;
        } else {
            result[i] = (prices[i] / prices[i - period]) * 100.0;
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocr100_basic() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let result = rocr100(&prices, 2).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Check calculations
        assert!((result[2] - 120.0).abs() < 1e-8); // (120/100) * 100 = 120
        assert!((result[3] - 104.54545454545455).abs() < 1e-8); // (115/110) * 100 ≈ 104.55
        assert!((result[4] - 108.33333333333333).abs() < 1e-8); // (130/120) * 100 ≈ 108.33
    }
}