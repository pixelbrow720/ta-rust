// MINUS_DI - Minus Directional Indicator
use crate::common::{TAError, TAResult};
use crate::momentum::minus_dm;
use crate::volatility::atr;

pub fn minus_di(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let len = close.len();
    if high.len() != len || low.len() != len {
        return Err(TAError::MismatchedInputLength);
    }
    let minusdm = minus_dm(high, low)?;
    let atr = atr(high, low, close, period)?;
    let mut out = vec![f64::NAN; len];
    for i in 0..len {
        if atr[i].abs() < 1e-12 {
            out[i] = 0.0;
        } else {
            out[i] = 100.0 * minusdm[i] / atr[i];
        }
    }
    Ok(out)
} 