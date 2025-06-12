//! Rate of Change Ratio (ROCR)
//! 
//! ROCR measures the ratio of current price to price n periods ago.
//! It's calculated as Price[today] / Price[n periods ago].

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Rate of Change Ratio.
/// 
/// ROCR = Price[today] / Price[n periods ago]
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ROCR values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as ROCR needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rocr;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let result = rocr(&prices, 2).unwrap();
/// assert_eq!(result.len(), 5);
/// // result[2] = 120.0 / 100.0 = 1.2
/// ```
pub fn rocr(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate ROCR starting from period index
    for i in period..len {
        if prices[i - period] == 0.0 {
            result[i] = f64::NAN;
        } else {
            result[i] = prices[i] / prices[i - period];
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocr_basic() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let result = rocr(&prices, 2).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Check calculations
        assert!((result[2] - 1.2).abs() < 1e-8); // 120/100 = 1.2
        assert!((result[3] - 1.0454545454545454).abs() < 1e-8); // 115/110 ≈ 1.0455
        assert!((result[4] - 1.0833333333333333).abs() < 1e-8); // 130/120 ≈ 1.0833
    }
}