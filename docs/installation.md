# Installation Guide

This guide will help you install and set up TA-Rust in your Rust project.

## 📦 Prerequisites

- **Rust**: Version 1.70.0 or later
- **Cargo**: Comes with Rust installation

### Installing Rust

If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) and follow the installation instructions for your platform.

```bash
# Verify your Rust installation
rustc --version
cargo --version
```

## 🚀 Adding TA-Rust to Your Project

### Method 1: Using Cargo.toml

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
ta-rust = "0.1.0"
```

### Method 2: Using Cargo Command

```bash
cargo add ta-rust
```

### Method 3: From Git (Latest Development)

```toml
[dependencies]
ta-rust = { git = "https://github.com/pixelbrow720/ta-rust" }
```

## 🔧 Feature Flags

TA-Rust supports several feature flags for different use cases:

```toml
[dependencies]
ta-rust = { version = "0.1.0", features = ["std"] }
```

### Available Features

- **`std`** (default): Standard library support
- **`no_std`**: No standard library (for embedded systems)

### No Standard Library Support

For embedded or no-std environments:

```toml
[dependencies]
ta-rust = { version = "0.1.0", default-features = false, features = ["no_std"] }
```

## 📋 Basic Setup

### 1. Create a New Project

```bash
cargo new my-trading-app
cd my-trading-app
```

### 2. Add TA-Rust Dependency

```bash
cargo add ta-rust
```

### 3. Basic Usage Example

Create or modify `src/main.rs`:

```rust
use ta_rust::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample price data
    let prices = vec![
        44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.85, 45.92,
        45.73, 46.16, 47.04, 46.07, 46.03, 46.83, 47.69, 46.49,
        46.26, 47.09, 46.66, 46.80, 47.12, 45.81, 46.12, 45.55,
    ];

    // Calculate Simple Moving Average
    let sma_result = sma(&prices, 10)?;
    println!("SMA(10): {:?}", sma_result);

    // Calculate RSI
    let rsi_result = rsi(&prices, 14)?;
    println!("RSI(14): {:?}", rsi_result);

    // Calculate MACD
    let (macd_line, signal_line, histogram) = macd(&prices, 12, 26, 9)?;
    println!("MACD Line: {:?}", macd_line);

    Ok(())
}
```

### 4. Run Your Project

```bash
cargo run
```

## 🧪 Development Dependencies

For development and testing, you might want to add these dependencies:

```toml
[dev-dependencies]
criterion = "0.5"      # For benchmarking
approx = "0.5"         # For floating-point comparisons
serde_json = "1.0"     # For JSON data handling
```

## 📊 Verification

### Test Installation

Create a simple test to verify everything works:

```rust
#[cfg(test)]
mod tests {
    use ta_rust::prelude::*;

    #[test]
    fn test_installation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = sma(&data, 3).unwrap();
        
        // Should have NaN for first two values, then 2.0, 3.0, 4.0
        assert!(result[0].is_nan());
        assert!(result[1].is_nan());
        assert!((result[2] - 2.0).abs() < 1e-8);
        assert!((result[3] - 3.0).abs() < 1e-8);
        assert!((result[4] - 4.0).abs() < 1e-8);
    }
}
```

Run the test:

```bash
cargo test test_installation
```

## 🔍 Common Issues

### Issue 1: Compilation Errors

**Problem**: Compilation fails with missing dependencies.

**Solution**: Make sure you have the latest Rust version:
```bash
rustup update
```

### Issue 2: No-std Environment

**Problem**: Standard library functions not available.

**Solution**: Use the no-std feature:
```toml
[dependencies]
ta-rust = { version = "0.1.0", default-features = false, features = ["no_std"] }
```

### Issue 3: Performance Issues

**Problem**: Slow compilation or runtime performance.

**Solution**: Use release mode for production:
```bash
cargo build --release
cargo run --release
```

## 🚀 Next Steps

After successful installation:

1. Read the [Quick Start Guide](quick-start.md)
2. Explore the [API Overview](api-overview.md)
3. Check out [Performance Guide](performance.md) for optimization tips
4. Browse [Indicator Documentation](indicators/) for specific functions

## 📚 Additional Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/) - Learn more about Cargo
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust programming
- [TA-Rust Examples](https://github.com/pixelbrow720/ta-rust/tree/main/examples) - More usage examples

## 💡 Tips

1. **Use release builds** for production applications
2. **Enable optimizations** in Cargo.toml for better performance:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   ```
3. **Pin versions** in production to avoid breaking changes
4. **Use workspaces** for larger projects with multiple crates

## 🆘 Getting Help

If you encounter issues:

1. Check the [GitHub Issues](https://github.com/pixelbrow720/ta-rust/issues)
2. Read the [FAQ](faq.md)
3. Contact the maintainer:
   - Email: pixelbrow13@gmail.com
   - Telegram: [@liu483](https://t.me/liu483)
   - GitHub: [@pixelbrow720](https://github.com/pixelbrow720)