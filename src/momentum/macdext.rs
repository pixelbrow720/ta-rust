// MACDEXT - MACD dengan tipe MA yang bisa dipilih
use crate::common::{TAError, TAResult, MAType};
use crate::overlap::ma;

/// MACDEXT: MACD dengan tipe MA custom
pub fn macdext(
    price: &[f64],
    fast_period: usize,
    fast_ma: MAType,
    slow_period: usize,
    slow_ma: MAType,
    signal_period: usize,
    signal_ma: MAType,
) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if price.len() < slow_period + signal_period - 1 {
        return Err(TAError::insufficient_data(slow_period + signal_period - 1, price.len()));
    }
    let fast = ma(price, fast_period, fast_ma)?;
    let slow = ma(price, slow_period, slow_ma)?;
    let mut macd = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < slow_period - 1 {
            continue;
        }
        macd[i] = fast[i] - slow[i];
    }
    let signal = ma(&macd, signal_period, signal_ma)?;
    let mut hist = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < slow_period + signal_period - 2 {
            continue;
        }
        hist[i] = macd[i] - signal[i];
    }
    Ok((macd, signal, hist))
} 