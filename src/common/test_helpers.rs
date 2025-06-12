//! Test helper functions for TA-Rust

use crate::common::Price;

/// Standard tolerance for floating point comparisons in tests
pub const DEFAULT_TOLERANCE: Price = 1e-8;

/// Relaxed tolerance for complex calculations
pub const RELAXED_TOLERANCE: Price = 1e-6;

/// Strict tolerance for simple calculations
pub const STRICT_TOLERANCE: Price = 1e-10;

/// Utility function to compare floating point arrays with tolerance
pub fn assert_arrays_approx_equal(actual: &[Price], expected: &[Price], tolerance: Price) {
    assert_eq!(actual.len(), expected.len(), "Array lengths don't match");
    
    for (i, (&a, &e)) in actual.iter().zip(expected.iter()).enumerate() {
        if e.is_nan() {
            assert!(a.is_nan(), "Expected NaN at index {}, got {}", i, a);
        } else {
            assert!(
                (a - e).abs() <= tolerance,
                "Values don't match at index {}: expected {}, got {}, diff = {}",
                i, e, a, (a - e).abs()
            );
        }
    }
}

/// Utility function to check if a result is within acceptable range
pub fn is_within_tolerance(actual: Price, expected: Price, tolerance: Price) -> bool {
    if expected.is_nan() {
        actual.is_nan()
    } else {
        (actual - expected).abs() <= tolerance
    }
}