//! Benchmarking syn

use criterion::{criterion_group, criterion_main, Criterion};
use shared::{Attribute, Struct};
use std::hint::black_box;

fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn xorshift(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

fn generate_structs() -> Vec<Struct> {
    let mut structs = Vec::new();
    for i in 0..1000 {
        let mut attributes = Vec::new();
        let number_of_attributes = xorshift(splitmix(splitmix(i))) % 10 + 1;
        for j in 0..number_of_attributes {
            attributes.push(Attribute {
                name: format!("field{}", j),
                r#type: if j % 2 == 0 {
                    "String".to_string()
                } else {
                    "i32".to_string()
                },
                optional: j % 3 == 0,
            });
        }
        structs.push(Struct {
            name: format!("MyStruct{}", i),
            attributes,
        });
    }
    structs
}

/// Benchmarks group.
fn syn_benchmarks(c: &mut Criterion) {
    let structs = generate_structs();
    c.bench_function("using_syn_full", |b| {
        b.iter(|| {
            for s in &structs {
                let _ = black_box(using_syn_full::implements_struct_and_foo(s));
            }
        })
    });
    c.bench_function("using_syn_partial", |b| {
        b.iter(|| {
            for s in &structs {
                let _ = black_box(using_syn_partial::implements_struct_and_foo(s));
            }
        })
    });
    c.bench_function("using_no_parsing_full_syn", |b| {
        b.iter(|| {
            for s in &structs {
                let _ = black_box(using_no_parsing_full_syn::implements_struct_and_foo(s));
            }
        })
    });
}

criterion_group!(benches, syn_benchmarks);
criterion_main!(benches);
