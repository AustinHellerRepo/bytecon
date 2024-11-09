use std::error::Error;
use burn::tensor::{quantization::{AffineQuantization, SymmetricQuantization}, DType};
use crate::ByteConverter;

impl ByteConverter for DType {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {

        // enum variant byte
        match self {
            burn::tensor::DType::F64 => {
                bytes.push(0);
            },
            burn::tensor::DType::F32 => {
                bytes.push(1);
            },
            burn::tensor::DType::F16 => {
                bytes.push(2);
            },
            burn::tensor::DType::BF16 => {
                bytes.push(3);
            },
            burn::tensor::DType::I64 => {
                bytes.push(4);
            },
            burn::tensor::DType::I32 => {
                bytes.push(5);
            },
            burn::tensor::DType::I16 => {
                bytes.push(6);
            },
            burn::tensor::DType::I8 => {
                bytes.push(7);
            },
            burn::tensor::DType::U64 => {
                bytes.push(8);
            },
            burn::tensor::DType::U32 => {
                bytes.push(9);
            },
            burn::tensor::DType::U8 => {
                bytes.push(10);
            },
            burn::tensor::DType::Bool => {
                bytes.push(11);
            },
            burn::tensor::DType::QFloat(strategy) => {
                bytes.push(12);
                match strategy {
                    burn::tensor::quantization::QuantizationStrategy::PerTensorAffineInt8(affine_quantization) => {
                        bytes.push(0);

                        // f32
                        affine_quantization.scale.append_to_bytes(bytes)?;

                        // i8
                        affine_quantization.offset.append_to_bytes(bytes)?;
                    },
                    burn::tensor::quantization::QuantizationStrategy::PerTensorSymmetricInt8(symmetric_quantization) => {
                        bytes.push(1);

                        // f32
                        symmetric_quantization.scale.append_to_bytes(bytes)?;
                    },
                }
            },
        }
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        
        let dtype_index = u8::extract_from_bytes(bytes, index)?;

        let dtype = match dtype_index {
            0 => {
                burn::tensor::DType::F64
            },
            1 => {
                burn::tensor::DType::F32
            },
            2 => {
                burn::tensor::DType::F16
            },
            3 => {
                burn::tensor::DType::BF16
            },
            4 => {
                burn::tensor::DType::I64
            },
            5 => {
                burn::tensor::DType::I32
            },
            6 => {
                burn::tensor::DType::I16
            },
            7 => {
                burn::tensor::DType::I8
            },
            8 => {
                burn::tensor::DType::U64
            },
            9 => {
                burn::tensor::DType::U32
            },
            10 => {
                burn::tensor::DType::U8
            },
            11 => {
                burn::tensor::DType::Bool
            },
            12 => {

                let strategy_index = u8::extract_from_bytes(bytes, index)?;

                match strategy_index {
                    0 => {
                        
                        let scale = f32::extract_from_bytes(bytes, index)?;

                        let offset = i8::extract_from_bytes(bytes, index)?;

                        burn::tensor::DType::QFloat(burn::tensor::quantization::QuantizationStrategy::PerTensorAffineInt8(AffineQuantization::init(scale, offset)))
                    },
                    1 => {
                        let scale = f32::extract_from_bytes(bytes, index)?;

                        burn::tensor::DType::QFloat(burn::tensor::quantization::QuantizationStrategy::PerTensorSymmetricInt8(SymmetricQuantization::init(scale)))
                    },
                    _ => {
                        panic!("Unexpected byte representing strategy.");
                    },
                }
            },
            _ => {
                panic!("Unexpected byte representing tensor DType");
            },
        };
        
        Ok(dtype)
    }
}