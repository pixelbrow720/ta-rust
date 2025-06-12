# Contributing to TA-Rust

Thank you for your interest in contributing to TA-Rust! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Git
- Basic understanding of technical analysis concepts

### Setting Up Development Environment

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/ta-rust.git
   cd ta-rust
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## 📋 Development Process

### Branch Naming

- `feature/indicator-name` - For new indicators
- `fix/issue-description` - For bug fixes
- `docs/section-name` - For documentation updates
- `perf/optimization-area` - For performance improvements

### Commit Messages

Follow conventional commits format:
- `feat: add RSI indicator implementation`
- `fix: correct EMA calculation for edge cases`
- `docs: update API documentation for SMA`
- `test: add comprehensive tests for MACD`
- `perf: optimize memory allocation in moving averages`

## 🔧 Implementation Guidelines

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Maintain consistent naming conventions
- Add comprehensive documentation

### Function Implementation Pattern

Each indicator function should follow this pattern:

```rust
/// Brief description of the indicator
/// 
/// # Parameters
/// - `data`: Input price data
/// - `period`: Calculation period
/// 
/// # Returns
/// Vector of calculated values with NaN for insufficient data periods
/// 
/// # Errors
/// - `InsufficientData` if data length < period
/// - `InvalidParameter` if period <= 0
/// 
/// # Example
/// ```rust
/// use ta_rust::overlap::sma;
/// 
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let result = sma(&prices, 3).unwrap();
/// assert_eq!(result[2], 2.0); // (1+2+3)/3
/// ```
pub fn indicator_name(data: &[Price], period: Period) -> TAResult<Vec<Price>> {
    // 1. Input validation
    validate_not_empty(data, "data")?;
    validate_period(period, "period")?;
    validate_sufficient_data(data, period, "data")?;
    
    // 2. Allocate output
    let mut output = allocate_output(data.len());
    
    // 3. Calculate indicator
    for i in (period - 1)..data.len() {
        // Calculation logic here
        output[i] = calculated_value;
    }
    
    Ok(output)
}
```

### Testing Requirements

Every function must include:

1. **Unit tests** with known expected values
2. **Edge case tests** (empty data, insufficient data, etc.)
3. **Accuracy tests** comparing against reference implementations
4. **Property-based tests** where applicable

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{assert_arrays_approx_equal, DEFAULT_TOLERANCE};

    #[test]
    fn test_indicator_basic() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = indicator_name(&data, 3).unwrap();
        let expected = vec![f64::NAN, f64::NAN, 2.0, 3.0, 4.0];
        assert_arrays_approx_equal(&result, &expected, DEFAULT_TOLERANCE);
    }

    #[test]
    fn test_indicator_insufficient_data() {
        let data = vec![1.0, 2.0];
        assert!(indicator_name(&data, 3).is_err());
    }

    #[test]
    fn test_indicator_empty_data() {
        let data = vec![];
        assert!(indicator_name(&data, 3).is_err());
    }

    #[test]
    fn test_indicator_invalid_period() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(indicator_name(&data, 0).is_err());
    }
}
```

### Documentation Requirements

- All public functions must have comprehensive rustdoc
- Include mathematical formulas where applicable
- Provide usage examples
- Document parameter constraints
- Explain return value format

### Performance Considerations

- Pre-allocate vectors with known capacity
- Use iterators instead of indexing where possible
- Avoid unnecessary allocations in loops
- Consider SIMD optimizations for hot paths
- Profile performance-critical functions

## 🧪 Testing

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test overlap

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Specific benchmark
cargo bench sma
```

### Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out html
```

## 📊 Accuracy Validation

### Reference Data

When implementing indicators, validate against:
1. TA-Lib C library results
2. Published academic papers
3. Financial data providers (Bloomberg, Reuters)
4. Other established libraries (pandas-ta, tulip, etc.)

### Tolerance Guidelines

- **Simple calculations**: 1e-10 tolerance
- **Complex calculations**: 1e-8 tolerance  
- **Iterative algorithms**: 1e-6 tolerance
- **Pattern recognition**: Exact match required

## 🚀 Performance Guidelines

### Optimization Priorities

1. **Correctness first** - Never sacrifice accuracy for speed
2. **Memory efficiency** - Minimize allocations
3. **CPU efficiency** - Optimize hot paths
4. **Maintainability** - Keep code readable

### Benchmarking Standards

New implementations should meet these performance targets:
- **Simple indicators** (SMA, EMA): < 1μs per 1000 data points
- **Complex indicators** (MACD, RSI): < 10μs per 1000 data points
- **Pattern recognition**: < 100μs per 1000 data points

## 📝 Documentation

### API Documentation

- Use rustdoc format
- Include mathematical formulas using LaTeX
- Provide complete examples
- Document all parameters and return values
- Explain error conditions

### Examples

Create examples in `examples/` directory:
- `basic_usage.rs` - Simple indicator usage
- `advanced_patterns.rs` - Complex pattern recognition
- `performance_comparison.rs` - Benchmarking examples

## 🔍 Code Review Process

### Before Submitting PR

1. Run `cargo fmt`
2. Run `cargo clippy`
3. Run `cargo test`
4. Run `cargo bench` (if performance-related)
5. Update documentation
6. Add changelog entry

### PR Requirements

- Clear description of changes
- Reference to related issues
- Test coverage for new code
- Performance impact assessment
- Breaking change documentation

### Review Criteria

- Code correctness and accuracy
- Test coverage and quality
- Documentation completeness
- Performance impact
- API design consistency
- Error handling robustness

## 🐛 Bug Reports

### Issue Template

```markdown
**Bug Description**
Clear description of the bug

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Reproduction Steps**
1. Step 1
2. Step 2
3. Step 3

**Environment**
- Rust version:
- TA-Rust version:
- OS:

**Test Data**
Minimal data set that reproduces the issue
```

## 💡 Feature Requests

### Proposal Template

```markdown
**Feature Description**
Clear description of the proposed feature

**Use Case**
Why is this feature needed?

**Implementation Ideas**
Suggestions for implementation

**Alternatives Considered**
Other approaches that were considered

**Additional Context**
Any other relevant information
```

## 📚 Resources

### Technical Analysis References

- [TA-Lib Documentation](https://ta-lib.org/doc/)
- [Technical Analysis Explained](https://www.amazon.com/Technical-Analysis-Explained-Fifth-Successful/dp/0071825177)
- [Encyclopedia of Technical Market Indicators](https://www.amazon.com/Encyclopedia-Technical-Market-Indicators-Second/dp/0070120579)

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

## 📞 Getting Help

- **GitHub Discussions** - For questions and general discussion
- **GitHub Issues** - For bug reports and feature requests
- **Discord** - Real-time chat (link in README)

## 🙏 Recognition

Contributors will be recognized in:
- CHANGELOG.md for each release
- README.md contributors section
- Cargo.toml authors field (for significant contributions)

Thank you for contributing to TA-Rust! 🚀