//! Linear Regression Slope

use crate::common::{TAError, TAResult};

/// Linear Regression Slope
pub fn linearreg_slope(data: &[f64], period: usize) -> TAResult<Vec<f64>> {
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
        
        let n = period as f64;
        let sum_x = (0..period).sum::<usize>() as f64;
        let sum_y = window.iter().sum::<f64>();
        let sum_xy = window.iter().enumerate().map(|(j, &y)| j as f64 * y).sum::<f64>();
        let sum_x2 = (0..period).map(|j| (j * j) as f64).sum::<f64>();
        
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        result[i] = slope;
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linearreg_slope_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = linearreg_slope(&data, 5).unwrap();
        
        // For linear data with slope 1, should return 1
        assert!((result[4] - 1.0).abs() < 1e-10);
    }
}