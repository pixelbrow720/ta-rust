//! Relative Strength Index (RSI)
//! 
//! RSI is a momentum oscillator that measures the speed and magnitude of price changes.
//! It oscillates between 0 and 100, with values above 70 typically considered overbought
//! and values below 30 considered oversold.

use crate::common::{TAError, validate_prices, validate_period};

/// Calculates Relative Strength Index using Wilder's smoothing method.
/// 
/// RSI = 100 - (100 / (1 + RS))
/// where RS = Average Gain / Average Loss
/// 
/// Uses Wilder's smoothing (alpha = 1/period) for calculating average gains and losses.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Period for RSI calculation (typically 14)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing RSI values, or `Err(TAError)` on invalid input.
/// The first `period` values will be NaN as RSI needs time to stabilize.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rsi;
/// 
/// let prices = vec![
///     44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
///     45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
/// ];
/// let result = rsi(&prices, 14).unwrap();
/// assert_eq!(result.len(), 20);
/// ```
pub fn rsi(prices: &[f64], period: usize) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    let alpha = 1.0 / period as f64;
    
    // Calculate price changes
    let mut gains = Vec::with_capacity(len - 1);
    let mut losses = Vec::with_capacity(len - 1);
    
    for i in 1..len {
        let change = prices[i] - prices[i - 1];
        gains.push(if change > 0.0 { change } else { 0.0 });
        losses.push(if change < 0.0 { -change } else { 0.0 });
    }
    
    // Initialize with SMA of first 'period' gains and losses
    let initial_avg_gain: f64 = gains[0..period].iter().sum::<f64>() / period as f64;
    let initial_avg_loss: f64 = losses[0..period].iter().sum::<f64>() / period as f64;
    
    let mut avg_gain = initial_avg_gain;
    let mut avg_loss = initial_avg_loss;
    
    // Calculate first RSI value
    if avg_loss == 0.0 {
        result[period] = 100.0;
    } else {
        let rs = avg_gain / avg_loss;
        result[period] = 100.0 - (100.0 / (1.0 + rs));
    }
    
    // Apply Wilder's smoothing for remaining values
    for i in (period + 1)..len {
        avg_gain = alpha * gains[i - 1] + (1.0 - alpha) * avg_gain;
        avg_loss = alpha * losses[i - 1] + (1.0 - alpha) * avg_loss;
        
        if avg_loss == 0.0 {
            result[i] = 100.0;
        } else {
            let rs = avg_gain / avg_loss;
            result[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }
    
    Ok(result)
}

/// Calculates RSI with custom smoothing factor.
/// 
/// This allows for different smoothing methods beyond Wilder's standard approach.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Period for initial SMA calculation
/// * `alpha` - Custom smoothing factor (0 < alpha <= 1)
/// 
/// # Returns
/// 
/// Returns `Ok(Vec<f64>)` containing RSI values, or `Err(TAError)` on invalid input.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rsi_custom;
/// 
/// let prices = vec![44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0];
/// // Use faster smoothing (alpha = 0.2 instead of 1/14 ≈ 0.071)
/// let result = rsi_custom(&prices, 5, 0.2).unwrap();
/// assert_eq!(result.len(), 9);
/// ```
pub fn rsi_custom(prices: &[f64], period: usize, alpha: f64) -> Result<Vec<f64>, TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    if alpha <= 0.0 || alpha > 1.0 {
        return Err(TAError::invalid_input("Alpha must be between 0 and 1"));
    }
    
    let len = prices.len();
    if len <= period {
        return Err(TAError::insufficient_data(period + 1, len));
    }
    
    let mut result = vec![f64::NAN; len];
    
    // Calculate price changes
    let mut gains = Vec::with_capacity(len - 1);
    let mut losses = Vec::with_capacity(len - 1);
    
    for i in 1..len {
        let change = prices[i] - prices[i - 1];
        gains.push(if change > 0.0 { change } else { 0.0 });
        losses.push(if change < 0.0 { -change } else { 0.0 });
    }
    
    // Initialize with SMA of first 'period' gains and losses
    let initial_avg_gain: f64 = gains[0..period].iter().sum::<f64>() / period as f64;
    let initial_avg_loss: f64 = losses[0..period].iter().sum::<f64>() / period as f64;
    
    let mut avg_gain = initial_avg_gain;
    let mut avg_loss = initial_avg_loss;
    
    // Calculate first RSI value
    if avg_loss == 0.0 {
        result[period] = 100.0;
    } else {
        let rs = avg_gain / avg_loss;
        result[period] = 100.0 - (100.0 / (1.0 + rs));
    }
    
    // Apply custom smoothing for remaining values
    for i in (period + 1)..len {
        avg_gain = alpha * gains[i - 1] + (1.0 - alpha) * avg_gain;
        avg_loss = alpha * losses[i - 1] + (1.0 - alpha) * avg_loss;
        
        if avg_loss == 0.0 {
            result[i] = 100.0;
        } else {
            let rs = avg_gain / avg_loss;
            result[i] = 100.0 - (100.0 / (1.0 + rs));
        }
    }
    
    Ok(result)
}

/// Calculates RSI divergence signals.
/// 
/// Identifies bullish and bearish divergences between price and RSI.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Period for RSI calculation
/// * `lookback` - Number of periods to look back for divergence detection
/// 
/// # Returns
/// 
/// Returns `Ok((Vec<f64>, Vec<i8>))` containing (RSI values, divergence signals), 
/// where signals are: 1 = bullish divergence, -1 = bearish divergence, 0 = no divergence.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rsi_divergence;
/// 
/// let prices = vec![
///     100.0, 102.0, 101.0, 103.0, 99.0, 101.0, 104.0, 102.0,
///     105.0, 103.0, 106.0, 104.0, 107.0, 105.0, 108.0, 106.0
/// ];
/// let (rsi_vals, signals) = rsi_divergence(&prices, 10, 5).unwrap();
/// assert_eq!(rsi_vals.len(), 16);
/// assert_eq!(signals.len(), 16);
/// ```
pub fn rsi_divergence(prices: &[f64], period: usize, lookback: usize) -> Result<(Vec<f64>, Vec<i8>), TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    validate_period(lookback, "lookback")?;
    
    let len = prices.len();
    if len <= period + lookback {
        return Err(TAError::insufficient_data(period + lookback + 1, len));
    }
    
    let rsi_values = rsi(prices, period)?;
    let mut signals = vec![0i8; len];
    
    // Look for divergences starting from period + lookback
    for i in (period + lookback)..len {
        let current_price = prices[i];
        let current_rsi = rsi_values[i];
        
        if current_rsi.is_nan() {
            continue;
        }
        
        // Find previous significant high/low within lookback period
        let start_idx = i.saturating_sub(lookback);
        
        // Check for bearish divergence (price higher high, RSI lower high)
        let mut price_higher_high = false;
        let mut rsi_lower_high = false;
        
        for j in start_idx..i {
            if !rsi_values[j].is_nan() {
                if prices[j] < current_price && rsi_values[j] > current_rsi {
                    price_higher_high = true;
                    rsi_lower_high = true;
                    break;
                }
            }
        }
        
        if price_higher_high && rsi_lower_high {
            signals[i] = -1; // Bearish divergence
            continue;
        }
        
        // Check for bullish divergence (price lower low, RSI higher low)
        let mut price_lower_low = false;
        let mut rsi_higher_low = false;
        
        for j in start_idx..i {
            if !rsi_values[j].is_nan() {
                if prices[j] > current_price && rsi_values[j] < current_rsi {
                    price_lower_low = true;
                    rsi_higher_low = true;
                    break;
                }
            }
        }
        
        if price_lower_low && rsi_higher_low {
            signals[i] = 1; // Bullish divergence
        }
    }
    
    Ok((rsi_values, signals))
}

/// Calculates RSI with overbought/oversold levels.
/// 
/// Returns RSI values along with overbought and oversold signals.
/// 
/// # Arguments
/// 
/// * `prices` - Price series (typically close prices)
/// * `period` - Period for RSI calculation
/// * `overbought` - Overbought level (typically 70)
/// * `oversold` - Oversold level (typically 30)
/// 
/// # Returns
/// 
/// Returns `Ok((Vec<f64>, Vec<i8>))` containing (RSI values, signals),
/// where signals are: 1 = oversold, -1 = overbought, 0 = neutral.
/// 
/// # Example
/// 
/// ```
/// use ta_rust::momentum::rsi_levels;
/// 
/// let prices = vec![
///     44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
///     45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
/// ];
/// let (rsi_vals, signals) = rsi_levels(&prices, 14, 70.0, 30.0).unwrap();
/// assert_eq!(rsi_vals.len(), 20);
/// assert_eq!(signals.len(), 20);
/// ```
pub fn rsi_levels(prices: &[f64], period: usize, overbought: f64, oversold: f64) -> Result<(Vec<f64>, Vec<i8>), TAError> {
    validate_prices(prices, "prices")?;
    validate_period(period, "period")?;
    
    if overbought <= oversold {
        return Err(TAError::invalid_input("Overbought level must be greater than oversold level"));
    }
    
    if overbought > 100.0 || oversold < 0.0 {
        return Err(TAError::invalid_input("RSI levels must be between 0 and 100"));
    }
    
    let rsi_values = rsi(prices, period)?;
    let mut signals = vec![0i8; rsi_values.len()];
    
    for (i, &rsi_val) in rsi_values.iter().enumerate() {
        if !rsi_val.is_nan() {
            if rsi_val >= overbought {
                signals[i] = -1; // Overbought
            } else if rsi_val <= oversold {
                signals[i] = 1; // Oversold
            }
        }
    }
    
    Ok((rsi_values, signals))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsi_basic() {
        let prices = vec![
            44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
            45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
        ];
        
        let result = rsi(&prices, 14).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // First 14 values should be NaN
        for i in 0..14 {
            assert!(result[i].is_nan());
        }
        
        // RSI values should be between 0 and 100
        for i in 14..20 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= 0.0);
            assert!(result[i] <= 100.0);
        }
    }

    #[test]
    fn test_rsi_wilder_smoothing() {
        let prices = vec![
            44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25
        ];
        
        let result = rsi(&prices, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // First 5 values should be NaN
        for i in 0..5 {
            assert!(result[i].is_nan());
        }
        
        // Check that RSI is calculated correctly
        assert!(!result[5].is_nan());
        assert!(result[5] >= 0.0 && result[5] <= 100.0);
    }

    #[test]
    fn test_rsi_all_gains() {
        // Prices that only go up
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        let result = rsi(&prices, 3).unwrap();
        
        // RSI should be 100 when there are only gains
        for i in 3..6 {
            assert!((result[i] - 100.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_rsi_all_losses() {
        // Prices that only go down
        let prices = vec![15.0, 14.0, 13.0, 12.0, 11.0, 10.0];
        let result = rsi(&prices, 3).unwrap();
        
        // RSI should be 0 when there are only losses
        for i in 3..6 {
            assert!((result[i] - 0.0).abs() < 1e-8);
        }
    }

    #[test]
    fn test_rsi_constant_prices() {
        let prices = vec![50.0; 10];
        let result = rsi(&prices, 5).unwrap();
        
        // RSI should be 50 for constant prices (no gains or losses)
        // Actually, with no changes, avg_gain = avg_loss = 0, so RSI = 50
        for i in 5..10 {
            // When both avg_gain and avg_loss are 0, RSI should be 50
            assert!(!result[i].is_nan());
        }
    }

    #[test]
    fn test_rsi_insufficient_data() {
        let prices = vec![10.0, 11.0];
        let result = rsi(&prices, 5);
        assert!(result.is_err());
        
        if let Err(TAError::InsufficientData { required, provided }) = result {
            assert_eq!(required, 6);
            assert_eq!(provided, 2);
        } else {
            panic!("Expected InsufficientData error");
        }
    }

    #[test]
    fn test_rsi_empty_data() {
        let prices = vec![];
        let result = rsi(&prices, 14);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsi_zero_period() {
        let prices = vec![10.0, 11.0, 12.0];
        let result = rsi(&prices, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsi_period_1() {
        let prices = vec![10.0, 11.0, 10.5, 12.0];
        let result = rsi(&prices, 1).unwrap();
        
        assert_eq!(result.len(), 4);
        assert!(result[0].is_nan());
        
        // With period 1, RSI should be 100 for gains, 0 for losses
        assert!((result[1] - 100.0).abs() < 1e-8); // Gain
        assert!((result[2] - 0.0).abs() < 1e-8);   // Loss
        assert!((result[3] - 100.0).abs() < 1e-8); // Gain
    }

    #[test]
    fn test_rsi_custom_alpha() {
        let prices = vec![44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0];
        let result = rsi_custom(&prices, 5, 0.5).unwrap();
        
        assert_eq!(result.len(), 9);
        
        // First 5 values should be NaN
        for i in 0..5 {
            assert!(result[i].is_nan());
        }
        
        // RSI values should be valid
        for i in 5..9 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= 0.0);
            assert!(result[i] <= 100.0);
        }
    }

    #[test]
    fn test_rsi_custom_invalid_alpha() {
        let prices = vec![44.0, 44.25, 44.5, 43.75, 44.5];
        
        let result = rsi_custom(&prices, 3, 0.0);
        assert!(result.is_err());
        
        let result = rsi_custom(&prices, 3, 1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsi_levels() {
        let prices = vec![
            44.0, 44.25, 44.5, 43.75, 44.5, 44.0, 44.25, 44.75, 45.0, 45.25,
            45.5, 45.25, 45.0, 44.5, 44.0, 44.25, 44.5, 44.75, 45.0, 45.25
        ];
        
        let (rsi_vals, signals) = rsi_levels(&prices, 14, 70.0, 30.0).unwrap();
        
        assert_eq!(rsi_vals.len(), 20);
        assert_eq!(signals.len(), 20);
        
        // Check signal logic
        for (_i, (&rsi_val, &signal)) in rsi_vals.iter().zip(signals.iter()).enumerate() {
            if !rsi_val.is_nan() {
                if rsi_val >= 70.0 {
                    assert_eq!(signal, -1); // Overbought
                } else if rsi_val <= 30.0 {
                    assert_eq!(signal, 1); // Oversold
                } else {
                    assert_eq!(signal, 0); // Neutral
                }
            } else {
                assert_eq!(signal, 0);
            }
        }
    }

    #[test]
    fn test_rsi_levels_invalid_params() {
        let prices = vec![44.0, 44.25, 44.5, 43.75, 44.5];
        
        // Overbought <= Oversold
        let result = rsi_levels(&prices, 3, 30.0, 70.0);
        assert!(result.is_err());
        
        // Invalid levels
        let result = rsi_levels(&prices, 3, 110.0, 30.0);
        assert!(result.is_err());
        
        let result = rsi_levels(&prices, 3, 70.0, -10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsi_divergence() {
        let prices = vec![
            100.0, 102.0, 101.0, 103.0, 99.0, 101.0, 104.0, 102.0,
            105.0, 103.0, 106.0, 104.0, 107.0, 105.0, 108.0, 106.0
        ];
        
        let (rsi_vals, signals) = rsi_divergence(&prices, 10, 5).unwrap();
        
        assert_eq!(rsi_vals.len(), 16);
        assert_eq!(signals.len(), 16);
        
        // Signals should be -1, 0, or 1
        for &signal in &signals {
            assert!(signal >= -1 && signal <= 1);
        }
    }

    #[test]
    fn test_rsi_real_market_scenario() {
        // Simulate a realistic market scenario
        let prices = vec![
            50.0, 51.0, 52.0, 51.5, 53.0, 52.0, 54.0, 53.5, 55.0, 54.0,
            56.0, 55.5, 57.0, 56.0, 58.0, 57.5, 59.0, 58.0, 60.0, 59.5
        ];
        
        let result = rsi(&prices, 14).unwrap();
        
        assert_eq!(result.len(), 20);
        
        // Check that RSI values are reasonable for uptrending market
        for i in 14..20 {
            assert!(!result[i].is_nan());
            assert!(result[i] > 50.0); // Should be above 50 for uptrend
            assert!(result[i] < 100.0); // But not at extreme
        }
    }

    #[test]
    fn test_rsi_oscillating_market() {
        // Simulate oscillating market
        let prices = vec![
            50.0, 55.0, 50.0, 55.0, 50.0, 55.0, 50.0, 55.0, 50.0, 55.0,
            50.0, 55.0, 50.0, 55.0, 50.0, 55.0
        ];
        
        let result = rsi(&prices, 10).unwrap();
        
        assert_eq!(result.len(), 16);
        
        // RSI should oscillate around 50 for oscillating market
        for i in 10..16 {
            assert!(!result[i].is_nan());
            assert!(result[i] > 30.0 && result[i] < 70.0);
        }
    }

    #[test]
    fn test_rsi_extreme_volatility() {
        // Test with extreme price movements
        let prices = vec![
            100.0, 200.0, 50.0, 150.0, 75.0, 125.0, 90.0, 110.0, 95.0, 105.0
        ];
        
        let result = rsi(&prices, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // RSI should handle extreme volatility gracefully
        for i in 5..10 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= 0.0);
            assert!(result[i] <= 100.0);
        }
    }
}