//! Accuracy testing utilities for validating TA-Rust against reference implementations

use ta_rust::common::{Price, TAResult};

/// Standard tolerance for floating point comparisons in tests
pub const DEFAULT_TOLERANCE: Price = 1e-8;

/// Relaxed tolerance for complex calculations
pub const RELAXED_TOLERANCE: Price = 1e-6;

/// Strict tolerance for simple calculations
pub const STRICT_TOLERANCE: Price = 1e-10;

/// Test result structure for accuracy validation
#[derive(Debug, Clone)]
pub struct AccuracyTestResult {
    /// Test name
    pub name: String,
    /// Whether the test passed
    pub passed: bool,
    /// Maximum error found
    pub max_error: Price,
    /// Average error
    pub avg_error: Price,
    /// Number of values compared
    pub count: usize,
    /// Tolerance used
    pub tolerance: Price,
}

impl AccuracyTestResult {
    /// Creates a new test result
    pub fn new(name: String, tolerance: Price) -> Self {
        Self {
            name,
            passed: true,
            max_error: 0.0,
            avg_error: 0.0,
            count: 0,
            tolerance,
        }
    }

    /// Adds a comparison to the test result
    pub fn add_comparison(&mut self, actual: Price, expected: Price) {
        self.count += 1;
        
        let error = if expected.is_nan() && actual.is_nan() {
            0.0 // Both NaN is considered no error
        } else if expected.is_nan() || actual.is_nan() {
            Price::INFINITY // One NaN, one not is infinite error
        } else {
            (actual - expected).abs()
        };
        
        if error > self.tolerance {
            self.passed = false;
        }
        
        if error.is_finite() {
            self.max_error = self.max_error.max(error);
            self.avg_error = (self.avg_error * (self.count - 1) as Price + error) / self.count as Price;
        }
    }

    /// Finalizes the test result
    pub fn finalize(mut self) -> Self {
        if self.count == 0 {
            self.passed = false;
        }
        self
    }
}

impl std::fmt::Display for AccuracyTestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} (max_err: {:.2e}, avg_err: {:.2e}, count: {}, tol: {:.2e})",
            self.name,
            if self.passed { "PASS" } else { "FAIL" },
            self.max_error,
            self.avg_error,
            self.count,
            self.tolerance
        )
    }
}

/// Compares two arrays of prices with the given tolerance
pub fn compare_arrays(
    actual: &[Price],
    expected: &[Price],
    tolerance: Price,
    test_name: &str,
) -> AccuracyTestResult {
    let mut result = AccuracyTestResult::new(test_name.to_string(), tolerance);
    
    if actual.len() != expected.len() {
        result.passed = false;
        return result.finalize();
    }
    
    for (&a, &e) in actual.iter().zip(expected.iter()) {
        result.add_comparison(a, e);
    }
    
    result.finalize()
}

/// Compares a single calculation result with expected value
pub fn compare_single(
    actual: Price,
    expected: Price,
    tolerance: Price,
    test_name: &str,
) -> AccuracyTestResult {
    let mut result = AccuracyTestResult::new(test_name.to_string(), tolerance);
    result.add_comparison(actual, expected);
    result.finalize()
}

/// Validates that a function produces the expected output for given input
pub fn validate_function<F>(
    function: F,
    input: &[Price],
    expected: &[Price],
    tolerance: Price,
    test_name: &str,
) -> AccuracyTestResult
where
    F: Fn(&[Price]) -> TAResult<Vec<Price>>,
{
    match function(input) {
        Ok(actual) => compare_arrays(&actual, expected, tolerance, test_name),
        Err(_) => {
            let mut result = AccuracyTestResult::new(test_name.to_string(), tolerance);
            result.passed = false;
            result
        }
    }
}

/// Validates that a function with period parameter produces expected output
pub fn validate_function_with_period<F>(
    function: F,
    input: &[Price],
    period: usize,
    expected: &[Price],
    tolerance: Price,
    test_name: &str,
) -> AccuracyTestResult
where
    F: Fn(&[Price], usize) -> TAResult<Vec<Price>>,
{
    match function(input, period) {
        Ok(actual) => compare_arrays(&actual, expected, tolerance, test_name),
        Err(_) => {
            let mut result = AccuracyTestResult::new(test_name.to_string(), tolerance);
            result.passed = false;
            result
        }
    }
}

/// Validates that a function with OHLC input produces expected output
pub fn validate_ohlc_function<F>(
    function: F,
    open: &[Price],
    high: &[Price],
    low: &[Price],
    close: &[Price],
    expected: &[Price],
    tolerance: Price,
    test_name: &str,
) -> AccuracyTestResult
where
    F: Fn(&[Price], &[Price], &[Price], &[Price]) -> TAResult<Vec<Price>>,
{
    match function(open, high, low, close) {
        Ok(actual) => compare_arrays(&actual, expected, tolerance, test_name),
        Err(_) => {
            let mut result = AccuracyTestResult::new(test_name.to_string(), tolerance);
            result.passed = false;
            result
        }
    }
}

/// Validates that a function with OHLC input and period produces expected output
pub fn validate_ohlc_function_with_period<F>(
    function: F,
    open: &[Price],
    high: &[Price],
    low: &[Price],
    close: &[Price],
    period: usize,
    expected: &[Price],
    tolerance: Price,
    test_name: &str,
) -> AccuracyTestResult
where
    F: Fn(&[Price], &[Price], &[Price], &[Price], usize) -> TAResult<Vec<Price>>,
{
    match function(open, high, low, close, period) {
        Ok(actual) => compare_arrays(&actual, expected, tolerance, test_name),
        Err(_) => {
            let mut result = AccuracyTestResult::new(test_name.to_string(), tolerance);
            result.passed = false;
            result
        }
    }
}

/// Test suite runner for multiple accuracy tests
pub struct AccuracyTestSuite {
    results: Vec<AccuracyTestResult>,
}

impl AccuracyTestSuite {
    /// Creates a new test suite
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    /// Adds a test result to the suite
    pub fn add_result(&mut self, result: AccuracyTestResult) {
        self.results.push(result);
    }

    /// Returns the number of passed tests
    pub fn passed_count(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Returns the number of failed tests
    pub fn failed_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Returns the total number of tests
    pub fn total_count(&self) -> usize {
        self.results.len()
    }

    /// Returns true if all tests passed
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }

    /// Prints a summary of the test results
    pub fn print_summary(&self) {
        println!("\n=== Accuracy Test Suite Results ===");
        println!("Total tests: {}", self.total_count());
        println!("Passed: {}", self.passed_count());
        println!("Failed: {}", self.failed_count());
        println!("Success rate: {:.1}%", 
                 self.passed_count() as f64 / self.total_count() as f64 * 100.0);
        
        if !self.all_passed() {
            println!("\nFailed tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  {}", result);
                }
            }
        }
        
        println!("\nDetailed results:");
        for result in &self.results {
            println!("  {}", result);
        }
    }

    /// Returns the results
    pub fn results(&self) -> &[AccuracyTestResult] {
        &self.results
    }
}

impl Default for AccuracyTestSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Macro for creating accuracy tests
#[macro_export]
macro_rules! accuracy_test {
    ($suite:expr, $func:expr, $input:expr, $expected:expr, $tolerance:expr, $name:expr) => {
        let result = validate_function($func, $input, $expected, $tolerance, $name);
        $suite.add_result(result);
    };
    
    ($suite:expr, $func:expr, $input:expr, $period:expr, $expected:expr, $tolerance:expr, $name:expr) => {
        let result = validate_function_with_period($func, $input, $period, $expected, $tolerance, $name);
        $suite.add_result(result);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accuracy_test_result() {
        let mut result = AccuracyTestResult::new("test".to_string(), 0.01);
        
        // Add some comparisons
        result.add_comparison(1.0, 1.001); // Within tolerance
        result.add_comparison(2.0, 2.005); // Within tolerance
        result.add_comparison(3.0, 3.02);  // Outside tolerance
        
        let final_result = result.finalize();
        assert!(!final_result.passed); // Should fail due to last comparison
        assert_eq!(final_result.count, 3);
        assert!(final_result.max_error > 0.01);
    }

    #[test]
    fn test_compare_arrays() {
        let actual = vec![1.0, 2.0, 3.0];
        let expected = vec![1.001, 1.999, 3.001];
        
        let result = compare_arrays(&actual, &expected, 0.01, "test");
        assert!(result.passed);
        
        let result = compare_arrays(&actual, &expected, 0.0001, "test");
        assert!(!result.passed);
    }

    #[test]
    fn test_compare_arrays_with_nan() {
        let actual = vec![1.0, Price::NAN, 3.0];
        let expected = vec![1.001, Price::NAN, 3.001];
        
        let result = compare_arrays(&actual, &expected, 0.01, "test");
        assert!(result.passed);
    }

    #[test]
    fn test_accuracy_test_suite() {
        let mut suite = AccuracyTestSuite::new();
        
        let result1 = AccuracyTestResult {
            name: "test1".to_string(),
            passed: true,
            max_error: 0.001,
            avg_error: 0.0005,
            count: 10,
            tolerance: 0.01,
        };
        
        let result2 = AccuracyTestResult {
            name: "test2".to_string(),
            passed: false,
            max_error: 0.02,
            avg_error: 0.01,
            count: 5,
            tolerance: 0.01,
        };
        
        suite.add_result(result1);
        suite.add_result(result2);
        
        assert_eq!(suite.total_count(), 2);
        assert_eq!(suite.passed_count(), 1);
        assert_eq!(suite.failed_count(), 1);
        assert!(!suite.all_passed());
    }
}