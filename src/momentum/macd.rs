// MACD - Moving Average Convergence/Divergence
// Input: price: &[f64], fast_period: usize, slow_period: usize, signal_period: usize
// Output: (Vec<f64>, Vec<f64>, Vec<f64>) => (macd, signal, hist)

use crate::common::{TAError, TAResult};
use crate::overlap::ema;

/// Calculates the MACD (Moving Average Convergence/Divergence) indicator.
/// 
/// # Arguments
/// * `price` - Input price series
/// * `fast_period` - Fast EMA period (default: 12)
/// * `slow_period` - Slow EMA period (default: 26)
/// * `signal_period` - Signal EMA period (default: 9)
/// 
/// # Returns
/// Tuple of (MACD line, Signal line, Histogram)
///
/// # Errors
/// Returns `TAError::InsufficientData` if input is too short.
pub fn macd(
    price: &[f64],
    fast_period: usize,
    slow_period: usize,
    signal_period: usize,
) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if price.is_empty() {
        return Err(TAError::invalid_input("Price data cannot be empty"));
    }
    
    if price.len() < slow_period {
        return Err(TAError::insufficient_data(slow_period, price.len()));
    }
    
    // Calculate EMAs
    let fast_ema = ema(price, fast_period)?;
    let slow_ema = ema(price, slow_period)?;
    
    // Calculate MACD line
    let mut macd_line = vec![f64::NAN; price.len()];
    for i in (slow_period - 1)..price.len() {
        if !fast_ema[i].is_nan() && !slow_ema[i].is_nan() {
            macd_line[i] = fast_ema[i] - slow_ema[i];
        }
    }
    
    // Calculate signal line using EMA of MACD line
    // Extract valid MACD values for signal calculation
    let valid_macd_start = slow_period - 1;
    let macd_for_signal: Vec<f64> = macd_line[valid_macd_start..].to_vec();
    
    if macd_for_signal.len() < signal_period {
        // Not enough data for signal line
        let signal_line = vec![f64::NAN; price.len()];
        let hist = vec![f64::NAN; price.len()];
        return Ok((macd_line, signal_line, hist));
    }
    
    let signal_ema = ema(&macd_for_signal, signal_period)?;
    
    // Map signal back to full length
    let mut signal_line = vec![f64::NAN; price.len()];
    for (i, &sig_val) in signal_ema.iter().enumerate() {
        let full_index = valid_macd_start + i;
        if full_index < price.len() {
            signal_line[full_index] = sig_val;
        }
    }
    
    // Calculate histogram
    let mut hist = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if !macd_line[i].is_nan() && !signal_line[i].is_nan() {
            hist[i] = macd_line[i] - signal_line[i];
        }
    }
    
    Ok((macd_line, signal_line, hist))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_macd_basic() {
        let price = vec![1.0; 50];
        let (macd, signal, hist) = macd(&price, 12, 26, 9).unwrap();
        // MACD line, signal, and hist should all be zero after enough periods
        for i in 35..50 {
            assert!((macd[i] - 0.0).abs() < 1e-8);
            assert!((signal[i] - 0.0).abs() < 1e-8);
            assert!((hist[i] - 0.0).abs() < 1e-8);
        }
    }
}
