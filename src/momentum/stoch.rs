// STOCH - Stochastic Oscillator
use crate::common::{TAError, TAResult, MAType};
use crate::overlap::ma;

/// Calculates the Stochastic Oscillator.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `fastk_period` - Fast %K period
/// * `slowk_period` - Slow %K period
/// * `slowk_ma` - Slow %K moving average type
/// * `slowd_period` - Slow %D period
/// * `slowd_ma` - Slow %D moving average type
/// 
/// # Returns
/// Tuple of (Slow %K, Slow %D)
pub fn stoch(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    fastk_period: usize,
    slowk_period: usize,
    slowk_ma: MAType,
    slowd_period: usize,
    slowd_ma: MAType,
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
    let slowk = ma(&fastk, slowk_period, slowk_ma)?;
    let slowd = ma(&slowk, slowd_period, slowd_ma)?;
    Ok((slowk, slowd))
} 