// STOCHF - Stochastic Fast
use crate::common::{TAError, TAResult, MAType};
use crate::overlap::ma;

/// Calculates the Fast Stochastic Oscillator.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `fastk_period` - Fast %K period
/// * `fastd_period` - Fast %D period
/// * `fastd_ma` - Fast %D moving average type
/// 
/// # Returns
/// Tuple of (Fast %K, Fast %D)
pub fn stochf(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    fastd_period: usize,
    fastd_ma: MAType,
) -> TAResult<(Vec<f64>, Vec<f64>)> {
    let len = close.len();
    if high.len() != len || low.len() != len {
        return Err(TAError::mismatched_inputs(format!("high: {}, low: {}, close: {}", high.len(), low.len(), len)));
    }
    if len < fastk_period {
        return Err(TAError::insufficient_data(fastk_period, len));
    }
    let mut fastk = vec![f64::NAN; len];
    for i in (fastk_period - 1)..len {
        let (hh, ll) = (
            high[i + 1 - fastk_period..=i].iter().cloned().fold(f64::MIN, f64::max),
            low[i + 1 - fastk_period..=i].iter().cloned().fold(f64::MAX, f64::min),
        );
        let denom = hh - ll;
        if denom.abs() < 1e-12 {
            fastk[i] = 0.0;
        } else {
            fastk[i] = 100.0 * (close[i] - ll) / denom;
        }
    }
    let fastd = ma(&fastk, fastd_period, fastd_ma)?;
    Ok((fastk, fastd))
} 