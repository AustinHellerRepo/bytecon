use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
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


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
enum LargeEnum {
    A(i8, LargeStruct),
    B(i16, LargeStruct),
    C(i32, LargeStruct),
    D(i64, LargeStruct),
    E(i128, LargeStruct),
    F(isize, LargeStruct),
    G(u8, LargeStruct),
    H(u16, LargeStruct),
    I(u32, LargeStruct),
    J(u64, LargeStruct),
    K(u128, LargeStruct),
    L(usize, LargeStruct),
    M(String, LargeStruct),
}

impl ByteConverter for LargeEnum {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Self::A(a, b) => {
                0u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::B(a, b) => {
                1u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::C(a, b) => {
                2u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::D(a, b) => {
                3u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::E(a, b) => {
                4u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::F(a, b) => {
                5u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::G(a, b) => {
                6u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::H(a, b) => {
                7u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::I(a, b) => {
                8u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::J(a, b) => {
                9u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::K(a, b) => {
                10u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::L(a, b) => {
                11u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
            Self::M(a, b) => {
                12u8.append_to_bytes(bytes)?;
                a.append_to_bytes(bytes)?;
                b.append_to_bytes(bytes)?;
            },
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => Ok(Self::A(
                i8::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            1u8 => Ok(Self::B(
                i16::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            2u8 => Ok(Self::C(
                i32::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            3u8 => Ok(Self::D(
                i64::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            4u8 => Ok(Self::E(
                i128::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            5u8 => Ok(Self::F(
                isize::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            6u8 => Ok(Self::G(
                u8::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            7u8 => Ok(Self::H(
                u16::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            8u8 => Ok(Self::I(
                u32::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            9u8 => Ok(Self::J(
                u64::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            10u8 => Ok(Self::K(
                u128::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            11u8 => Ok(Self::L(
                usize::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            12u8 => Ok(Self::M(
                String::extract_from_bytes(bytes, index)?,
                LargeStruct::extract_from_bytes(bytes, index)?,
            )),
            _ => Err("Unexpected enum variant bytes.".into()),
        }
    }
}

impl LargeEnum {
    fn new(depth: usize, rng: &mut ChaCha8Rng) -> Self {
        let enum_variant_byte: u8 = rng.gen_range(0..=12);
        match enum_variant_byte {
            0u8 => Self::A(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            1u8 => Self::B(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            2u8 => Self::C(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            3u8 => Self::D(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            4u8 => Self::E(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            5u8 => Self::F(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            6u8 => Self::G(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            7u8 => Self::H(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            8u8 => Self::I(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            9u8 => Self::J(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            10u8 => Self::K(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            11u8 => Self::L(
                rand::random(),
                LargeStruct::new(depth, rng),
            ),
            12u8 => Self::M(
                String::from("this is a string within the enum type."),
                LargeStruct::new(depth, rng),
            ),
            _ => panic!("unexpected enum variant byte."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct LargeStruct {
    a: i8,
    b: i16,
    c: i32,
    d: i64,
    e: i128,
    f: isize,
    g: u8,
    h: u16,
    i: u32,
    j: u64,
    k: u128,
    l: usize,
    m: String,
    n: Option<Box<LargeEnum>>,
}

impl ByteConverter for LargeStruct {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.a.append_to_bytes(bytes)?;
        self.b.append_to_bytes(bytes)?;
        self.c.append_to_bytes(bytes)?;
        self.d.append_to_bytes(bytes)?;
        self.e.append_to_bytes(bytes)?;
        self.f.append_to_bytes(bytes)?;
        self.g.append_to_bytes(bytes)?;
        self.h.append_to_bytes(bytes)?;
        self.i.append_to_bytes(bytes)?;
        self.j.append_to_bytes(bytes)?;
        self.k.append_to_bytes(bytes)?;
        self.l.append_to_bytes(bytes)?;
        self.m.append_to_bytes(bytes)?;
        self.n.append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Self {
            a: i8::extract_from_bytes(bytes, index)?,
            b: i16::extract_from_bytes(bytes, index)?,
            c: i32::extract_from_bytes(bytes, index)?,
            d: i64::extract_from_bytes(bytes, index)?,
            e: i128::extract_from_bytes(bytes, index)?,
            f: isize::extract_from_bytes(bytes, index)?,
            g: u8::extract_from_bytes(bytes, index)?,
            h: u16::extract_from_bytes(bytes, index)?,
            i: u32::extract_from_bytes(bytes, index)?,
            j: u64::extract_from_bytes(bytes, index)?,
            k: u128::extract_from_bytes(bytes, index)?,
            l: usize::extract_from_bytes(bytes, index)?,
            m: String::extract_from_bytes(bytes, index)?,
            n: Option::<Box<LargeEnum>>::extract_from_bytes(bytes, index)?,
        })
    }
}

impl LargeStruct {
    fn new(depth: usize, rng: &mut ChaCha8Rng) -> Self {
        if depth == 0 {
            Self {
                a: rng.gen(),
                b: rng.gen(),
                c: rng.gen(),
                d: rng.gen(),
                e: rng.gen(),
                f: rng.gen(),
                g: rng.gen(),
                h: rng.gen(),
                i: rng.gen(),
                j: rng.gen(),
                k: rng.gen(),
                l: rng.gen(),
                m: String::from("final layer of the process is here"),
                n: None,
            }
        }
        else {
            Self {
                a: rng.gen(),
                b: rng.gen(),
                c: rng.gen(),
                d: rng.gen(),
                e: rng.gen(),
                f: rng.gen(),
                g: rng.gen(),
                h: rng.gen(),
                i: rng.gen(),
                j: rng.gen(),
                k: rng.gen(),
                l: rng.gen(),
                m: String::from("final layer of the process is here"),
                n: Some(Box::new(LargeEnum::new(depth - 1, rng))),
            }
        }
    }
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

/// Benchmark: Serialization with bincode (LargeStruct)
#[cfg(feature = "bincode")]
fn bench_large_bincode_serialize(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
    let test_data = LargeStruct::new(50, &mut rng);

    c.bench_function("bincode_serialize_large", |b| {
        b.iter(|| {
            let _encoded = bincode::serialize(black_box(&test_data)).unwrap();
        });
    });
}

/// Benchmark: Deserialization with bincode (LargeStruct)
#[cfg(feature = "bincode")]
fn bench_large_bincode_deserialize(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
    let test_data = LargeStruct::new(50, &mut rng);
    let encoded = bincode::serialize(&test_data).unwrap();

    c.bench_function("bincode_deserialize_large", |b| {
        b.iter(|| {
            let _: LargeStruct = bincode::deserialize(black_box(&encoded)).unwrap();
        });
    });
}

/// Benchmark: Serialization with ByteConverter (LargeStruct)
fn bench_large_bytecon_serialize(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
    let test_data = LargeStruct::new(50, &mut rng);

    c.bench_function("bytecon_serialize_large", |b| {
        b.iter(|| {
            let _encoded = test_data.to_vec_bytes_with_capacity(1024).unwrap();
        });
    });
}

/// Benchmark: Deserialization with ByteConverter (LargeStruct)
fn bench_large_bytecon_deserialize(c: &mut Criterion) {
    let mut rng = ChaCha8Rng::from_seed([0u8; 32]);
    let test_data = LargeStruct::new(50, &mut rng);
    let encoded = test_data.to_vec_bytes_with_capacity(7000).unwrap();
    println!("encoded bytes length: {}", encoded.len());

    c.bench_function("bytecon_deserialize_large", |b| {
        b.iter(|| {
            let _: LargeStruct = LargeStruct::deserialize_from_bytes(black_box(&encoded)).unwrap();
        });
    });
}

/// Benchmark: Serialization with bincode (small)
#[cfg(feature = "bincode")]
fn bench_small_bincode_serialize(c: &mut Criterion) {
    let test_data = 1u8;

    c.bench_function("bincode_serialize_small", |b| {
        b.iter(|| {
            let _encoded = bincode::serialize(black_box(&test_data)).unwrap();
        });
    });
}

/// Benchmark: Deserialization with bincode (small)
#[cfg(feature = "bincode")]
fn bench_small_bincode_deserialize(c: &mut Criterion) {
    let test_data = 1u8;
    let encoded = bincode::serialize(&test_data).unwrap();

    c.bench_function("bincode_deserialize_small", |b| {
        b.iter(|| {
            let _: u8 = bincode::deserialize(black_box(&encoded)).unwrap();
        });
    });
}

/// Benchmark: Serialization with ByteConverter (small)
fn bench_small_bytecon_serialize(c: &mut Criterion) {
    let test_data = 1u8;

    c.bench_function("bytecon_serialize_small", |b| {
        b.iter(|| {
            let _encoded = test_data.to_vec_bytes_with_capacity(10).unwrap();
        });
    });
}

/// Benchmark: Deserialization with ByteConverter (small)
fn bench_small_bytecon_deserialize(c: &mut Criterion) {
    let test_data = 1u8;
    let encoded = test_data.to_vec_bytes_with_capacity(10).unwrap();

    c.bench_function("bytecon_deserialize_small", |b| {
        b.iter(|| {
            let _: u8 = u8::deserialize_from_bytes(black_box(&encoded)).unwrap();
        });
    });
}

// Group all benchmarks
criterion_group!(
    benches,
    bench_bincode_serialize,
    bench_bincode_deserialize,
    bench_bytecon_serialize,
    bench_bytecon_deserialize,
    bench_large_bincode_serialize,
    bench_large_bincode_deserialize,
    bench_large_bytecon_serialize,
    bench_large_bytecon_deserialize,
    bench_small_bincode_serialize,
    bench_small_bincode_deserialize,
    bench_small_bytecon_serialize,
    bench_small_bytecon_deserialize,
);
criterion_main!(benches);
