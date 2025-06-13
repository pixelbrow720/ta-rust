#!/bin/bash

# TA-Rust Testing Suite Runner
# This script runs comprehensive tests to verify ta-rust compatibility with TA-Lib

set -e  # Exit on any error

echo "🚀 TA-Rust Testing Suite"
echo "========================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the ta-rust root directory"
    exit 1
fi

# Check if conda environment is available
if ! command -v conda &> /dev/null; then
    print_warning "Conda not found. Please ensure conda is installed and in PATH"
fi

# Step 1: Build ta-rust
print_status "Building ta-rust..."
if cargo build --release; then
    print_success "ta-rust build completed"
else
    print_error "ta-rust build failed"
    exit 1
fi

# Step 2: Run existing Rust tests
print_status "Running existing Rust unit tests..."
if cargo test; then
    print_success "Rust unit tests passed"
else
    print_error "Some Rust unit tests failed"
    exit 1
fi

# Step 3: Check if Python and TA-Lib are available
print_status "Checking Python environment..."

# Try to activate talib-env if it exists
if conda info --envs | grep -q "talib-env"; then
    print_status "Found talib-env conda environment"
    CONDA_ENV="talib-env"
else
    print_warning "talib-env conda environment not found, using current environment"
    CONDA_ENV=""
fi

# Function to run Python with conda environment
run_python() {
    if [ -n "$CONDA_ENV" ]; then
        conda run -n "$CONDA_ENV" python "$@"
    else
        python "$@"
    fi
}

# Check if TA-Lib is available
print_status "Checking TA-Lib availability..."
if run_python -c "import talib; print('TA-Lib version:', talib.__version__)" 2>/dev/null; then
    print_success "TA-Lib is available"
else
    print_error "TA-Lib not found. Please install it:"
    echo "  conda create -n talib-env python=3.9"
    echo "  conda activate talib-env"
    echo "  conda install -c conda-forge ta-lib"
    echo "  pip install numpy pandas"
    exit 1
fi

# Step 4: Generate reference data using TA-Lib
print_status "Generating TA-Lib reference data..."
if run_python test/simple_comparison.py; then
    print_success "Reference data generated successfully"
else
    print_error "Failed to generate reference data"
    exit 1
fi

# Step 5: Add serde_json dependency if not present
print_status "Checking dependencies..."
if ! grep -q "serde_json" Cargo.toml; then
    print_status "Adding serde_json dependency..."
    # Add serde_json to Cargo.toml dependencies
    sed -i '/\[dependencies\]/a serde_json = "1.0"' Cargo.toml
    sed -i '/\[dependencies\]/a serde = { version = "1.0", features = ["derive"] }' Cargo.toml
fi

# Step 6: Run Rust comparison test
print_status "Running Rust vs TA-Lib comparison..."
if cargo run --bin rust_comparison_test; then
    print_success "All compatibility tests passed! 🎉"
    TESTS_PASSED=true
else
    print_warning "Some compatibility tests failed"
    TESTS_PASSED=false
fi

# Step 8: Generate final report
print_status "Generating final test report..."

echo ""
echo "📋 FINAL TEST REPORT"
echo "===================="

# Check if test results exist
if [ -f "test/rust_test_results.json" ]; then
    # Extract key metrics from JSON (requires jq, but we'll do it manually)
    TOTAL_TESTS=$(grep -o '"total_tests":[0-9]*' test/rust_test_results.json | cut -d: -f2)
    PASSED_TESTS=$(grep -o '"passed_tests":[0-9]*' test/rust_test_results.json | cut -d: -f2)
    SUCCESS_RATE=$(grep -o '"success_rate":[0-9.]*' test/rust_test_results.json | cut -d: -f2)
    
    echo "Total Tests: $TOTAL_TESTS"
    echo "Passed Tests: $PASSED_TESTS"
    echo "Success Rate: $SUCCESS_RATE%"
else
    print_warning "Detailed test results not found"
fi

echo ""
echo "📁 Generated Files:"
echo "  - test/talib_reference_data.json (TA-Lib reference data)"
echo "  - test/rust_test_results.json (Detailed test results)"

echo ""
echo "🔧 Available Test Scripts:"
echo "  - test/simple_comparison.py (Generate TA-Lib reference data)"
echo "  - test/rust_comparison_test (Rust compatibility test)"
echo "  - test/run_tests.sh (This comprehensive test suite)"

# Cleanup
rm -rf test/temp_test_project
rm -f test/rust_comparison_test

if [ "$TESTS_PASSED" = true ]; then
    print_success "🎉 All tests completed successfully!"
    echo ""
    echo "✅ TA-Rust Phase 1-5 implementation is verified to be compatible with TA-Lib!"
    echo "✅ You can confidently use ta-rust as a drop-in replacement for TA-Lib"
    exit 0
else
    print_warning "⚠️  Some tests failed. Please review the output above."
    echo ""
    echo "� Next steps:"
    echo "  1. Check test/rust_test_results.json for detailed failure information"
    echo "  2. Fix any implementation issues in the failing functions"
    echo "  3. Re-run this test suite to verify fixes"
    exit 1
fi
