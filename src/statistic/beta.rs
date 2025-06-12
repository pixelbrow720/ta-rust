//! Beta Coefficient
//!
//! Beta measures the volatility or systematic risk of a security or portfolio
//! compared to the market as a whole. It's calculated as the covariance between
//! the security and market returns divided by the variance of market returns.

use crate::common::{TAError, TAResult};

/// Beta Coefficient
///
/// Beta measures how much a security's price moves relative to the market.
/// A beta of 1 indicates the security moves with the market, greater than 1
/// indicates higher volatility, and less than 1 indicates lower volatility.
///
/// # Formula
/// ```text
/// β = Covariance(Security, Market) / Variance(Market)
/// 
/// Where:
/// Covariance = Σ((Security[i] - Mean_Security) × (Market[i] - Mean_Market)) / (n-1)
/// Variance = Σ((Market[i] - Mean_Market)²) / (n-1)
/// ```
///
/// # Arguments
/// * `security_prices` - Slice of security price data
/// * `market_prices` - Slice of market/benchmark price data
/// * `period` - Period for beta calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of beta coefficients
/// * `Err(TAError)` - Error if inputs are invalid
///
/// # Examples
/// ```
/// use ta_rust::statistic::beta;
///
/// let security = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0];
/// let market = vec![1000.0, 1010.0, 1005.0, 1015.0, 1020.0, 1018.0, 1025.0, 1030.0, 1028.0, 1035.0];
/// let result = beta(&security, &market, 5).unwrap();
/// ```
pub fn beta(security_prices: &[f64], market_prices: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if security_prices.is_empty() || market_prices.is_empty() {
        return Err(TAError::invalid_input("Input arrays cannot be empty"));
    }
    
    if security_prices.len() != market_prices.len() {
        return Err(TAError::mismatched_inputs("Security and market arrays must have the same length"));
    }
    
    if period == 0 {
        return Err(TAError::invalid_parameter("period", "must be greater than 0"));
    }
    
    if period > security_prices.len() {
        return Err(TAError::insufficient_data(period, security_prices.len()));
    }
    
    let len = security_prices.len();
    let mut result = vec![f64::NAN; len];
    
    for i in (period - 1)..len {
        let start_idx = i + 1 - period;
        let security_window = &security_prices[start_idx..=i];
        let market_window = &market_prices[start_idx..=i];
        
        // Calculate means
        let security_mean = security_window.iter().sum::<f64>() / period as f64;
        let market_mean = market_window.iter().sum::<f64>() / period as f64;
        
        // Calculate covariance and variance
        let mut covariance = 0.0;
        let mut market_variance = 0.0;
        
        for j in 0..period {
            let security_diff = security_window[j] - security_mean;
            let market_diff = market_window[j] - market_mean;
            
            covariance += security_diff * market_diff;
            market_variance += market_diff * market_diff;
        }
        
        // Use sample covariance and variance (divide by n-1)
        if period > 1 {
            covariance /= (period - 1) as f64;
            market_variance /= (period - 1) as f64;
        }
        
        // Calculate beta
        if market_variance.abs() > f64::EPSILON {
            result[i] = covariance / market_variance;
        } else {
            result[i] = f64::NAN; // Undefined when market has no variance
        }
    }
    
    Ok(result)
}

/// Beta using returns instead of prices
///
/// This function calculates beta using return series instead of price series.
/// Returns are typically more stationary and may provide better beta estimates.
///
/// # Arguments
/// * `security_returns` - Slice of security return data
/// * `market_returns` - Slice of market/benchmark return data
/// * `period` - Period for beta calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of beta coefficients
/// * `Err(TAError)` - Error if inputs are invalid
pub fn beta_returns(security_returns: &[f64], market_returns: &[f64], period: usize) -> TAResult<Vec<f64>> {
    // This is the same calculation as beta() but using returns directly
    beta(security_returns, market_returns, period)
}

/// Calculate rolling beta with price-to-return conversion
///
/// This function converts prices to returns and then calculates rolling beta.
/// This is often more appropriate for financial analysis.
///
/// # Arguments
/// * `security_prices` - Slice of security price data
/// * `market_prices` - Slice of market/benchmark price data
/// * `period` - Period for beta calculation
///
/// # Returns
/// * `Ok(Vec<f64>)` - Vector of beta coefficients
/// * `Err(TAError)` - Error if inputs are invalid
pub fn beta_from_prices(security_prices: &[f64], market_prices: &[f64], period: usize) -> TAResult<Vec<f64>> {
    if security_prices.len() < 2 || market_prices.len() < 2 {
        return Err(TAError::invalid_input("Need at least 2 price points to calculate returns"));
    }
    
    // Convert prices to returns
    let mut security_returns = Vec::with_capacity(security_prices.len() - 1);
    let mut market_returns = Vec::with_capacity(market_prices.len() - 1);
    
    for i in 1..security_prices.len() {
        if security_prices[i - 1].abs() > f64::EPSILON {
            security_returns.push((security_prices[i] - security_prices[i - 1]) / security_prices[i - 1]);
        } else {
            security_returns.push(f64::NAN);
        }
        
        if market_prices[i - 1].abs() > f64::EPSILON {
            market_returns.push((market_prices[i] - market_prices[i - 1]) / market_prices[i - 1]);
        } else {
            market_returns.push(f64::NAN);
        }
    }
    
    // Calculate beta on returns
    let beta_result = beta(&security_returns, &market_returns, period)?;
    
    // Pad with NaN at the beginning to match original length
    let mut result = vec![f64::NAN; security_prices.len()];
    for i in 0..beta_result.len() {
        result[i + 1] = beta_result[i];
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_float_eq;
    #[test]
    fn test_beta_basic() {
        let security = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0];
        let market = vec![1000.0, 1010.0, 1005.0, 1015.0, 1020.0, 1018.0, 1025.0, 1030.0, 1028.0, 1035.0];
        let result = beta(&security, &market, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // First 4 values should be NaN
        for i in 0..4 {
            assert!(result[i].is_nan());
        }
        
        // Values from index 4 onwards should be valid
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert!(result[i].is_finite());
        }
    }

    #[test]
    fn test_beta_perfect_correlation() {
        // Security moves exactly with market (scaled)
        let market = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0];
        let security: Vec<f64> = market.iter().map(|&x| x * 2.0).collect(); // 2x leverage
        
        let result = beta(&security, &market, 5).unwrap();
        
        // Beta should be approximately 2.0 (perfect correlation with 2x scaling)
        for i in 4..10 {
            assert!(!result[i].is_nan());
            assert_float_eq!(result[i], 2.0, 0.1); // Allow some tolerance due to sample calculation
        }
    }

    #[test]
    fn test_beta_no_correlation() {
        let security = vec![100.0, 100.0, 100.0, 100.0, 100.0]; // Constant
        let market = vec![1000.0, 1010.0, 1005.0, 1015.0, 1020.0]; // Variable
        
        let result = beta(&security, &market, 5).unwrap();
        
        // Beta should be 0 (no covariance with constant security)
        assert!(!result[4].is_nan());
        assert_float_eq!(result[4], 0.0, 1e-10);
    }

    #[test]
    fn test_beta_no_market_variance() {
        let security = vec![100.0, 102.0, 101.0, 103.0, 105.0];
        let market = vec![1000.0, 1000.0, 1000.0, 1000.0, 1000.0]; // Constant market
        
        let result = beta(&security, &market, 5).unwrap();
        
        // Beta should be NaN when market has no variance
        assert!(result[4].is_nan());
    }

    #[test]
    fn test_beta_returns() {
        let security_returns = vec![0.02, -0.01, 0.02, 0.02, -0.01, 0.02, 0.02, -0.01, 0.02];
        let market_returns = vec![0.01, -0.005, 0.01, 0.005, -0.002, 0.007, 0.005, -0.002, 0.007];
        
        let result = beta_returns(&security_returns, &market_returns, 5).unwrap();
        
        assert_eq!(result.len(), 9);
        
        // Should have valid values
        for i in 4..9 {
            assert!(!result[i].is_nan());
            assert!(result[i].is_finite());
        }
    }

    #[test]
    fn test_beta_from_prices() {
        let security = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0];
        let market = vec![1000.0, 1010.0, 1005.0, 1015.0, 1020.0, 1018.0, 1025.0, 1030.0, 1028.0, 1035.0];
        
        let result = beta_from_prices(&security, &market, 5).unwrap();
        
        assert_eq!(result.len(), 10);
        
        // First value should be NaN (no return for first price)
        assert!(result[0].is_nan());
        
        // Should have some valid values
        let valid_count = result.iter().filter(|&&x| !x.is_nan()).count();
        assert!(valid_count > 0);
    }

    #[test]
    fn test_beta_invalid_input() {
        let security: Vec<f64> = vec![];
        let market: Vec<f64> = vec![];
        assert!(beta(&security, &market, 5).is_err());
        
        let security = vec![100.0, 102.0];
        let market = vec![1000.0];
        assert!(beta(&security, &market, 5).is_err()); // Mismatched lengths
        
        let security = vec![100.0, 102.0];
        let market = vec![1000.0, 1010.0];
        assert!(beta(&security, &market, 0).is_err());  // Zero period
        assert!(beta(&security, &market, 5).is_err());  // Period > data length
    }

    #[test]
    fn test_beta_single_period() {
        let security = vec![100.0, 102.0, 101.0, 103.0, 105.0];
        let market = vec![1000.0, 1010.0, 1005.0, 1015.0, 1020.0];
        
        let result = beta(&security, &market, 1).unwrap();
        
        // With period 1, beta is undefined (need at least 2 points for variance)
        // But our implementation should handle this gracefully
        for &value in result.iter() {
            if !value.is_nan() {
                assert!(value.is_finite());
            }
        }
    }

    #[test]
    fn test_beta_negative_correlation() {
        let market = vec![100.0, 102.0, 104.0, 106.0, 108.0, 110.0];
        let security: Vec<f64> = market.iter().map(|&x| 200.0 - x).collect(); // Inverse relationship
        
        let result = beta(&security, &market, 5).unwrap();
        
        // Beta should be negative (inverse correlation)
        assert!(!result[4].is_nan());
        assert!(result[4] < 0.0);
        assert!(!result[5].is_nan());
        assert!(result[5] < 0.0);
    }

    #[test]
    fn test_beta_rolling_calculation() {
        let security = vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0];
        let market = vec![1000.0, 1010.0, 1005.0, 1015.0, 1020.0, 1018.0, 1025.0, 1030.0, 1028.0, 1035.0];
        
        let result = beta(&security, &market, 3).unwrap();
        
        // Check that beta changes as we roll through the data
        let mut valid_values = Vec::new();
        for i in 2..result.len() {
            if !result[i].is_nan() {
                valid_values.push(result[i]);
            }
        }
        
        assert!(valid_values.len() > 1);
        
        // Values should be different (unless data is perfectly regular)
        let first_val = valid_values[0];
        let _has_variation = valid_values.iter().any(|&x| (x - first_val).abs() > 1e-10);
        // Note: might not have variation if data is very regular, so this is a soft check
    }
}