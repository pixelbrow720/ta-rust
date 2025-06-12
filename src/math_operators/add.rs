//! Vector Arithmetic Add

use crate::common::{TAResult, Price};
use crate::common::utils::{validate_not_empty, validate_same_length, allocate_output};

/// Performs element-wise addition of two price arrays
///
/// # Formula
/// ```text
/// ADD[i] = array1[i] + array2[i]
/// ```
///
/// # Parameters
/// - `array1`: First input array
/// - `array2`: Second input array
///
/// # Returns
/// Vector containing the element-wise sum of the input arrays.
///
/// # Errors
/// - `EmptyInput` if either array is empty
/// - `MismatchedInputs` if arrays have different lengths
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::add;
///
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let result = add(&a, &b).unwrap();
/// // result = [5.0, 7.0, 9.0]
/// ```
pub fn add(array1: &[Price], array2: &[Price]) -> TAResult<Vec<Price>> {
    // Input validation
    validate_not_empty(array1, "array1")?;
    validate_not_empty(array2, "array2")?;
    validate_same_length(array1, array2, "array1", "array2")?;

    let mut output = allocate_output(array1.len());
    
    // Perform element-wise addition
    for i in 0..array1.len() {
        output[i] = array1[i] + array2[i];
    }

    Ok(output)
}

/// Adds a scalar value to each element of an array
///
/// # Parameters
/// - `array`: Input array
/// - `scalar`: Scalar value to add to each element
///
/// # Returns
/// Vector containing the array with scalar added to each element.
///
/// # Example
/// ```rust
/// use ta_rust::math_operators::add_scalar;
///
/// let a = vec![1.0, 2.0, 3.0];
/// let result = add_scalar(&a, 10.0).unwrap();
/// // result = [11.0, 12.0, 13.0]
/// ```
pub fn add_scalar(array: &[Price], scalar: Price) -> TAResult<Vec<Price>> {
    validate_not_empty(array, "array")?;

    let mut output = allocate_output(array.len());
    
    for i in 0..array.len() {
        output[i] = array[i] + scalar;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_add_basic() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = add(&a, &b).unwrap();
        let expected = vec![5.0, 7.0, 9.0];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_add_negative_numbers() {
        let a = vec![-1.0, 2.0, -3.0];
        let b = vec![4.0, -5.0, 6.0];
        let result = add(&a, &b).unwrap();
        let expected = vec![3.0, -3.0, 3.0];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_add_scalar() {
        let a = vec![1.0, 2.0, 3.0];
        let result = add_scalar(&a, 10.0).unwrap();
        let expected = vec![11.0, 12.0, 13.0];
        
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_add_mismatched_lengths() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0];
        
        assert!(add(&a, &b).is_err());
    }

    #[test]
    fn test_add_empty_arrays() {
        let a = vec![];
        let b = vec![];
        
        assert!(add(&a, &b).is_err());
    }
}