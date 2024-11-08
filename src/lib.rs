use std::{error::Error, future::Future};

// TODO add a version byte at the front of each append_to_bytes call
//      this can be used to match on within the extract so that changes in format across versions of this crate are unaffected

#[cfg(feature = "burn")]
pub mod burn;

#[cfg(feature = "tokio")]
pub mod tokio;

pub trait ByteConverter {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>>;
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized;
}

pub trait ByteStreamReader {
    fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error>>;
}

pub trait ByteStreamReaderAsync {
    fn read_to_byte_converter<T: ByteConverter>(&mut self) -> impl Future<Output = Result<T, Box<dyn Error>>>;
}

pub trait ByteStreamWriter {
    fn write_from_byte_converter(&mut self, byte_converter: impl ByteConverter) -> Result<(), Box<dyn Error>>;
}

pub trait ByteStreamWriterAsync {
    fn write_from_byte_converter(&mut self, byte_converter: impl ByteConverter) -> impl Future<Output = Result<(), Box<dyn Error>>>;
}

#[derive(thiserror::Error, Debug)]
enum ByteConverterError {
    #[error("Index {index} out of range of bytes array with length {length}.")]
    IndexOutOfRange {
        index: usize,
        length: usize,
    },
    #[error("Failed to extract 64-bit usize on this machine.")]
    FailedToExtractSixtyFourBitUsize,
    #[error("Unexpected number of bytes {bytes_length} for usize that expects either 4 or 8.")]
    UnexpectedSizeOfUsize {
        bytes_length: usize,
    },
    #[error("Unexpected byte value {byte_value} when trying to parse to boolean value of 0 or 1.")]
    UnexpectedByteValueForBoolean {
        byte_value: u8,
    },
}

fn get_single_byte(bytes: &Vec<u8>, index: &mut usize) -> Result<u8, Box<dyn Error>> {
    let bytes_length = bytes.len();
    let index_deref = *index;
    let next_index = index_deref + 1;
    if bytes_length < next_index {
        return Err(ByteConverterError::IndexOutOfRange {
            index: next_index,
            length: bytes_length,
        }.into());
    }
    let byte = bytes[index_deref];
    *index = next_index;
    Ok(byte)
}

fn get_multiple_bytes<'a>(bytes: &'a Vec<u8>, index: &mut usize, size: usize) -> Result<&'a [u8], Box<dyn Error>> {
    let bytes_length = bytes.len();
    let index_deref = *index;
    let next_index = index_deref + size;
    if bytes_length < next_index {
        return Err(ByteConverterError::IndexOutOfRange {
            index: next_index,
            length: bytes_length,
        }.into());
    }
    let multiple_bytes = &bytes[index_deref..next_index];
    *index = next_index;
    Ok(multiple_bytes)
}

impl ByteConverter for bool {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {

        // byte
        bytes.push(if *self { 1 } else { 0 });
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {

        // byte
        let byte = get_single_byte(bytes, index)?;

        let bool = match byte {
            0 => false,
            1 => true,
            _ => {
                return Err(ByteConverterError::UnexpectedByteValueForBoolean {
                    byte_value: byte,
                }.into());
            },
        };

        Ok(bool)
    }
}

impl ByteConverter for f32 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        
        // f32
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        
        // f32
        let float_bytes = get_multiple_bytes(bytes, index, 4)?;
        let float = f32::from_le_bytes(float_bytes.try_into().expect("Failed to splice out bytes."));
        Ok(float)
    }
}

impl ByteConverter for i8 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        
        // i8
        bytes.push((*self) as u8);
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        
        let u8_byte = get_single_byte(bytes, index)?;
        let i8_byte = u8_byte as i8;
        Ok(i8_byte)
    }
}

impl ByteConverter for String {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        let string_bytes = self.as_bytes();
        let string_bytes_length = string_bytes.len();

        // usize
        bytes.extend(&string_bytes_length.to_le_bytes());

        // slice
        bytes.extend_from_slice(string_bytes);
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        
        // usize
        let string_bytes_length = usize::extract_from_bytes(bytes, index)?;
        
        // slice
        let string_bytes = get_multiple_bytes(bytes, index, string_bytes_length)?;

        Ok(Self::from_utf8(string_bytes.to_vec()).expect("Failed to convert bytes to a String."))
    }
}

impl ByteConverter for u8 {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
        bytes.push(*self);
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        let byte = get_single_byte(bytes, index)?;
        Ok(byte)
    }
}

impl ByteConverter for usize {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {

        if cfg!(target_pointer_width = "64") {
            // byte
            bytes.push(8);

            // 8 bytes
            bytes.extend(&(*self as u64).to_le_bytes());
        }
        else {
            // byte
            bytes.push(4);

            // 4 bytes
            bytes.extend(&(*self as u32).to_le_bytes());
        }
        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        
        let usize_length = get_single_byte(bytes, index)? as usize;

        if !cfg!(target_pointer_width = "64") && usize_length == 8 {
            return Err(ByteConverterError::FailedToExtractSixtyFourBitUsize.into());
        }

        let usize_bytes = get_multiple_bytes(bytes, index, usize_length)?;

        let usize_value = match usize_length {
            8 => {
                usize::try_from(u64::from_le_bytes(usize_bytes.try_into().expect("Failed to splice out bytes."))).expect("Failed to cast 8 bytes to usize.")
            },
            4 => {
                usize::try_from(u32::from_le_bytes(usize_bytes.try_into().expect("Failed to splice out bytes."))).expect("Failed to cast 4 bytes to usize.")
            },
            _ => {
                return Err(ByteConverterError::UnexpectedSizeOfUsize {
                    bytes_length: usize_length,
                }.into());
            }
        };

        Ok(usize_value)
    }
}

impl<T: ByteConverter> ByteConverter for Vec<T> {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {

        // usize
        self.len().append_to_bytes(bytes)?;

        for element in self {
            // T
            element.append_to_bytes(bytes)?;
        }

        Ok(())
    }
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
        
        // usize
        let list_length = usize::extract_from_bytes(bytes, index)?;

        let mut list = Vec::with_capacity(list_length);
        for _ in 0..list_length {
            // T
            let list_element = T::extract_from_bytes(bytes, index)?;
            list.push(list_element);
        }

        Ok(list)
    }
}

// TODO uncomment once the nightly stability of overriding specialization is pushed into stable Rust
//impl ByteCon for Vec<usize> {
//    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error>> {
//
//        let self_length = self.len();
//        
//        // usize of list
//        self_length.append_to_bytes(bytes)?;
//
//        if self_length != 0 {
//
//            // usize of element
//            if cfg!(target_pointer_width = "64") {
//
//                // byte
//                bytes.push(8);
//
//                // 8 bytes
//                for element in self {
//                    bytes.extend(&(*element as u64).to_le_bytes());
//                }
//            }
//            else {
//
//                // byte
//                bytes.push(4);
//
//                // 4 bytes
//                for element in self {
//                    bytes.extend(&(*element as u32).to_le_bytes());
//                }
//            }
//        }
//        
//        Ok(())
//    }
//    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error>> where Self: Sized {
//        
//        // usize of list
//        let self_length = usize::extract_from_bytes(bytes, index)?;
//
//        if self_length == 0 {
//            return Ok(Vec::new());
//        }
//
//        // usize length of element
//        let usize_length = get_single_byte(bytes, index)? as usize;
//
//        if !cfg!(target_pointer_width = "64") && usize_length == 8 {
//            return Err(ByteConError::FailedToExtractSixtyFourBitUsize.into());
//        }
//
//        let list = Vec::with_capacity(self_length);
//        for _ in 0..self_length {
//            let usize_bytes = get_multiple_bytes(bytes, index, usize_length)?;
//
//            let usize_value = match usize_length {
//                8 => {
//                    usize::try_from(u64::from_le_bytes(usize_bytes.try_into().expect("Failed to splice out bytes."))).expect("Failed to cast 8 bytes to usize.")
//                },
//                4 => {
//                    usize::try_from(u32::from_le_bytes(usize_bytes.try_into().expect("Failed to splice out bytes."))).expect("Failed to cast 4 bytes to usize.")
//                },
//                _ => {
//                    return Err(ByteConError::UnexpectedSizeOfUsize {
//                        bytes_length: usize_length,
//                    }.into());
//                }
//            };
//        }
//        Ok(list)
//    }
//}

impl<TRead: std::io::Read> ByteStreamReader for TRead {
    fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        let mut chunk = [0u8; 64];

        let mut initial_packet = [0u8; 8];
        let read_result = self.read_exact(&mut initial_packet);
        if let Err(error) = read_result {
            let result: Result<T, Box<dyn Error>> = Err(Box::new(error));
            return result;
        }

        let expected_bytes_length: u64 = u64::from_le_bytes(initial_packet);
        while (bytes.len() as u64) < expected_bytes_length {
            let read_bytes_length_result = self.read(&mut chunk);
            if let Err(error) = read_bytes_length_result {
                let result: Result<T, Box<dyn Error>> = Err(Box::new(error));
                return result;
            }

            let read_bytes_length = read_bytes_length_result.unwrap();

            if read_bytes_length != 0 {
                bytes.extend_from_slice(&chunk[..read_bytes_length]);
            }
        }

        let mut index = 0;
        T::extract_from_bytes(&bytes, &mut index)
    }
}

impl<TWrite: std::io::Write> ByteStreamWriter for TWrite {
    fn write_from_byte_converter(&mut self, byte_converter: impl ByteConverter) -> Result<(), Box<dyn Error>> {
        let mut stream_bytes = Vec::new();
        byte_converter.append_to_bytes(&mut stream_bytes)?;
        self.write(&stream_bytes)?;
        Ok(())
    }
}