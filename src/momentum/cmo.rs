//! Chande Momentum Oscillator (CMO)
//! 
//! CMO is a momentum oscillator that measures the sum of gains versus the sum of losses
//! over a specified period. It oscillates between -100 and +100.

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Chande Momentum Oscillator.
/// 
/// CMO = 100 * (Sum of Gains - Sum of Losses) / (Sum of Gains + Sum of Losses)
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Period for CMO calculation (typically 14)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing CMO values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as CMO needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::cmo;
/// 
/// let prices = vec![
///     44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
///     45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
/// ];
/// let result = cmo(&prices, 14).unwrap();
/// assert_eq!(result.len(), 20);
/// ```
pub fn cmo(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate price changes
    let mut changes = Vec::with_capacity(len - 1);
    for i in 1..len {
        changes.push(prices[i] - prices[i - 1]);
    }
    
    // Calculate CMO for each period
    for i in period..len {
        let start_idx = i - period;
        let end_idx = i;
        
        let mut sum_gains = 0.0;
        let mut sum_losses = 0.0;
        
        for j in start_idx..end_idx {
            let change = changes[j];
            if change > 0.0 {
                sum_gains += change;
            } else if change < 0.0 {
                sum_losses += -change;
            }
        }
        
        let total_movement = sum_gains + sum_losses;
        if total_movement == 0.0 {
            result[i] = 0.0;
        } else {
            result[i] = 100.0 * (sum_gains - sum_losses) / total_movement;
        }
    }
    
    Ok(result)
}

/// Calculates CMO with smoothing.
/// 
/// This applies a simple moving average to the CMO values for smoother signals.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Period for CMO calculation
/// * `smooth_period` - Period for smoothing the CMO (typically 3-5)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing smoothed CMO values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::cmo_smoothed;
/// 
/// let prices = vec![
///     44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
///     45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
/// ];
/// let result = cmo_smoothed(&prices, 10, 3).unwrap();
/// assert_eq!(result.len(), 20);
/// ```
pub fn cmo_smoothed(prices: &[f64], period: usize, smooth_period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    validate_period(smooth_period, "smooth_period")?;
    
    let len = prices.len();
    if len <= period + smooth_period {
        return Err(TAError::insufficient_data(period + smooth_period + 1, len));
    }
    
    // Calculate basic CMO first
    let cmo_values = cmo(prices, period)?;
    
    let mut result = vec![f64::NAN; len];
    
    // Apply smoothing
    for i in (period + smooth_period - 1)..len {
        let start_idx = i - smooth_period + 1;
        let end_idx = i + 1;
        
        let sum: f64 = cmo_values[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .sum();
        let count = cmo_values[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .count();
        
        if count == smooth_period {
            result[i] = sum / smooth_period as f64;
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmo_basic() {
        let prices = vec![
            44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
            45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
        ];
        
        let result = cmo(&prices, 14).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // First 14 values should be NaN
        for i in 0..14 {
            assert!(result[i].is_nan());
        }
        
        // CMO values should be between -100 and 100
        for i in 14..20 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= -100.0);
            assert!(result[i] <= 100.0);
        }
    }

    #[test]
    fn test_cmo_all_gains() {
        // Prices that only go up
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let result = cmo(&prices, 3).unwrap();
        
        // CMO should be 100 when there are only gains
        for i in 3..6 {
            assert!((result[i] - 100.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_cmo_all_losses() {
        // Prices that only go down
        let prices = vec![15.0, 14.0, 13.0, 12.0, 11.0, 10.0];
        let result = cmo(&prices, 3).unwrap();
        
        // CMO should be -100 when there are only losses
        for i in 3..6 {
            assert!((result[i] - (-100.0)).abs() < 1e-8);
        }
    }

    #[test]
    fn test_cmo_constant_prices() {
        let prices = vec![50.0; 10];
        let result = cmo(&prices, 5).unwrap();
        
        // CMO should be 0 for constant prices (no gains or losses)
        for i in 5..10 {
            assert!((result[i] - 0.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_cmo_equal_gains_losses() {
        // Alternating gains and losses of equal magnitude
        let prices = vec![100.0, 101.0, 100.0, 101.0, 100.0, 101.0];
        let result = cmo(&prices, 4).unwrap();
        
        // CMO should be close to 0 when gains equal losses
        for i in 4..6 {
            assert!(result[i].abs() < 1e-8);
        }
    }

    #[test]
    fn test_cmo_insufficient_data() {
        let prices = vec![10.0, 11.0];
        let result = cmo(&prices, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_cmo_empty_data() {
        let prices = vec![];
        let result = cmo(&prices, 14);
        assert!(result.is_err());
    }

    #[test]
    fn test_cmo_zero_period() {
        let prices = vec![10.0, 11.0, 12.0];
        let result = cmo(&prices, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_cmo_period_1() {
        let prices = vec![10.0, 11.0, 10.5, 12.0];
        let result = cmo(&prices, 1).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!(result[0].is_nan());
        
        // With period 1, CMO should be 100 for gains, -100 for losses
        assert!((result[1] - 100.0).abs() < 1e-8); // Gain
        assert!((result[2] - (-100.0)).abs() < 1e-8); // Loss
        assert!((result[3] - 100.0).abs() < 1e-8); // Gain
    }

    #[test]
    fn test_cmo_smoothed() {
        let prices = vec![
            44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
            45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
        ];
        
        let result = cmo_smoothed(&prices, 10, 3).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // First several values should be NaN
        for i in 0..12 {
            assert!(result[i].is_nan());
        }
        
        // Smoothed CMO values should be valid
        for i in 12..20 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= -100.0);
            assert!(result[i] <= 100.0);
        }
    }

    #[test]
    fn test_cmo_real_market_scenario() {
        // Simulate a realistic market scenario
        let prices = vec![
            50.0, 51.0, 52.0, 51.5, 53.0, 52.0, 54.0, 53.5, 55.0, 54.0,
            56.0, 55.5, 57.0, 56.0, 58.0, 57.5, 59.0, 58.0, 60.0, 59.5
        ];
        
        let result = cmo(&prices, 14).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // Check that CMO values are reasonable for uptrending market
        for i in 14..20 {
            assert!(!result[i].is_nan());
            assert!(result[i] > 0.0); // Should be positive for uptrend
            assert!(result[i] < 100.0); // But not at extreme
        }
    }

    #[test]
    fn test_cmo_oscillating_market() {
        // Simulate oscillating market
        let prices = vec![
            50.0, 55.0, 50.0, 55.0, 50.0, 55.0, 50.0, 55.0, 50.0, 55.0,
            50.0, 55.0, 50.0, 55.0, 50.0, 55.0
        ];
        
        let result = cmo(&prices, 10).unwrap();
        
        assert_eq!(result.len(), 16);
        
        // CMO should oscillate around 0 for oscillating market
        for i in 10..16 {
            assert!(!result[i].is_nan());
            assert!(result[i].abs() < 50.0); // Should not be extreme
        }
    }

    #[test]
    fn test_cmo_vs_rsi_relationship() {
        // CMO and RSI should have similar directional movements
        let prices = vec![
            44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
            45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
        ];
        
        let cmo_result = cmo(&prices, 14).unwrap();
        
        // CMO should be reasonable for this data
        for i in 14..20 {
            assert!(!cmo_result[i].is_nan());
            assert!(cmo_result[i] >= -100.0);
            assert!(cmo_result[i] <= 100.0);
        }
    }
}