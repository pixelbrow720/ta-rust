//! Rate of Change Percentage (ROCP)
//! 
//! ROCP measures the percentage change in price over a specified period.
//! It's calculated as (Price[today] - Price[n periods ago]) / Price[n periods ago].

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Rate of Change Percentage.
/// 
/// ROCP = (Price[today] - Price[n periods ago]) / Price[n periods ago]
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing ROCP values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as ROCP needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rocp;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let result = rocp(&prices, 2).unwrap();
/// assert_eq!(result.len(), 5);
/// // result[2] = (120.0 - 100.0) / 100.0 = 0.2 (20%)
/// ```
pub fn rocp(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate ROCP starting from period index
    for i in period..len {
        if prices[i - period] == 0.0 {
            result[i] = f64::NAN;
        } else {
            result[i] = (prices[i] - prices[i - period]) / prices[i - period];
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rocp_basic() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let result = rocp(&prices, 2).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Check calculations
        assert!((result[2] - 0.2).abs() < 1e-8); // (120-100)/100 = 0.2
        assert!((result[3] - 0.045454545454545456).abs() < 1e-8); // (115-110)/110 ≈ 0.0455
        assert!((result[4] - 0.08333333333333333).abs() < 1e-8); // (130-120)/120 ≈ 0.0833
    }
}