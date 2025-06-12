//! Error handling for TA-Rust

use thiserror::Error;

/// Result type used throughout the library
pub type TAResult<T> = Result<T, TAError>;

/// Errors that can occur during technical analysis calculations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum TAError {
    /// Insufficient data provided for calculation
    #[error("Insufficient data: need at least {required} data points, got {provided}")]
    InsufficientData {
        /// Required number of data points
        required: usize,
        /// Provided number of data points
        provided: usize,
    },

    /// Invalid parameter value
    #[error("Invalid parameter '{parameter}': {reason}")]
    InvalidParameter {
        /// Parameter name
        parameter: String,
        /// Reason why the parameter is invalid
        reason: String,
    },

    /// Empty input data
    #[error("Input data is empty")]
    EmptyInput,

    /// Mismatched input lengths
    #[error("Input arrays have different lengths: {details}")]
    MismatchedInputs {
        /// Details about the mismatch
        details: String,
    },

    /// Invalid input values (NaN, infinite, etc.)
    #[error("Invalid input values detected: {details}")]
    InvalidInput {
        /// Details about the invalid input
        details: String,
    },

    /// Calculation overflow or underflow
    #[error("Numerical error during calculation: {details}")]
    NumericalError {
        /// Details about the numerical error
        details: String,
    },

    /// Unsupported operation or feature
    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation {
        /// Description of the unsupported operation
        operation: String,
    },

    /// Internal library error
    #[error("Internal error: {details}")]
    InternalError {
        /// Details about the internal error
        details: String,
    },
}

impl TAError {
    /// Creates an insufficient data error
    pub fn insufficient_data(required: usize, provided: usize) -> Self {
        Self::InsufficientData { required, provided }
    }

    /// Creates an invalid parameter error
    pub fn invalid_parameter<S: Into<String>>(parameter: S, reason: S) -> Self {
        Self::InvalidParameter {
            parameter: parameter.into(),
            reason: reason.into(),
        }
    }

    /// Creates a mismatched inputs error
    pub fn mismatched_inputs<S: Into<String>>(details: S) -> Self {
        Self::MismatchedInputs {
            details: details.into(),
        }
    }

    /// Creates an invalid input error
    pub fn invalid_input<S: Into<String>>(details: S) -> Self {
        Self::InvalidInput {
            details: details.into(),
        }
    }

    /// Creates a numerical error
    pub fn numerical_error<S: Into<String>>(details: S) -> Self {
        Self::NumericalError {
            details: details.into(),
        }
    }

    /// Creates an unsupported operation error
    pub fn unsupported_operation<S: Into<String>>(operation: S) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
        }
    }

    /// Creates an internal error
    pub fn internal_error<S: Into<String>>(details: S) -> Self {
        Self::InternalError {
            details: details.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = TAError::insufficient_data(10, 5);
        assert!(matches!(err, TAError::InsufficientData { required: 10, provided: 5 }));

        let err = TAError::invalid_parameter("period", "must be positive");
        assert!(matches!(err, TAError::InvalidParameter { .. }));

        let err = TAError::mismatched_inputs("arrays have different lengths");
        assert!(matches!(err, TAError::MismatchedInputs { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = TAError::insufficient_data(10, 5);
        let msg = format!("{}", err);
        assert!(msg.contains("Insufficient data"));
        assert!(msg.contains("10"));
        assert!(msg.contains("5"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = TAError::insufficient_data(10, 5);
        let err2 = TAError::insufficient_data(10, 5);
        let err3 = TAError::insufficient_data(10, 6);

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }
}