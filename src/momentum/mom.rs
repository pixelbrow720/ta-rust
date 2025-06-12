//! Momentum (MOM)
//! 
//! Momentum measures the rate of change in price over a specified period.
//! It's one of the simplest momentum indicators, calculated as the difference
//! between the current price and the price n periods ago.

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Momentum indicator.
/// 
/// Momentum = Price[today] - Price[n periods ago]
/// 
/// Positive values indicate upward momentum, negative values indicate downward momentum.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back (typically 10 or 14)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing momentum values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as momentum needs historical data.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::mom;
/// 
/// let prices = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5];
/// let result = mom(&prices, 3).unwrap();
/// assert_eq!(result.len(), 8);
/// // result[3] = 11.5 - 10.0 = 1.5
/// ```
pub fn mom(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate momentum starting from period index
    for i in period..len {
        result[i] = prices[i] - prices[i - period];
    }
    
    Ok(result)
}

/// Calculates Momentum with percentage output.
/// 
/// Momentum % = ((Price[today] / Price[n periods ago]) - 1) * 100
/// 
/// This provides momentum as a percentage change rather than absolute difference.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing momentum percentage values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::mom_percent;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
/// let result = mom_percent(&prices, 2).unwrap();
/// assert_eq!(result.len(), 5);
/// // result[2] = ((120.0 / 100.0) - 1) * 100 = 20.0%
/// ```
pub fn mom_percent(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate momentum percentage starting from period index
    for i in period..len {
        if prices[i - period] == 0.0 {
            result[i] = f64::NAN;
        } else {
            result[i] = ((prices[i] / prices[i - period]) - 1.0) * 100.0;
        }
    }
    
    Ok(result)
}

/// Calculates Momentum oscillator (normalized between -100 and +100).
/// 
/// This normalizes momentum to make it easier to compare across different price levels.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back
/// * `smooth_period` - Period for smoothing the oscillator (typically 3-5)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing normalized momentum values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::mom_oscillator;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0, 125.0, 140.0];
/// let result = mom_oscillator(&prices, 3, 2).unwrap();
/// assert_eq!(result.len(), 7);
/// ```
pub fn mom_oscillator(prices: &[f64], period: usize, smooth_period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    validate_period(smooth_period, "smooth_period")?;
    
    let len = prices.len();
    if len <= period + smooth_period {
        return Err(TAError::insufficient_data(period + smooth_period + 1, len));
    }
    
    // Calculate basic momentum first
    let momentum = mom(prices, period)?;
    
    let mut result = vec![f64::NAN; len];
    
    // Smooth and normalize momentum
    for i in (period + smooth_period - 1)..len {
        let start_idx = i - smooth_period + 1;
        let end_idx = i + 1;
        
        // Calculate average momentum over smooth_period
        let sum: f64 = momentum[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .sum();
        let count = momentum[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .count();
        
        if count == smooth_period {
            let avg_momentum = sum / count as f64;
            let base_price = prices[i - period];
            
            if base_price != 0.0 {
                // Normalize to percentage
                result[i] = (avg_momentum / base_price) * 100.0;
            }
        }
    }
    
    Ok(result)
}

/// Calculates Momentum with signal line (smoothed momentum).
/// 
/// This provides both the momentum and a signal line for crossover analysis.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Number of periods to look back for momentum
/// * `signal_period` - Period for signal line smoothing
/// 
/// # Returns
/// 
/// Returns `Ok((Vec<f64>, Vec<f64>))` containing (momentum, signal_line), or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::mom_with_signal;
/// 
/// let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0, 125.0, 140.0, 135.0];
/// let (momentum, signal) = mom_with_signal(&prices, 3, 2).unwrap();
/// assert_eq!(momentum.len(), 8);
/// assert_eq!(signal.len(), 8);
/// ```
pub fn mom_with_signal(prices: &[f64], period: usize, signal_period: usize) -> Result<(Vec<f64>, Vec<f64>), TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    validate_period(signal_period, "signal_period")?;
    
    let len = prices.len();
    if len <= period + signal_period {
        return Err(TAError::insufficient_data(period + signal_period + 1, len));
    }
    
    // Calculate momentum
    let momentum = mom(prices, period)?;
    
    // Calculate signal line using SMA of momentum
    let mut signal = vec![f64::NAN; len];
    
    for i in (period + signal_period - 1)..len {
        let start_idx = i - signal_period + 1;
        let end_idx = i + 1;
        
        let sum: f64 = momentum[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .sum();
        let count = momentum[start_idx..end_idx].iter()
            .filter(|&&x| !x.is_nan())
            .count();
        
        if count == signal_period {
            signal[i] = sum / signal_period as f64;
        }
    }
    
    Ok((momentum, signal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mom_basic() {
        let prices = vec![10.0, 11.0, 12.0, 11.5, 13.0, 12.5, 14.0, 13.5];
        let result = mom(&prices, 3).unwrap();
        
        assert_eq!(result.len(), 8);
        
        // First 3 values should be NaN
        for i in 0..3 {
            assert!(result[i].is_nan());
        }
        
        // Check specific calculations
        assert!((result[3] - (11.5 - 10.0)).abs() < 1e-8); // 1.5
        assert!((result[4] - (13.0 - 11.0)).abs() < 1e-8); // 2.0
        assert!((result[5] - (12.5 - 12.0)).abs() < 1e-8); // 0.5
        assert!((result[6] - (14.0 - 11.5)).abs() < 1e-8); // 2.5
        assert!((result[7] - (13.5 - 13.0)).abs() < 1e-8); // 0.5
    }

    #[test]
    fn test_mom_period_1() {
        let prices = vec![10.0, 11.0, 12.0, 11.5, 13.0];
        let result = mom(&prices, 1).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First value should be NaN
        assert!(result[0].is_nan());
        
        // Check calculations (should be price differences)
        assert!((result[1] - (11.0 - 10.0)).abs() < 1e-8); // 1.0
        assert!((result[2] - (12.0 - 11.0)).abs() < 1e-8); // 1.0
        assert!((result[3] - (11.5 - 12.0)).abs() < 1e-8); // -0.5
        assert!((result[4] - (13.0 - 11.5)).abs() < 1e-8); // 1.5
    }

    #[test]
    fn test_mom_negative_momentum() {
        let prices = vec![15.0, 14.0, 13.0, 12.0, 11.0];
        let result = mom(&prices, 2).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // Check negative momentum
        assert!((result[2] - (13.0 - 15.0)).abs() < 1e-8); // -2.0
        assert!((result[3] - (12.0 - 14.0)).abs() < 1e-8); // -2.0
        assert!((result[4] - (11.0 - 13.0)).abs() < 1e-8); // -2.0
    }

    #[test]
    fn test_mom_insufficient_data() {
        let prices = vec![10.0, 11.0];
        let result = mom(&prices, 3);
        assert!(result.is_err());
        
        if let Err(TAError::InsufficientData { required, provided }) = result {
            assert_eq!(required, 4);
            assert_eq!(provided, 2);
        } else {
            panic!("Expected InsufficientData error");
        }
    }

    #[test]
    fn test_mom_empty_data() {
        let prices = vec![];
        let result = mom(&prices, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_mom_zero_period() {
        let prices = vec![10.0, 11.0, 12.0];
        let result = mom(&prices, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_mom_percent() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0];
        let result = mom_percent(&prices, 2).unwrap();
        
        assert_eq!(result.len(), 5);
        
        // First 2 values should be NaN
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        
        // Check percentage calculations
        assert!((result[2] - 20.0).abs() < 1e-8); // (120/100 - 1) * 100 = 20%
        assert!((result[3] - 4.545454545454546).abs() < 1e-8); // (115/110 - 1) * 100 ≈ 4.55%
        assert!((result[4] - 8.333333333333334).abs() < 1e-8); // (130/120 - 1) * 100 ≈ 8.33%
    }

    #[test]
    fn test_mom_percent_zero_price() {
        let prices = vec![0.0, 110.0, 120.0];
        let result = mom_percent(&prices, 1).unwrap();
        
        // Should handle zero price gracefully
        assert!(result[1].is_nan()); // Division by zero
    }

    #[test]
    fn test_mom_oscillator() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0, 125.0, 140.0];
        let result = mom_oscillator(&prices, 3, 2).unwrap();
        
        assert_eq!(result.len(), 7);
        
        // First several values should be NaN (period + smooth_period - 1 = 3 + 2 - 1 = 4)
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        
        // Later values should be valid
        assert!(!result[4].is_nan());
        assert!(!result[5].is_nan());
        assert!(!result[6].is_nan());
    }

    #[test]
    fn test_mom_with_signal() {
        let prices = vec![100.0, 110.0, 120.0, 115.0, 130.0, 125.0, 140.0, 135.0];
        let (momentum, signal) = mom_with_signal(&prices, 3, 2).unwrap();
        
        assert_eq!(momentum.len(), 8);
        assert_eq!(signal.len(), 8);
        
        // Check that momentum is calculated correctly
        for i in 0..3 {
            assert!(momentum[i].is_nan());
        }
        
        // Check that signal is calculated correctly (period + signal_period - 1 = 3 + 2 - 1 = 4)
        for i in 0..4 {
            assert!(signal[i].is_nan());
        }
        
        // Signal should be smoothed version of momentum
        assert!(!signal[4].is_nan());
        assert!(!signal[5].is_nan());
        assert!(!signal[6].is_nan());
        assert!(!signal[7].is_nan());
    }

    #[test]
    fn test_mom_real_market_data() {
        // Simulate real market price movements
        let prices = vec![
            100.0, 102.0, 101.5, 103.0, 99.0, 101.0, 104.0, 102.5,
            105.0, 103.0, 106.0, 104.5, 107.0, 105.0, 108.0
        ];
        
        let result = mom(&prices, 10).unwrap();
        
        assert_eq!(result.len(), 15);
        
        // First 10 values should be NaN
        for i in 0..10 {
            assert!(result[i].is_nan());
        }
        
        // Check some calculations
        assert!((result[10] - (106.0 - 100.0)).abs() < 1e-8); // 6.0
        assert!((result[11] - (104.5 - 102.0)).abs() < 1e-8); // 2.5
        assert!((result[12] - (107.0 - 101.5)).abs() < 1e-8); // 5.5
    }

    #[test]
    fn test_mom_constant_prices() {
        let prices = vec![100.0; 10];
        let result = mom(&prices, 5).unwrap();
        
        // Momentum should be 0 for constant prices
        for i in 5..10 {
            assert!((result[i] - 0.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_mom_trending_up() {
        let prices = vec![100.0, 101.0, 102.0, 103.0, 104.0, 105.0];
        let result = mom(&prices, 3).unwrap();
        
        // All momentum values should be positive for uptrend
        for i in 3..6 {
            assert!(result[i] > 0.0);
        }
    }

    #[test]
    fn test_mom_trending_down() {
        let prices = vec![105.0, 104.0, 103.0, 102.0, 101.0, 100.0];
        let result = mom(&prices, 3).unwrap();
        
        // All momentum values should be negative for downtrend
        for i in 3..6 {
            assert!(result[i] < 0.0);
        }
    }

    #[test]
    fn test_mom_oscillating() {
        let prices = vec![100.0, 105.0, 100.0, 105.0, 100.0, 105.0];
        let result = mom(&prices, 2).unwrap();
        
        // Momentum should oscillate between positive and negative
        assert!((result[2] - 0.0).abs() < 1e-8); // 100 - 100 = 0
        assert!((result[3] - 0.0).abs() < 1e-8); // 105 - 105 = 0
        assert!((result[4] - 0.0).abs() < 1e-8); // 100 - 100 = 0
        assert!((result[5] - 0.0).abs() < 1e-8); // 105 - 105 = 0
    }

    #[test]
    fn test_mom_large_period() {
        let prices: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = mom(&prices, 15).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // First 15 values should be NaN
        for i in 0..15 {
            assert!(result[i].is_nan());
        }
        
        // Check calculations
        assert!((result[15] - (16.0 - 1.0)).abs() < 1e-8); // 15.0
        assert!((result[16] - (17.0 - 2.0)).abs() < 1e-8); // 15.0
        assert!((result[17] - (18.0 - 3.0)).abs() < 1e-8); // 15.0
    }
}