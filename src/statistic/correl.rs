//! Pearson's Correlation Coefficient
//!
//! Correlation measures the linear relationship between two variables.
//! Values range from -1 (perfect negative correlation) to +1 (perfect positive correlation),
//! with 0 indicating no linear relationship.

use crate::common::{TAError, TAResult};

/// Pearson's Correlation Coefficient
///
/// Calculates the Pearson correlation coefficient between two data series over a rolling window.
/// The correlation coefficient measures the strength and direction of the linear relationship.
///
/// # Formula
/// ```text
/// r = Σ((x[i] - x̄)(y[i] - ȳ)) / √(Σ(x[i] - x̄)² × Σ(y[i] - ȳ)²)
/// 
/// Where x̄ and ȳ are means of x and y
/// Range: -1 to +1
/// ```
///
/// # Arguments
/// * `series1` - First data series
/// * `series2` - Second data series
/// * `period` - Period for correlation calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of correlation coefficients
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::statistic::correl;
///
/// let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0];
/// let result = correl(&series1, &series2, 5).unwrap();
/// // Should show perfect positive correlation (1.0)
/// ```
pub fn correl(series1: &[f64], series2: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if series1.is_empty() || series2.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if series1.len() != series2.len() {
        return Err(TAError::mismatched_inputs("Input arrays must have the same length"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if period > series1.len() {
        return Err(TAError::insufficient_data(period, series1.len()));
    }
    
    let len = series1.len();
    let mut result = vec![f64::NAN; len];
    
    for i in (period - 1)..len {
        let start_idx = i + 1 - period;
        let window1 = &series1[start_idx..=i];
        let window2 = &series2[start_idx..=i];
        
        // Calculate means
        let mean1 = window1.iter().sum::<f64>() / period as f64;
        let mean2 = window2.iter().sum::<f64>() / period as f64;
        
        // Calculate correlation components
        let mut numerator = 0.0;
        let mut sum_sq1 = 0.0;
        let mut sum_sq2 = 0.0;
        
        for j in 0..period {
            let diff1 = window1[j] - mean1;
            let diff2 = window2[j] - mean2;
            
            numerator += diff1 * diff2;
            sum_sq1 += diff1 * diff1;
            sum_sq2 += diff2 * diff2;
        }
        
        // Calculate correlation coefficient
        let denominator = (sum_sq1 * sum_sq2).sqrt();
        
        if denominator.abs() > f64::EPSILON {
            result[i] = numerator / denominator;
            
            // Clamp to [-1, 1] to handle floating point precision issues
            result[i] = result[i].max(-1.0).min(1.0);
        } else {
            result[i] = f64::NAN; // Undefined when one or both series have no variance
        }
    }
    
    Ok(result)
}

/// Correlation with default period (20)
///
/// This is a convenience function using a common default period.
///
/// # Arguments
/// * `series1` - First data series
/// * `series2` - Second data series
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of correlation coefficients
/// * `Err(TAError)` - Error if inputs are invalid
pub fn correl_default(series1: &[f64], series2: &[f64]) -> TAResult<Vec<f64>> {
    correl(series1, series2, 20)
}

/// Calculate correlation matrix for multiple series
///
/// Returns the correlation coefficients between all pairs of input series.
///
/// # Arguments
/// * `series` - Vector of data series
/// * `period` - Period for correlation calculation
///
/// # Returns
/// * `Ok(Vec<Vec<Vec<f64>>>)` - 3D vector: [time][series1][series2] = correlation
/// * `Err(TAError)` - Error if inputs are invalid
pub fn correl_matrix(series: &[Vec<f64>], period: usize) -> TAResult<Vec<Vec<Vec<f64>>>> {
    if series.is_empty() {
        return Err(TAError::invalid_input("Series array cannot be empty"));
    }
    
    let num_series = series.len();
    let len = series[0].len();
    
    // Check all series have the same length
    for s in series.iter() {
        if s.len() != len {
            return Err(TAError::mismatched_inputs("All series must have the same length"));
        }
    }
    
    let mut result = vec![vec![vec![f64::NAN; num_series]; num_series]; len];
    
    // Calculate correlation for each pair
    for i in 0..num_series {
        for j in 0..num_series {
            if i == j {
                // Self-correlation is always 1.0 (where defined)
                let _self_correl = vec![1.0; len];
                for t in (period - 1)..len {
                    result[t][i][j] = 1.0;
                }
            } else {
                let correl_ij = correl(&series[i], &series[j], period)?;
                for t in 0..len {
                    result[t][i][j] = correl_ij[t];
                }
            }
        }
    }
    
    Ok(result)
}

/// Calculate correlation strength categories
///
/// Categorizes correlation values into strength levels for easier interpretation.
///
/// # Arguments
/// * `correlations` - Vector of correlation coefficients
///
/// # Returns
/// * `Vec<String>` - Vector of correlation strength descriptions
pub fn correl_strength(correlations: &[f64]) -> Vec<String> {
    correlations.iter().map(|&corr| {
        if corr.is_nan() {
            "Undefined".to_string()
        } else {
            let abs_corr = corr.abs();
            let strength = if abs_corr >= 0.9 {
                "Very Strong"
            } else if abs_corr >= 0.7 {
                "Strong"
            } else if abs_corr >= 0.5 {
                "Moderate"
            } else if abs_corr >= 0.3 {
                "Weak"
            } else {
                "Very Weak"
            };
            
            let direction = if corr > 0.0 {
                "Positive"
            } else {
                "Negative"
            };
            
            format!("{} {}", direction, strength)
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_correl_perfect_positive() {
        let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0];
        let result = correl(&series1, &series2, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        
        // Values from index 4 onwards should be 1.0 (perfect positive correlation)
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert_float_eq!(result[i], 1.0, 1e-10);
        }
    }

    #[test]
    fn test_correl_perfect_negative() {
        let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let series2 = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let result = correl(&series1, &series2, 5).unwrap();
        
        // Should show perfect negative correlation (-1.0)
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert_float_eq!(result[i], -1.0, 1e-10);
        }
    }

    #[test]
    fn test_correl_no_correlation() {
        let series1 = vec![1.0, 1.0, 1.0, 1.0, 1.0]; // Constant
        let series2 = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // Variable
        let result = correl(&series1, &series2, 5).unwrap();
        
        // Correlation with constant series is undefined
        assert!(result[4].is_nan());
    }

    #[test]
    fn test_correl_zero_correlation() {
        // Create orthogonal series (should have zero correlation)
        let series1 = vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0];
        let series2 = vec![1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 2.0];
        let result = correl(&series1, &series2, 4).unwrap();
        
        // Should have some values close to zero
        for i in 3..8 {
            assert!(!result[i].is_nan());
            assert!(result[i].abs() <= 1.0); // Valid correlation range
        }
    }

    #[test]
    fn test_correl_default() {
        let series1 = vec![1.0; 25]; // Need enough data for default period (20)
        let series2 = vec![2.0; 25];
        
        let result1 = correl_default(&series1, &series2).unwrap();
        let result2 = correl(&series1, &series2, 20).unwrap();
        
        assert_eq!(result1.len(), result2.len());
        for i in 0..result1.len() {
            if result1[i].is_nan() && result2[i].is_nan() {
                continue;
            }
            assert_float_eq!(result1[i], result2[i], 1e-10);
        }
    }

    #[test]
    fn test_correl_matrix() {
        let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0]; // Perfect positive correlation with series1
        let series3 = vec![5.0, 4.0, 3.0, 2.0, 1.0]; // Perfect negative correlation with series1
        
        let series = vec![series1, series2, series3];
        let result = correl_matrix(&series, 3).unwrap();
        
        assert_eq!(result.len(), 5); // Time dimension
        assert_eq!(result[0].len(), 3); // Number of series
        assert_eq!(result[0][0].len(), 3); // Number of series
        
        // Check self-correlations at a valid time point
        for t in 2..5 {
            for i in 0..3 {
                assert_float_eq!(result[t][i][i], 1.0, 1e-10);
            }
            
            // Check series1 vs series2 (should be 1.0)
            assert_float_eq!(result[t][0][1], 1.0, 1e-10);
            assert_float_eq!(result[t][1][0], 1.0, 1e-10);
            
            // Check series1 vs series3 (should be -1.0)
            assert_float_eq!(result[t][0][2], -1.0, 1e-10);
            assert_float_eq!(result[t][2][0], -1.0, 1e-10);
        }
    }

    #[test]
    fn test_correl_strength() {
        let correlations = vec![0.95, 0.75, 0.55, 0.35, 0.15, -0.85, -0.45, 0.0, f64::NAN];
        let strengths = correl_strength(&correlations);
        
        assert_eq!(strengths.len(), 9);
        assert!(strengths[0].contains("Very Strong"));
        assert!(strengths[1].contains("Strong"));
        assert!(strengths[2].contains("Moderate"));
        assert!(strengths[3].contains("Weak"));
        assert!(strengths[4].contains("Very Weak"));
        assert!(strengths[5].contains("Negative"));
        assert!(strengths[6].contains("Negative"));
        assert!(strengths[8].contains("Undefined"));
    }

    #[test]
    fn test_correl_invalid_input() {
        let series1: Vec<f64> = vec![];
        let series2: Vec<f64> = vec![];
        assert!(correl(&series1, &series2, 5).is_err());
        
        let series1 = vec![1.0, 2.0];
        let series2 = vec![1.0];
        assert!(correl(&series1, &series2, 5).is_err()); // Mismatched lengths
        
        let series1 = vec![1.0, 2.0];
        let series2 = vec![1.0, 2.0];
        assert!(correl(&series1, &series2, 0).is_err());  // Zero period
        assert!(correl(&series1, &series2, 5).is_err());  // Period > data length
    }

    #[test]
    fn test_correl_single_period() {
        let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = correl(&series1, &series2, 1).unwrap();
        
        // With period 1, correlation is undefined (need variance)
        for &value in result.iter() {
            assert!(value.is_nan());
        }
    }

    #[test]
    fn test_correl_two_period() {
        let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = correl(&series1, &series2, 2).unwrap();
        
        // With period 2, should have valid correlations
        for i in 1..5 {
            assert!(!result[i].is_nan());
            assert_float_eq!(result[i], 1.0, 1e-10); // Perfect correlation
        }
    }

    #[test]
    fn test_correl_rolling_window() {
        // Create data where correlation changes over time
        let mut series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // Increasing
        series1.extend(vec![5.0, 4.0, 3.0, 2.0, 1.0]); // Decreasing
        
        let mut series2 = vec![1.0, 2.0, 3.0, 4.0, 5.0]; // Increasing
        series2.extend(vec![6.0, 7.0, 8.0, 9.0, 10.0]); // Still increasing
        
        let result = correl(&series1, &series2, 5).unwrap();
        
        // Early correlation should be positive
        assert!(result[4] > 0.0);
        
        // Later correlation should be different (likely negative)
        assert!(result[9] != result[4]);
    }

    #[test]
    fn test_correl_boundary_values() {
        let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = correl(&series1, &series2, 5).unwrap();
        
        // All valid correlations should be in [-1, 1] range
        for &value in result.iter() {
            if !value.is_nan() {
                assert!(value >= -1.0 && value <= 1.0);
            }
        }
    }
}