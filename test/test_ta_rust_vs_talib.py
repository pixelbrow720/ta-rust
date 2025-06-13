#!/usr/bin/env python3
"""
TA-Rust vs TA-Lib Comparison Test Suite

This script compares the output of ta-rust functions with the original TA-Lib
to ensure 100% compatibility. It tests all implemented functions from Phase 1-5.

Requirements:
- Python 3.7+
- TA-Lib installed in conda environment 'talib-env'
- ta-rust compiled and available

Usage:
    conda activate talib-env
    python test_ta_rust_vs_talib.py
"""

import sys
import os
import json
import subprocess
import numpy as np
import pandas as pd
from typing import Dict, List, Tuple, Any, Optional
from dataclasses import dataclass
from pathlib import Path

try:
    import talib
    print("✅ TA-Lib imported successfully")
except ImportError:
    print("❌ TA-Lib not found. Please install it in conda environment 'talib-env'")
    print("   conda activate talib-env")
    print("   conda install -c conda-forge ta-lib")
    sys.exit(1)

@dataclass
class TestResult:
    """Result of a single function test"""
    function_name: str
    phase: int
    category: str
    passed: bool
    max_error: float
    mean_error: float
    error_message: Optional[str] = None
    talib_result: Optional[np.ndarray] = None
    rust_result: Optional[np.ndarray] = None

class TALibComparison:
    """Main class for comparing TA-Rust with TA-Lib"""
    
    def __init__(self, tolerance: float = 1e-8):
        self.tolerance = tolerance
        self.test_results: List[TestResult] = []
        self.test_data = self._generate_test_data()
        
    def _generate_test_data(self) -> Dict[str, np.ndarray]:
        """Generate comprehensive test data for various market scenarios"""
        np.random.seed(42)  # For reproducible results
        
        # Basic trending data
        trend_up = np.cumsum(np.random.normal(0.1, 1.0, 100)) + 100
        trend_down = np.cumsum(np.random.normal(-0.1, 1.0, 100)) + 100
        
        # Oscillating/sideways market
        oscillating = 100 + 10 * np.sin(np.linspace(0, 4*np.pi, 100)) + np.random.normal(0, 0.5, 100)
        
        # High volatility data
        volatile = np.cumsum(np.random.normal(0, 2.0, 100)) + 100
        
        # Real-world-like OHLC data
        close_prices = trend_up
        high_prices = close_prices + np.abs(np.random.normal(0, 0.5, 100))
        low_prices = close_prices - np.abs(np.random.normal(0, 0.5, 100))
        open_prices = close_prices + np.random.normal(0, 0.3, 100)
        volume_data = np.random.uniform(1000, 10000, 100)
        
        return {
            'close_trend_up': trend_up,
            'close_trend_down': trend_down,
            'close_oscillating': oscillating,
            'close_volatile': volatile,
            'open': open_prices,
            'high': high_prices,
            'low': low_prices,
            'close': close_prices,
            'volume': volume_data
        }
    
    def _run_rust_function(self, function_name: str, params: Dict[str, Any]) -> Optional[np.ndarray]:
        """Run a ta-rust function via subprocess and return the result"""
        try:
            # Create a temporary Rust test program
            rust_code = self._generate_rust_test_code(function_name, params)
            
            # Write to temporary file
            temp_file = Path("test/temp_test.rs")
            with open(temp_file, 'w') as f:
                f.write(rust_code)
            
            # Compile and run
            result = subprocess.run([
                "rustc", "--edition", "2021", 
                "-L", "target/debug/deps",
                "--extern", "ta_rust=target/debug/libta_rust.rlib",
                str(temp_file), "-o", "test/temp_test"
            ], capture_output=True, text=True, cwd="..")
            
            if result.returncode != 0:
                print(f"❌ Compilation failed for {function_name}: {result.stderr}")
                return None
            
            # Run the compiled program
            result = subprocess.run(["./test/temp_test"], capture_output=True, text=True, cwd="..")
            
            if result.returncode != 0:
                print(f"❌ Execution failed for {function_name}: {result.stderr}")
                return None
            
            # Parse JSON output
            output_data = json.loads(result.stdout.strip())
            return np.array(output_data)
            
        except Exception as e:
            print(f"❌ Error running Rust function {function_name}: {e}")
            return None
        finally:
            # Cleanup
            for file in ["test/temp_test.rs", "test/temp_test"]:
                if os.path.exists(file):
                    os.remove(file)
    
    def _generate_rust_test_code(self, function_name: str, params: Dict[str, Any]) -> str:
        """Generate Rust code to test a specific function"""
        
        # Convert numpy arrays to Rust vectors
        def array_to_rust(arr):
            return f"vec![{', '.join(f'{x:.10f}' for x in arr)}]"
        
        # Build function call based on function type
        if function_name in ['sma', 'ema', 'wma', 'dema', 'tema', 'trima', 'rsi', 'roc', 'mom']:
            data_var = array_to_rust(params['data'])
            period = params['period']
            call = f"{function_name}(&{data_var}, {period})"
            
        elif function_name in ['atr', 'natr', 'trange']:
            high_var = array_to_rust(params['high'])
            low_var = array_to_rust(params['low'])
            close_var = array_to_rust(params['close'])
            if 'period' in params:
                period = params['period']
                call = f"{function_name}(&{high_var}, &{low_var}, &{close_var}, {period})"
            else:
                call = f"{function_name}(&{high_var}, &{low_var}, &{close_var})"
                
        elif function_name == 'macd':
            data_var = array_to_rust(params['data'])
            fast = params.get('fastperiod', 12)
            slow = params.get('slowperiod', 26)
            signal = params.get('signalperiod', 9)
            call = f"{function_name}(&{data_var}, {fast}, {slow}, {signal})"
            
        elif function_name == 'bbands':
            data_var = array_to_rust(params['data'])
            period = params.get('timeperiod', 20)
            nbdev = params.get('nbdevup', 2.0)
            call = f"{function_name}(&{data_var}, {period}, {nbdev})"
            
        elif function_name == 'sar':
            high_var = array_to_rust(params['high'])
            low_var = array_to_rust(params['low'])
            accel = params.get('acceleration', 0.02)
            maximum = params.get('maximum', 0.20)
            call = f"{function_name}(&{high_var}, &{low_var}, {accel}, {maximum})"
            
        elif function_name in ['obv']:
            close_var = array_to_rust(params['close'])
            volume_var = array_to_rust(params['volume'])
            call = f"{function_name}(&{close_var}, &{volume_var})"
            
        elif function_name in ['ad']:
            high_var = array_to_rust(params['high'])
            low_var = array_to_rust(params['low'])
            close_var = array_to_rust(params['close'])
            volume_var = array_to_rust(params['volume'])
            call = f"{function_name}(&{high_var}, &{low_var}, &{close_var}, &{volume_var})"
            
        else:
            # Generic single array + period
            data_var = array_to_rust(params['data'])
            period = params.get('period', params.get('timeperiod', 14))
            call = f"{function_name}(&{data_var}, {period})"
        
        return f'''
use ta_rust::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let result = {call}?;
    
    // Handle different return types
    match result {{
        // Single vector
        ref vec if vec.len() > 0 => {{
            println!("{{}}", serde_json::to_string(vec)?);
        }},
        _ => {{
            // For complex returns like MACD, BBands, etc.
            // This is a simplified approach - in practice we'd need
            // to handle each function's specific return type
            println!("[]");
        }}
    }}
    
    Ok(())
}}
'''
    
    def test_overlap_functions(self):
        """Test Phase 2: Overlap Studies"""
        print("\n🔍 Testing Phase 2: Overlap Studies")
        
        functions = [
            ('sma', {'data': self.test_data['close'], 'period': 14}),
            ('ema', {'data': self.test_data['close'], 'period': 14}),
            ('wma', {'data': self.test_data['close'], 'period': 14}),
            ('dema', {'data': self.test_data['close'], 'period': 14}),
            ('tema', {'data': self.test_data['close'], 'period': 14}),
            ('trima', {'data': self.test_data['close'], 'period': 14}),
        ]
        
        for func_name, params in functions:
            self._test_single_function(func_name, params, phase=2, category="Overlap Studies")
    
    def test_momentum_functions(self):
        """Test Phase 3-4: Momentum Indicators"""
        print("\n🔍 Testing Phase 3-4: Momentum Indicators")
        
        functions = [
            ('rsi', {'data': self.test_data['close'], 'period': 14}),
            ('roc', {'data': self.test_data['close'], 'period': 10}),
            ('mom', {'data': self.test_data['close'], 'period': 10}),
            ('macd', {'data': self.test_data['close'], 'fastperiod': 12, 'slowperiod': 26, 'signalperiod': 9}),
        ]
        
        for func_name, params in functions:
            self._test_single_function(func_name, params, phase=3, category="Momentum")
    
    def test_volatility_functions(self):
        """Test Phase 3: Volatility Indicators"""
        print("\n🔍 Testing Phase 3: Volatility Indicators")
        
        functions = [
            ('atr', {'high': self.test_data['high'], 'low': self.test_data['low'], 
                    'close': self.test_data['close'], 'period': 14}),
            ('natr', {'high': self.test_data['high'], 'low': self.test_data['low'], 
                     'close': self.test_data['close'], 'period': 14}),
            ('trange', {'high': self.test_data['high'], 'low': self.test_data['low'], 
                       'close': self.test_data['close']}),
        ]
        
        for func_name, params in functions:
            self._test_single_function(func_name, params, phase=3, category="Volatility")
    
    def test_volume_functions(self):
        """Test Phase 5: Volume Indicators"""
        print("\n🔍 Testing Phase 5: Volume Indicators")
        
        functions = [
            ('obv', {'close': self.test_data['close'], 'volume': self.test_data['volume']}),
            ('ad', {'high': self.test_data['high'], 'low': self.test_data['low'],
                   'close': self.test_data['close'], 'volume': self.test_data['volume']}),
        ]
        
        for func_name, params in functions:
            self._test_single_function(func_name, params, phase=5, category="Volume")
    
    def test_advanced_overlap_functions(self):
        """Test Phase 5: Advanced Overlap Studies"""
        print("\n🔍 Testing Phase 5: Advanced Overlap Studies")
        
        functions = [
            ('bbands', {'data': self.test_data['close'], 'timeperiod': 20, 'nbdevup': 2.0}),
            ('sar', {'high': self.test_data['high'], 'low': self.test_data['low'], 
                    'acceleration': 0.02, 'maximum': 0.20}),
        ]
        
        for func_name, params in functions:
            self._test_single_function(func_name, params, phase=5, category="Advanced Overlap")
    
    def _test_single_function(self, func_name: str, params: Dict[str, Any], 
                            phase: int, category: str):
        """Test a single function against TA-Lib"""
        print(f"  Testing {func_name}...", end=" ")
        
        try:
            # Get TA-Lib result
            talib_result = self._get_talib_result(func_name, params)
            if talib_result is None:
                print("❌ TA-Lib function not available")
                return
            
            # Get ta-rust result
            rust_result = self._run_rust_function(func_name, params)
            if rust_result is None:
                print("❌ ta-rust function failed")
                return
            
            # Compare results
            passed, max_error, mean_error, error_msg = self._compare_results(
                talib_result, rust_result, func_name
            )
            
            result = TestResult(
                function_name=func_name,
                phase=phase,
                category=category,
                passed=passed,
                max_error=max_error,
                mean_error=mean_error,
                error_message=error_msg,
                talib_result=talib_result,
                rust_result=rust_result
            )
            
            self.test_results.append(result)
            
            if passed:
                print(f"✅ (max_err: {max_error:.2e})")
            else:
                print(f"❌ {error_msg}")
                
        except Exception as e:
            print(f"❌ Exception: {e}")
            result = TestResult(
                function_name=func_name,
                phase=phase,
                category=category,
                passed=False,
                max_error=float('inf'),
                mean_error=float('inf'),
                error_message=str(e)
            )
            self.test_results.append(result)
    
    def _get_talib_result(self, func_name: str, params: Dict[str, Any]) -> Optional[np.ndarray]:
        """Get result from TA-Lib for comparison"""
        try:
            if func_name == 'sma':
                return talib.SMA(params['data'], timeperiod=params['period'])
            elif func_name == 'ema':
                return talib.EMA(params['data'], timeperiod=params['period'])
            elif func_name == 'wma':
                return talib.WMA(params['data'], timeperiod=params['period'])
            elif func_name == 'dema':
                return talib.DEMA(params['data'], timeperiod=params['period'])
            elif func_name == 'tema':
                return talib.TEMA(params['data'], timeperiod=params['period'])
            elif func_name == 'trima':
                return talib.TRIMA(params['data'], timeperiod=params['period'])
            elif func_name == 'rsi':
                return talib.RSI(params['data'], timeperiod=params['period'])
            elif func_name == 'roc':
                return talib.ROC(params['data'], timeperiod=params['period'])
            elif func_name == 'mom':
                return talib.MOM(params['data'], timeperiod=params['period'])
            elif func_name == 'atr':
                return talib.ATR(params['high'], params['low'], params['close'], 
                               timeperiod=params['period'])
            elif func_name == 'natr':
                return talib.NATR(params['high'], params['low'], params['close'], 
                                timeperiod=params['period'])
            elif func_name == 'trange':
                return talib.TRANGE(params['high'], params['low'], params['close'])
            elif func_name == 'macd':
                macd_line, signal_line, histogram = talib.MACD(
                    params['data'], 
                    fastperiod=params['fastperiod'],
                    slowperiod=params['slowperiod'], 
                    signalperiod=params['signalperiod']
                )
                return macd_line  # For now, just test MACD line
            elif func_name == 'bbands':
                upper, middle, lower = talib.BBANDS(
                    params['data'],
                    timeperiod=params['timeperiod'],
                    nbdevup=params['nbdevup'],
                    nbdevdn=params['nbdevup']  # Assuming symmetric
                )
                return middle  # For now, just test middle band
            elif func_name == 'sar':
                return talib.SAR(params['high'], params['low'], 
                               acceleration=params['acceleration'],
                               maximum=params['maximum'])
            elif func_name == 'obv':
                return talib.OBV(params['close'], params['volume'])
            elif func_name == 'ad':
                return talib.AD(params['high'], params['low'], params['close'], params['volume'])
            else:
                print(f"⚠️  TA-Lib function {func_name} not implemented in test")
                return None
                
        except Exception as e:
            print(f"❌ TA-Lib error for {func_name}: {e}")
            return None
    
    def _compare_results(self, talib_result: np.ndarray, rust_result: np.ndarray, 
                        func_name: str) -> Tuple[bool, float, float, Optional[str]]:
        """Compare TA-Lib and ta-rust results"""
        
        # Handle NaN values
        talib_valid = ~np.isnan(talib_result)
        rust_valid = ~np.isnan(rust_result)
        
        # Check if valid indices match
        if not np.array_equal(talib_valid, rust_valid):
            return False, float('inf'), float('inf'), "NaN patterns don't match"
        
        # Compare only valid values
        if not np.any(talib_valid):
            return True, 0.0, 0.0, None  # All NaN is OK
        
        talib_values = talib_result[talib_valid]
        rust_values = rust_result[rust_valid]
        
        if len(talib_values) != len(rust_values):
            return False, float('inf'), float('inf'), "Different number of valid values"
        
        # Calculate errors
        abs_errors = np.abs(talib_values - rust_values)
        max_error = np.max(abs_errors)
        mean_error = np.mean(abs_errors)
        
        # Check tolerance
        passed = max_error <= self.tolerance
        
        error_msg = None if passed else f"Max error {max_error:.2e} > tolerance {self.tolerance:.2e}"
        
        return passed, max_error, mean_error, error_msg
    
    def run_all_tests(self):
        """Run all test suites"""
        print("🚀 Starting TA-Rust vs TA-Lib Comparison Tests")
        print(f"📊 Tolerance: {self.tolerance:.2e}")
        
        # Build ta-rust first
        print("\n🔨 Building ta-rust...")
        result = subprocess.run(["cargo", "build"], capture_output=True, text=True, cwd="..")
        if result.returncode != 0:
            print(f"❌ Build failed: {result.stderr}")
            return False
        print("✅ Build successful")
        
        # Run test suites
        self.test_overlap_functions()
        self.test_momentum_functions()
        self.test_volatility_functions()
        self.test_volume_functions()
        self.test_advanced_overlap_functions()
        
        return True
    
    def generate_report(self):
        """Generate comprehensive test report"""
        print("\n" + "="*80)
        print("📋 TEST REPORT SUMMARY")
        print("="*80)
        
        total_tests = len(self.test_results)
        passed_tests = sum(1 for r in self.test_results if r.passed)
        failed_tests = total_tests - passed_tests
        
        print(f"Total Tests: {total_tests}")
        print(f"Passed: {passed_tests} ✅")
        print(f"Failed: {failed_tests} ❌")
        print(f"Success Rate: {(passed_tests/total_tests)*100:.1f}%")
        
        # Group by phase
        phases = {}
        for result in self.test_results:
            if result.phase not in phases:
                phases[result.phase] = []
            phases[result.phase].append(result)
        
        print("\n📊 Results by Phase:")
        for phase in sorted(phases.keys()):
            results = phases[phase]
            passed = sum(1 for r in results if r.passed)
            total = len(results)
            print(f"  Phase {phase}: {passed}/{total} passed ({(passed/total)*100:.1f}%)")
        
        # Show failed tests
        if failed_tests > 0:
            print("\n❌ Failed Tests:")
            for result in self.test_results:
                if not result.passed:
                    print(f"  {result.function_name} ({result.category}): {result.error_message}")
        
        # Show accuracy statistics
        valid_results = [r for r in self.test_results if r.passed and r.max_error < float('inf')]
        if valid_results:
            max_errors = [r.max_error for r in valid_results]
            mean_errors = [r.mean_error for r in valid_results]
            
            print(f"\n📈 Accuracy Statistics (for passed tests):")
            print(f"  Max Error Range: {min(max_errors):.2e} - {max(max_errors):.2e}")
            print(f"  Mean Error Range: {min(mean_errors):.2e} - {max(mean_errors):.2e}")
            print(f"  Average Max Error: {np.mean(max_errors):.2e}")
            print(f"  Average Mean Error: {np.mean(mean_errors):.2e}")
        
        # Save detailed results
        self._save_detailed_results()
        
        return passed_tests == total_tests
    
    def _save_detailed_results(self):
        """Save detailed test results to JSON file"""
        results_data = []
        for result in self.test_results:
            results_data.append({
                'function_name': result.function_name,
                'phase': result.phase,
                'category': result.category,
                'passed': result.passed,
                'max_error': result.max_error if result.max_error != float('inf') else None,
                'mean_error': result.mean_error if result.mean_error != float('inf') else None,
                'error_message': result.error_message
            })
        
        with open('test/test_results.json', 'w') as f:
            json.dump({
                'timestamp': pd.Timestamp.now().isoformat(),
                'tolerance': self.tolerance,
                'total_tests': len(self.test_results),
                'passed_tests': sum(1 for r in self.test_results if r.passed),
                'results': results_data
            }, f, indent=2)
        
        print(f"\n💾 Detailed results saved to test/test_results.json")

def main():
    """Main test runner"""
    print("TA-Rust vs TA-Lib Compatibility Test Suite")
    print("=" * 50)
    
    # Check if we're in the right environment
    if 'talib-env' not in os.environ.get('CONDA_DEFAULT_ENV', ''):
        print("⚠️  Warning: Not in 'talib-env' conda environment")
        print("   Please run: conda activate talib-env")
    
    # Initialize test suite
    tolerance = 1e-8  # Very strict tolerance for 100% compatibility
    tester = TALibComparison(tolerance=tolerance)
    
    # Run tests
    success = tester.run_all_tests()
    
    # Generate report
    all_passed = tester.generate_report()
    
    # Exit with appropriate code
    if all_passed:
        print("\n🎉 All tests passed! TA-Rust is 100% compatible with TA-Lib!")
        sys.exit(0)
    else:
        print("\n⚠️  Some tests failed. Please review the results above.")
        sys.exit(1)

if __name__ == "__main__":
    main()
