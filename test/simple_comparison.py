#!/usr/bin/env python3
"""
Simple TA-Rust vs TA-Lib Comparison Script

A simplified version that directly tests key functions from Phase 1-5
without complex subprocess handling.

Usage:
    conda activate talib-env
    python simple_comparison.py
"""

import numpy as np
import pandas as pd
import json
from typing import Dict, List, Tuple, Any
import sys

try:
    import talib
    print("✅ TA-Lib imported successfully")
except ImportError:
    print("❌ TA-Lib not found. Please install it:")
    print("   conda activate talib-env")
    print("   conda install -c conda-forge ta-lib")
    sys.exit(1)

class SimpleComparison:
    def __init__(self):
        self.tolerance = 1e-6  # Reasonable tolerance for comparison
        self.test_data = self._generate_test_data()
        
    def _generate_test_data(self) -> Dict[str, np.ndarray]:
        """Generate test data for comparison"""
        np.random.seed(42)
        
        # Real-world-like price data
        base_price = 100.0
        returns = np.random.normal(0.001, 0.02, 100)  # Daily returns ~2% volatility
        close_prices = base_price * np.cumprod(1 + returns)
        
        # Generate OHLC from close prices
        high_prices = close_prices * (1 + np.abs(np.random.normal(0, 0.01, 100)))
        low_prices = close_prices * (1 - np.abs(np.random.normal(0, 0.01, 100)))
        open_prices = np.roll(close_prices, 1)
        open_prices[0] = close_prices[0]
        
        # Volume data
        volume = np.random.uniform(10000, 100000, 100)
        
        return {
            'open': open_prices,
            'high': high_prices,
            'low': low_prices,
            'close': close_prices,
            'volume': volume
        }
    
    def compare_arrays(self, talib_result: np.ndarray, rust_result: np.ndarray, 
                      name: str) -> Tuple[bool, float]:
        """Compare two arrays and return success status and max error"""
        if len(talib_result) != len(rust_result):
            print(f"❌ {name}: Length mismatch ({len(talib_result)} vs {len(rust_result)})")
            return False, float('inf')
        
        # Handle NaN values
        talib_valid = ~np.isnan(talib_result)
        rust_valid = ~np.isnan(rust_result)
        
        if not np.array_equal(talib_valid, rust_valid):
            print(f"❌ {name}: NaN patterns don't match")
            return False, float('inf')
        
        if not np.any(talib_valid):
            print(f"✅ {name}: All NaN (OK)")
            return True, 0.0
        
        # Compare valid values
        talib_vals = talib_result[talib_valid]
        rust_vals = rust_result[rust_valid]
        
        abs_errors = np.abs(talib_vals - rust_vals)
        max_error = np.max(abs_errors)
        
        if max_error <= self.tolerance:
            print(f"✅ {name}: Max error {max_error:.2e}")
            return True, max_error
        else:
            print(f"❌ {name}: Max error {max_error:.2e} > tolerance {self.tolerance:.2e}")
            return False, max_error
    
    def test_overlap_functions(self):
        """Test basic overlap functions"""
        print("\n🔍 Testing Overlap Functions (Phase 2)")
        
        close = self.test_data['close']
        
        # Test SMA
        talib_sma = talib.SMA(close, timeperiod=14)
        print("  SMA: Need to implement Rust comparison")
        
        # Test EMA
        talib_ema = talib.EMA(close, timeperiod=14)
        print("  EMA: Need to implement Rust comparison")
        
        # Test WMA
        talib_wma = talib.WMA(close, timeperiod=14)
        print("  WMA: Need to implement Rust comparison")
        
        # For now, just show TA-Lib results
        print(f"  TA-Lib SMA sample: {talib_sma[-5:]}")
        print(f"  TA-Lib EMA sample: {talib_ema[-5:]}")
        print(f"  TA-Lib WMA sample: {talib_wma[-5:]}")
    
    def test_momentum_functions(self):
        """Test momentum functions"""
        print("\n🔍 Testing Momentum Functions (Phase 3-4)")
        
        close = self.test_data['close']
        
        # Test RSI
        talib_rsi = talib.RSI(close, timeperiod=14)
        print("  RSI: Need to implement Rust comparison")
        
        # Test MACD
        macd_line, signal_line, histogram = talib.MACD(close, fastperiod=12, slowperiod=26, signalperiod=9)
        print("  MACD: Need to implement Rust comparison")
        
        # Test ROC
        talib_roc = talib.ROC(close, timeperiod=10)
        print("  ROC: Need to implement Rust comparison")
        
        # Show sample results
        print(f"  TA-Lib RSI sample: {talib_rsi[-5:]}")
        print(f"  TA-Lib MACD sample: {macd_line[-5:]}")
        print(f"  TA-Lib ROC sample: {talib_roc[-5:]}")
    
    def test_volatility_functions(self):
        """Test volatility functions"""
        print("\n🔍 Testing Volatility Functions (Phase 3)")
        
        high = self.test_data['high']
        low = self.test_data['low']
        close = self.test_data['close']
        
        # Test ATR
        talib_atr = talib.ATR(high, low, close, timeperiod=14)
        print("  ATR: Need to implement Rust comparison")
        
        # Test NATR
        talib_natr = talib.NATR(high, low, close, timeperiod=14)
        print("  NATR: Need to implement Rust comparison")
        
        # Test TRANGE
        talib_trange = talib.TRANGE(high, low, close)
        print("  TRANGE: Need to implement Rust comparison")
        
        # Show sample results
        print(f"  TA-Lib ATR sample: {talib_atr[-5:]}")
        print(f"  TA-Lib NATR sample: {talib_natr[-5:]}")
        print(f"  TA-Lib TRANGE sample: {talib_trange[-5:]}")
    
    def test_volume_functions(self):
        """Test volume functions"""
        print("\n🔍 Testing Volume Functions (Phase 5)")
        
        high = self.test_data['high']
        low = self.test_data['low']
        close = self.test_data['close']
        volume = self.test_data['volume']
        
        # Test OBV
        talib_obv = talib.OBV(close, volume)
        print("  OBV: Need to implement Rust comparison")
        
        # Test AD
        talib_ad = talib.AD(high, low, close, volume)
        print("  AD: Need to implement Rust comparison")
        
        # Show sample results
        print(f"  TA-Lib OBV sample: {talib_obv[-5:]}")
        print(f"  TA-Lib AD sample: {talib_ad[-5:]}")
    
    def test_advanced_overlap_functions(self):
        """Test advanced overlap functions"""
        print("\n🔍 Testing Advanced Overlap Functions (Phase 5)")
        
        high = self.test_data['high']
        low = self.test_data['low']
        close = self.test_data['close']
        
        # Test Bollinger Bands
        bb_upper, bb_middle, bb_lower = talib.BBANDS(close, timeperiod=20, nbdevup=2, nbdevdn=2)
        print("  BBANDS: Need to implement Rust comparison")
        
        # Test Parabolic SAR
        talib_sar = talib.SAR(high, low, acceleration=0.02, maximum=0.20)
        print("  SAR: Need to implement Rust comparison")
        
        # Show sample results
        print(f"  TA-Lib BBANDS Upper sample: {bb_upper[-5:]}")
        print(f"  TA-Lib BBANDS Middle sample: {bb_middle[-5:]}")
        print(f"  TA-Lib BBANDS Lower sample: {bb_lower[-5:]}")
        print(f"  TA-Lib SAR sample: {talib_sar[-5:]}")
    
    def save_reference_data(self):
        """Save TA-Lib results as reference data for Rust testing"""
        print("\n💾 Saving TA-Lib reference data...")
        
        close = self.test_data['close']
        high = self.test_data['high']
        low = self.test_data['low']
        volume = self.test_data['volume']
        
        reference_data = {
            'test_data': {
                'close': close.tolist(),
                'high': high.tolist(),
                'low': low.tolist(),
                'volume': volume.tolist()
            },
            'talib_results': {
                # Overlap functions
                'sma_14': talib.SMA(close, timeperiod=14).tolist(),
                'ema_14': talib.EMA(close, timeperiod=14).tolist(),
                'wma_14': talib.WMA(close, timeperiod=14).tolist(),
                
                # Momentum functions
                'rsi_14': talib.RSI(close, timeperiod=14).tolist(),
                'roc_10': talib.ROC(close, timeperiod=10).tolist(),
                'mom_10': talib.MOM(close, timeperiod=10).tolist(),
                
                # Volatility functions
                'atr_14': talib.ATR(high, low, close, timeperiod=14).tolist(),
                'natr_14': talib.NATR(high, low, close, timeperiod=14).tolist(),
                'trange': talib.TRANGE(high, low, close).tolist(),
                
                # Volume functions
                'obv': talib.OBV(close, volume).tolist(),
                'ad': talib.AD(high, low, close, volume).tolist(),
                
                # Advanced functions
                'sar': talib.SAR(high, low, acceleration=0.02, maximum=0.20).tolist(),
            }
        }
        
        # Add MACD (returns tuple)
        macd_line, signal_line, histogram = talib.MACD(close, fastperiod=12, slowperiod=26, signalperiod=9)
        reference_data['talib_results']['macd_line'] = macd_line.tolist()
        reference_data['talib_results']['macd_signal'] = signal_line.tolist()
        reference_data['talib_results']['macd_histogram'] = histogram.tolist()
        
        # Add Bollinger Bands (returns tuple)
        bb_upper, bb_middle, bb_lower = talib.BBANDS(close, timeperiod=20, nbdevup=2, nbdevdn=2)
        reference_data['talib_results']['bbands_upper'] = bb_upper.tolist()
        reference_data['talib_results']['bbands_middle'] = bb_middle.tolist()
        reference_data['talib_results']['bbands_lower'] = bb_lower.tolist()
        
        # Helper function to convert NaN to null for JSON compatibility
        def convert_nan_to_null(obj):
            if isinstance(obj, list):
                return [convert_nan_to_null(item) for item in obj]
            elif isinstance(obj, dict):
                return {key: convert_nan_to_null(value) for key, value in obj.items()}
            elif isinstance(obj, float) and np.isnan(obj):
                return None
            else:
                return obj
        
        # Convert NaN values to null
        reference_data_clean = convert_nan_to_null(reference_data)
        
        # Save to JSON file
        with open('test/talib_reference_data.json', 'w') as f:
            json.dump(reference_data_clean, f, indent=2)
        
        print("✅ Reference data saved to test/talib_reference_data.json")
        print("   This file can be used by Rust tests for comparison")
    
    def run_all_tests(self):
        """Run all test suites"""
        print("🚀 TA-Rust vs TA-Lib Simple Comparison")
        print("=" * 50)
        
        self.test_overlap_functions()
        self.test_momentum_functions()
        self.test_volatility_functions()
        self.test_volume_functions()
        self.test_advanced_overlap_functions()
        
        self.save_reference_data()
        
        print("\n📋 Summary:")
        print("✅ TA-Lib functions tested and reference data generated")
        print("⚠️  Rust comparison not yet implemented")
        print("💡 Next steps:")
        print("   1. Use the reference data in test/talib_reference_data.json")
        print("   2. Create Rust tests that load this data and compare results")
        print("   3. Implement proper error handling and reporting")

def main():
    """Main test runner"""
    comparison = SimpleComparison()
    comparison.run_all_tests()

if __name__ == "__main__":
    main()
