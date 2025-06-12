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
    if price.len() < slow_period + signal_period - 1 {
        return Err(TAError::InsufficientData);
    }
    let fast_ema = ema(price, fast_period)?;
    let slow_ema = ema(price, slow_period)?;
    let mut macd_line = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < slow_period - 1 {
            continue;
        }
        macd_line[i] = fast_ema[i] - slow_ema[i];
    }
    let signal_line = ema(&macd_line, signal_period)?;
    let mut hist = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < slow_period + signal_period - 2 {
            continue;
        }
        hist[i] = macd_line[i] - signal_line[i];
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