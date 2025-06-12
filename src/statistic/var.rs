//! Variance
//!
//! Variance measures the average squared deviation from the mean.
//! It's the square of the standard deviation.

use crate::common::{TAError, TAResult};

/// Variance
///
/// Calculates the variance over a rolling window.
///
/// # Formula
/// ```text
/// σ² = Σ(x[i] - μ)² / n
/// ```
///
/// # Arguments
/// * `data` - Input data series
/// * `period` - Period for variance calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of variance values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn var(data: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if data.is_empty() {
        return Err(TAError::invalid_input("Data cannot be empty"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if period > data.len() {
        return Err(TAError::insufficient_data(period, data.len()));
    }
    
    let len = data.len();
    let mut result = vec![f64::NAN; len];
    
    for i in (period - 1)..len {
        let start_idx = i + 1 - period;
        let window = &data[start_idx..=i];
        
        // Calculate mean
        let mean = window.iter().sum::<f64>() / period as f64;
        
        // Calculate variance
        let variance = window.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / period as f64;
        
        result[i] = variance;
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_var_basic() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = var(&data, 8).unwrap();
        
        // For this data set, the variance should be 4.0
        assert_float_eq!(result[7], 4.0, 1e-10);
    }
}