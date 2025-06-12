// STOCH - Stochastic Oscillator
use crate::common::{TAError, TAResult, MAType};
use crate::overlap::ma;

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
        return Err(TAError::MismatchedInputLength);
    }
    if len < fastk_period {
        return Err(TAError::InsufficientData);
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