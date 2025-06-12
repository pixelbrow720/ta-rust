// PLUS_DI - Plus Directional Indicator
use crate::common::{TAError, TAResult};
use crate::momentum::plus_dm;
use crate::volatility::atr;

/// Calculates the Plus Directional Indicator.
/// 
/// # Arguments
/// * `high` - High prices
/// * `low` - Low prices
/// * `close` - Close prices
/// * `period` - Period for calculation
/// 
/// # Returns
/// Vector of Plus DI values
pub fn plus_di(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let len = close.len();
    if high.len() != len || low.len() != len {
        return Err(TAError::mismatched_inputs(format!("high: {}, low: {}, close: {}", high.len(), low.len(), len)));
    }
    let plusdm = plus_dm(high, low)?;
    let atr = atr(high, low, close, period)?;
    let mut out = vec![f64::NAN; len];
    for i in 0..len {
        if atr[i].abs() < 1e-12 {
            out[i] = 0.0;
        } else {
            out[i] = 100.0 * plusdm[i] / atr[i];
        }
    }
    Ok(out)
} 