// AROONOSC - Aroon Oscillator
use crate::common::{TAError, TAResult};
use crate::momentum::aroon;

pub fn aroonosc(
    high: &[f64],
    low: &[f64],
    period: usize,
) -> TAResult<Vec<f64>> {
    let (up, down) = aroon(high, low, period)?;
    let len = up.len();
    let mut out = vec![f64::NAN; len];
    for i in 0..len {
        out[i] = up[i] - down[i];
    }
    Ok(out)
} 