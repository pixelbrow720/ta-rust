//! Standard Deviation
//!
//! Standard deviation measures the amount of variation or dispersion in a dataset.
//! It's commonly used in finance to measure volatility and risk.

use crate::common::{TAError, TAResult};

/// Standard Deviation
///
/// Calculates the standard deviation over a rolling window. The result can be
/// scaled by a multiplier (deviations parameter) for use in indicators like
/// Bollinger Bands.
///
/// # Formula
/// ```text
/// σ = √(Σ(x[i] - μ)² / n)
/// Result = σ × deviations
/// 
/// Where μ is the mean of the window
/// ```
///
/// # Arguments
/// * `data` - Input data series
/// * `period` - Period for standard deviation calculation
/// * `deviations` - Multiplier for the standard deviation (typically 1.0)
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of standard deviation values
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::statistic::stddev;
///
/// let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
/// let result = stddev(&data, 5, 1.0).unwrap();
/// ```
pub fn stddev(data: &[f64], period: usize, deviations: f64) -> TAResult<Vec<f64>> {
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
        
        // Calculate standard deviation
        let std_dev = variance.sqrt();
        
        result[i] = std_dev * deviations;
    }
    
    Ok(result)
}

/// Standard Deviation with default parameters (period=5, deviations=1.0)
///
/// This is a convenience function using common default parameters.
///
/// # Arguments
/// * `data` - Input data series
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of standard deviation values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn stddev_default(data: &[f64]) -> TAResult<Vec<f64>> {
    stddev(data, 5, 1.0)
}

/// Sample Standard Deviation
///
/// Calculates the sample standard deviation (using n-1 in the denominator)
/// instead of population standard deviation (using n).
///
/// # Arguments
/// * `data` - Input data series
/// * `period` - Period for standard deviation calculation
/// * `deviations` - Multiplier for the standard deviation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of sample standard deviation values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn stddev_sample(data: &[f64], period: usize, deviations: f64) -> TAResult<Vec<f64>> {
    if data.is_empty() {
        return Err(TAError::invalid_input("Data cannot be empty"));
    }
    
    if period <= 1 {
        return Err(TAError::invalid_input("Period must be greater than 1 for sample standard deviation"));
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
        
        // Calculate sample variance (divide by n-1)
        let variance = window.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (period - 1) as f64;
        
        // Calculate standard deviation
        let std_dev = variance.sqrt();
        
        result[i] = std_dev * deviations;
    }
    
    Ok(result)
}

/// Rolling Coefficient of Variation
///
/// Calculates the coefficient of variation (standard deviation / mean) over a rolling window.
/// This provides a normalized measure of variability.
///
/// # Arguments
/// * `data` - Input data series
/// * `period` - Period for calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of coefficient of variation values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn coefficient_of_variation(data: &[f64], period: usize) -> TAResult<Vec<f64>> {
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
        
        if mean.abs() > f64::EPSILON {
            // Calculate variance
            let variance = window.iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>() / period as f64;
            
            // Calculate coefficient of variation
            let std_dev = variance.sqrt();
            result[i] = std_dev / mean.abs();
        } else {
            result[i] = f64::NAN; // Undefined when mean is zero
        }
    }
    
    Ok(result)
}

/// Z-Score calculation
///
/// Calculates the z-score (number of standard deviations from the mean) for each point
/// relative to a rolling window.
///
/// # Arguments
/// * `data` - Input data series
/// * `period` - Period for mean and standard deviation calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of z-score values
/// * `Err(TAError)` - Error if inputs are invalid
pub fn zscore(data: &[f64], period: usize) -> TAResult<Vec<f64>> {
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
        
        let std_dev = variance.sqrt();
        
        if std_dev > f64::EPSILON {
            // Calculate z-score for the current data point
            result[i] = (data[i] - mean) / std_dev;
        } else {
            result[i] = f64::NAN; // Undefined when standard deviation is zero
        }
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_stddev_basic() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let result = stddev(&data, 5, 1.0).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        
        // Values from index 4 onwards should be valid
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= 0.0); // Standard deviation is always non-negative
        }
    }

    #[test]
    fn test_stddev_constant_data() {
        let data = vec![20.0; 10];
        let result = stddev(&data, 5, 1.0).unwrap();
        
        // Standard deviation of constant data should be 0
        for i in 4..10 {
            assert_float_eq!(result[i], 0.0, 1e-10);
        }
    }

    #[test]
    fn test_stddev_with_multiplier() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let result1 = stddev(&data, 5, 1.0).unwrap();
        let result2 = stddev(&data, 5, 2.0).unwrap();
        
        // Result with 2.0 multiplier should be exactly double
        for i in 4..10 {
            assert_float_eq!(result2[i], result1[i] * 2.0, 1e-10);
        }
    }

    #[test]
    fn test_stddev_default() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        
        let result1 = stddev_default(&data).unwrap();
        let result2 = stddev(&data, 5, 1.0).unwrap();
        
        assert_eq!(result1.len(), result2.len());
        for i in 0..result1.len() {
            if result1[i].is_nan() && result2[i].is_nan() {
                continue;
            }
            assert_float_eq!(result1[i], result2[i], 1e-10);
        }
    }

    #[test]
    fn test_stddev_sample() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let result_pop = stddev(&data, 5, 1.0).unwrap();
        let result_sample = stddev_sample(&data, 5, 1.0).unwrap();
        
        // Sample standard deviation should be larger than population standard deviation
        for i in 4..10 {
            assert!(result_sample[i] > result_pop[i]);
            
            // Relationship: sample_std = pop_std * sqrt(n/(n-1))
            let expected = result_pop[i] * (5.0 / 4.0_f64).sqrt();
            assert_float_eq!(result_sample[i], expected, 1e-10);
        }
    }

    #[test]
    fn test_coefficient_of_variation() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let result = coefficient_of_variation(&data, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // CV should be positive for positive data
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert!(result[i] >= 0.0);
        }
    }

    #[test]
    fn test_coefficient_of_variation_zero_mean() {
        let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0]; // Mean is 0
        let result = coefficient_of_variation(&data, 5).unwrap();
        
        // CV should be NaN when mean is zero
        assert!(result[4].is_nan());
    }

    #[test]
    fn test_zscore() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0, 23.0, 22.0, 21.0, 20.0, 19.0];
        let result = zscore(&data, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // Z-scores should be reasonable values
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert!(result[i].abs() < 10.0); // Should be within reasonable range
        }
    }

    #[test]
    fn test_zscore_constant_data() {
        let data = vec![20.0; 10];
        let result = zscore(&data, 5).unwrap();
        
        // Z-score is undefined for constant data (zero standard deviation)
        for i in 4..10 {
            assert!(result[i].is_nan());
        }
    }

    #[test]
    fn test_zscore_properties() {
        // Create data where we know the z-score
        let data = vec![18.0, 19.0, 20.0, 21.0, 22.0]; // Mean = 20, current value = 22
        let result = zscore(&data, 5).unwrap();
        
        // For this specific case, we can calculate expected z-score
        // Mean = 20, std = sqrt(2), current = 22
        // z = (22 - 20) / sqrt(2) = 2 / sqrt(2) = sqrt(2)
        let expected = 2.0 / (2.0_f64).sqrt();
        assert_float_eq!(result[4], expected, 1e-10);
    }

    #[test]
    fn test_stddev_invalid_input() {
        let data: Vec<f64> = vec![];
        assert!(stddev(&data, 5, 1.0).is_err());
        
        let data = vec![20.0, 21.0];
        assert!(stddev(&data, 0, 1.0).is_err());  // Zero period
        assert!(stddev(&data, 5, 1.0).is_err());  // Period > data length
        
        // Sample standard deviation with period <= 1
        assert!(stddev_sample(&data, 1, 1.0).is_err());
    }

    #[test]
    fn test_stddev_single_period() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0];
        let result = stddev(&data, 1, 1.0).unwrap();
        
        // With period 1, standard deviation should be 0 (no variance)
        for &value in result.iter() {
            assert_float_eq!(value, 0.0, 1e-10);
        }
    }

    #[test]
    fn test_stddev_known_values() {
        // Test with known standard deviation
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let result = stddev(&data, 8, 1.0).unwrap();
        
        // For this data set, the population standard deviation should be 2.0
        assert_float_eq!(result[7], 2.0, 1e-10);
    }

    #[test]
    fn test_stddev_rolling_window() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = stddev(&data, 3, 1.0).unwrap();
        
        // Standard deviation should be consistent for this linear data
        // For any 3 consecutive integers, std dev = sqrt(2/3) ≈ 0.8165
        let expected = (2.0 / 3.0_f64).sqrt();
        
        for i in 2..10 {
            assert_float_eq!(result[i], expected, 1e-10);
        }
    }

    #[test]
    fn test_stddev_negative_multiplier() {
        let data = vec![20.0, 21.0, 22.0, 23.0, 24.0];
        let result = stddev(&data, 3, -1.0).unwrap();
        
        // Negative multiplier should give negative results
        for i in 2..5 {
            assert!(result[i] <= 0.0);
        }
    }
}