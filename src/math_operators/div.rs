//! Vector Arithmetic Division

use crate::common::{TAResult, Price};
use crate::common::utils::{validate_not_empty, validate_same_length, allocate_output};

/// Performs element-wise division of two price arrays
pub fn div(array1: &[Price], array2: &[Price]) -> TAResult<Vec<Price>> {
    validate_not_empty(array1, "array1")?;
    validate_not_empty(array2, "array2")?;
    validate_same_length(array1, array2, "array1", "array2")?;

    let mut output = allocate_output(array1.len());
    
    for i in 0..array1.len() {
        if array2[i] == 0.0 {
            output[i] = Price::NAN;
        } else {
            output[i] = array1[i] / array2[i];
        }
    }

    Ok(output)
}

/// Divides each element of an array by a scalar value
pub fn div_scalar(array: &[Price], scalar: Price) -> TAResult<Vec<Price>> {
    validate_not_empty(array, "array")?;

    if scalar == 0.0 {
        return Err(crate::common::TAError::invalid_parameter(
            "scalar",
            "division by zero"
        ));
    }

    let mut output = allocate_output(array.len());
    
    for i in 0..array.len() {
        output[i] = array[i] / scalar;
    }

    Ok(output)
}