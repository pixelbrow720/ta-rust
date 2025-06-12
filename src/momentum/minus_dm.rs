// MINUS_DM - Minus Directional Movement
use crate::common::{TAError, TAResult};

pub fn minus_dm(high: &[f64], low: &[f64]) -> TAResult<Vec<f64>> {
    let len = high.len();
    if low.len() != len {
        return Err(TAError::MismatchedInputLength);
    }
    let mut out = vec![f64::NAN; len];
    for i in 1..len {
        let up = high[i] - high[i - 1];
        let down = low[i - 1] - low[i];
        if down > 0.0 && down > up {
            out[i] = down;
        } else {
            out[i] = 0.0;
        }
    }
    Ok(out)
} 