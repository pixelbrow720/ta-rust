// ULTOSC - Ultimate Oscillator
use crate::common::{TAError, TAResult};

pub fn ultosc(
    high: &[f64],
    low: &[f64],
    close: &[f64],
    period1: usize,
    period2: usize,
    period3: usize,
) -> TAResult<Vec<f64>> {
    let len = close.len();
    if high.len() != len || low.len() != len {
        return Err(TAError::MismatchedInputLength);
    }
    if len < period3 {
        return Err(TAError::InsufficientData);
    }
    let mut bp = vec![f64::NAN; len];
    let mut tr = vec![f64::NAN; len];
    for i in 0..len {
        let prev_close = if i == 0 { close[0] } else { close[i - 1] };
        bp[i] = close[i] - low[i].min(prev_close);
        tr[i] = high[i].max(prev_close) - low[i].min(prev_close);
    }
    let mut out = vec![f64::NAN; len];
    for i in (period3 - 1)..len {
        let sum1: f64 = bp[i + 1 - period1..=i].iter().sum();
        let sumtr1: f64 = tr[i + 1 - period1..=i].iter().sum();
        let sum2: f64 = bp[i + 1 - period2..=i].iter().sum();
        let sumtr2: f64 = tr[i + 1 - period2..=i].iter().sum();
        let sum3: f64 = bp[i + 1 - period3..=i].iter().sum();
        let sumtr3: f64 = tr[i + 1 - period3..=i].iter().sum();
        let avg1 = sum1 / sumtr1;
        let avg2 = sum2 / sumtr2;
        let avg3 = sum3 / sumtr3;
        out[i] = 100.0 * ((4.0 * avg1) + (2.0 * avg2) + avg3) / 7.0;
    }
    Ok(out)
} 