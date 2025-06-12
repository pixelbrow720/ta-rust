//! Benchmarks for TA-Rust functions

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

// Benchmark data generators
fn generate_test_data(size: usize) -> Vec<f64> {
    (0..size).map(|i| (i as f64 * 0.1).sin() * 100.0 + 100.0).collect()
}

#[allow(dead_code)]
fn generate_ohlc_data(size: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let base_prices = generate_test_data(size);
    let open: Vec<f64> = base_prices.iter().map(|&p| p * 0.99).collect();
    let high: Vec<f64> = base_prices.iter().map(|&p| p * 1.02).collect();
    let low: Vec<f64> = base_prices.iter().map(|&p| p * 0.98).collect();
    let close = base_prices;
    (open, high, low, close)
}

// Placeholder benchmark functions (will be implemented as functions are added)
fn bench_placeholder(c: &mut Criterion) {
    let data = generate_test_data(1000);
    
    c.bench_function("placeholder", |b| {
        b.iter(|| {
            // Placeholder for actual function benchmarks
            black_box(&data);
        })
    });
}

// Benchmark different data sizes
fn bench_data_sizes(c: &mut Criterion) {
    let sizes = vec![100, 500, 1000, 5000, 10000];
    
    for size in sizes {
        let data = generate_test_data(size);
        
        c.bench_with_input(
            BenchmarkId::new("data_generation", size),
            &size,
            |b, &_size| {
                b.iter(|| {
                    black_box(&data);
                })
            },
        );
    }
}

// Memory allocation benchmarks
fn bench_memory_allocation(c: &mut Criterion) {
    c.bench_function("vec_allocation_1000", |b| {
        b.iter(|| {
            let v: Vec<f64> = vec![0.0; 1000];
            black_box(v);
        })
    });
    
    c.bench_function("vec_with_capacity_1000", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(1000);
            for i in 0..1000 {
                v.push(i as f64);
            }
            black_box(v);
        })
    });
}

// Future benchmark groups will be added here as functions are implemented:
// - bench_overlap_studies
// - bench_momentum_indicators  
// - bench_volume_indicators
// - bench_volatility_indicators
// - bench_pattern_recognition
// - bench_math_functions

criterion_group!(
    benches,
    bench_placeholder,
    bench_data_sizes,
    bench_memory_allocation
);

criterion_main!(benches);