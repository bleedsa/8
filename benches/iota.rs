#![allow(non_snake_case)]

use std::{ptr, hint::black_box};
use criterion::{Criterion, criterion_group, criterion_main};
use eight::{pre::*, verb::mon::iota};

fn naive_iota(n: usize) -> *const I {
    let ptr: *mut I = unsafe { xxx::new(n).unwrap() };

    for i in 0..n {
        unsafe {
            ptr::write(ptr.add(i), i as I);
        }
    }

    ptr
}

fn criterion_bench(c: &mut Criterion) {
    let mut B = |N: &str, n: usize| {
        c.bench_function(N, |b| b.iter(|| {
            let ptr = iota(black_box(n));
            unsafe { xxx::free(ptr as *mut I, n) };
        }));
        c.bench_function(&format!("[naive] {}", N), |b| b.iter(|| {
            let ptr = naive_iota(black_box(n));
            unsafe { xxx::free(ptr as *mut I, n) };
        }));
    };

    B("small", 100);
    B("medium", 1_000);
    B("big", 50_000);
    B("huge", 500_000);
}

criterion_group!(benches, criterion_bench);
criterion_main!(benches);
