// MACDFIX - MACD dengan fixed 12/26, signal period custom
use crate::common::{TAError, TAResult};
use crate::overlap::ema;

pub fn macdfix(price: &[f64], signal_period: usize) -> TAResult<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    if price.len() < 26 + signal_period - 1 {
        return Err(TAError::InsufficientData);
    }
    let fast_ema = ema(price, 12)?;
    let slow_ema = ema(price, 26)?;
    let mut macd = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < 25 {
            continue;
        }
        macd[i] = fast_ema[i] - slow_ema[i];
    }
    let signal = ema(&macd, signal_period)?;
    let mut hist = vec![f64::NAN; price.len()];
    for i in 0..price.len() {
        if i < 25 + signal_period - 1 {
            continue;
        }
        hist[i] = macd[i] - signal[i];
    }
    Ok((macd, signal, hist))
} 