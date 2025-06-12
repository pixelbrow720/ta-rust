// STOCHRSI - Stochastic RSI
use crate::common::{TAError, TAResult, MAType};
use crate::momentum::rsi;
use crate::overlap::ma;

pub fn stochrsi(
    price: &[f64],
    rsi_period: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_ma: MAType,
) -> TAResult<(Vec<f64>, Vec<f64>)> {
    let rsi_vec = rsi(price, rsi_period)?;
    let len = rsi_vec.len();
    if len < fastk_period {
        return Err(TAError::InsufficientData);
    }
    let mut stochrsi = vec![f64::NAN; len];
    for i in (fastk_period - 1)..len {
        let min_rsi = rsi_vec[i + 1 - fastk_period..=i].iter().cloned().fold(f64::MAX, f64::min);
        let max_rsi = rsi_vec[i + 1 - fastk_period..=i].iter().cloned().fold(f64::MIN, f64::max);
        let denom = max_rsi - min_rsi;
        if denom.abs() < 1e-12 {
            stochrsi[i] = 0.0;
        } else {
            stochrsi[i] = (rsi_vec[i] - min_rsi) / denom;
        }
    }
    let fastk: Vec<f64> = stochrsi.iter().map(|&v| v * 100.0).collect();
    let fastd = ma(&fastk, fastd_period, fastd_ma)?;
    Ok((fastk, fastd))
} 