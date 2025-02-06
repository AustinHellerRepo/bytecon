use criterion::{black_box, criterion_group, criterion_main, Criterion};
use serde::{Serialize, Deserialize};
use std::error::Error;
use bytecon::ByteConverter;

#[cfg(feature = "bincode")]
use bincode;

/// Test struct for benchmarking
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct TestStruct {
    id: u64,
    name: String,
    values: Vec<i32>,
}

// Implement ByteConverter manually for TestStruct
impl ByteConverter for TestStruct {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.id.append_to_bytes(bytes)?;
        self.name.append_to_bytes(bytes)?;
        self.values.append_to_bytes(bytes)?;
        Ok(())
    }

    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let id = u64::extract_from_bytes(bytes, index)?;
        let name = String::extract_from_bytes(bytes, index)?;
        let values = Vec::<i32>::extract_from_bytes(bytes, index)?;
        Ok(Self { id, name, values })
    }
}

/// Benchmark: Serialization with bincode
#[cfg(feature = "bincode")]
fn bench_bincode_serialize(c: &mut Criterion) {
    let test_data = TestStruct {
        id: 42,
        name: "Hello, world!".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };

    c.bench_function("bincode_serialize", |b| {
        b.iter(|| {
            let _encoded = bincode::serialize(black_box(&test_data)).unwrap();
        });
    });
}

/// Benchmark: Deserialization with bincode
#[cfg(feature = "bincode")]
fn bench_bincode_deserialize(c: &mut Criterion) {
    let test_data = TestStruct {
        id: 42,
        name: "Hello, world!".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };
    let encoded = bincode::serialize(&test_data).unwrap();

    c.bench_function("bincode_deserialize", |b| {
        b.iter(|| {
            let _: TestStruct = bincode::deserialize(black_box(&encoded)).unwrap();
        });
    });
}

/// Benchmark: Serialization with ByteConverter
fn bench_bytecon_serialize(c: &mut Criterion) {
    let test_data = TestStruct {
        id: 42,
        name: "Hello, world!".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };

    c.bench_function("bytecon_serialize", |b| {
        b.iter(|| {
            let _encoded = test_data.to_vec_bytes_with_capacity(1024).unwrap();
        });
    });
}

/// Benchmark: Deserialization with ByteConverter
fn bench_bytecon_deserialize(c: &mut Criterion) {
    let test_data = TestStruct {
        id: 42,
        name: "Hello, world!".to_string(),
        values: vec![1, 2, 3, 4, 5],
    };
    let encoded = test_data.to_vec_bytes_with_capacity(1024).unwrap();

    c.bench_function("bytecon_deserialize", |b| {
        b.iter(|| {
            let _: TestStruct = TestStruct::deserialize_from_bytes(black_box(&encoded)).unwrap();
        });
    });
}

// Group all benchmarks
criterion_group!(
    benches,
    bench_bincode_serialize,
    bench_bincode_deserialize,
    bench_bytecon_serialize,
    bench_bytecon_deserialize
);
criterion_main!(benches);
