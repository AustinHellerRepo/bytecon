use std::{any::{Any, TypeId}, cell::RefCell, collections::{HashMap, VecDeque}, error::Error, ffi::CString, future::Future, path::PathBuf, rc::Rc, sync::{Arc, Mutex, RwLock}};

// TODO add a version byte at the front of each append_to_bytes call
//      this can be used to match on within the extract so that changes in format across versions of this crate are unaffected
// TODO replace string errors with usage of ByteConverterError variants

#[cfg(feature = "bevy")]
pub mod bevy;

#[cfg(all(feature = "bincode", not(feature = "burn_dtype")))]
pub mod bincode;

#[cfg(feature = "burn")]
pub mod burn;

#[cfg(feature = "burn_dtype")]
pub mod burn_dtype;

#[cfg(feature = "glam")]
pub mod glam;

#[cfg(feature = "rand")]
pub mod rand;

#[cfg(feature = "rustls")]
pub mod rustls;

#[cfg(feature = "tokio")]
pub mod tokio;

pub trait ByteConverter {
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized;
    #[inline(always)]
    fn to_vec_bytes(&self) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        let mut bytes = Vec::new();
        self.append_to_bytes(&mut bytes)?;
        Ok(bytes)
    }
    #[inline(always)]
    fn to_vec_bytes_with_capacity(&self, capacity: usize) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        let mut bytes = Vec::with_capacity(capacity);
        self.append_to_bytes(&mut bytes)?;
        Ok(bytes)
    }
    #[inline(always)]
    fn clone_via_bytes(&self) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let bytes = self.to_vec_bytes()?;
        Self::deserialize_from_bytes(&bytes)
    }
    // this is useful if you know that there is only one type contained within the collection of bytes
    #[inline(always)]
    fn deserialize_from_bytes(bytes: &Vec<u8>) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let mut index = 0;
        let instance = Self::extract_from_bytes(bytes, &mut index)?;
        if index != bytes.len() {
            return Err("The provided bytes contained more than one instance of a type. Deserializing did not exhaust the total length of the provided bytes.".into());
        }
        Ok(instance)
    }
    // this is useful if you only have a generic TByteConverter as self but you can guarantee that it is a specific type based on your own logic
    #[inline(always)]
    fn cast_via_bytes<TByteConverter>(&self) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>> where TByteConverter: ByteConverter {
        let bytes = self.to_vec_bytes()?;
        TByteConverter::deserialize_from_bytes(&bytes)
    }
}
pub trait ByteStreamReader {
    fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error + Send + Sync + 'static>>;
}

pub trait ByteStreamReaderAsync {
    fn read_to_byte_converter<T: ByteConverter>(&mut self) -> impl Future<Output = Result<T, Box<dyn Error + Send + Sync + 'static>>>;
}

pub trait ByteStreamWriter {
    fn write_from_byte_converter(&mut self, byte_converter: &impl ByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>>;
}

pub trait ByteStreamWriterAsync {
    fn write_from_byte_converter(&mut self, byte_converter: &impl ByteConverter) -> impl Future<Output = Result<(), Box<dyn Error + Send + Sync + 'static>>>;
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
    #[error("Failed to convert from {from_type} to {to_type}.")]
    FailedToConvertToType {
        from_type: String,
        to_type: String,
    },
    #[error("Unexpected byte value {byte_value} when trying to parse to Option value of Some or None.")]
    UnexpectedByteValueForOption {
        byte_value: u8,
    },
    #[error("Failed to lock mutex.")]
    FailedToLockMutex,
}

#[inline(always)]
fn get_single_byte(bytes: &Vec<u8>, index: &mut usize) -> Result<u8, Box<dyn Error + Send + Sync + 'static>> {
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

#[inline(always)]
fn get_multiple_bytes<'a>(bytes: &'a Vec<u8>, index: &mut usize, size: usize) -> Result<&'a [u8], Box<dyn Error + Send + Sync + 'static>> {
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

impl ByteConverter for () {
    #[inline(always)]
    fn append_to_bytes(&self, _: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(_: &Vec<u8>, _: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(())
    }
}

impl ByteConverter for bool {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

        // byte
        bytes.push(if *self { 1 } else { 0 });
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {

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

impl ByteConverter for char {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // char as u32
        ((*self) as u32).append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // char as u32
        let char_value = char::from_u32(u32::extract_from_bytes(bytes, index)?)
            .ok_or(ByteConverterError::FailedToConvertToType {
                from_type: String::from(std::any::type_name::<u32>()),
                to_type: String::from(std::any::type_name::<char>()),
            })?;
        Ok(char_value)
    }
}

impl ByteConverter for f32 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // f32
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // f32
        let float_bytes = get_multiple_bytes(bytes, index, 4)?;
        let float = f32::from_le_bytes(float_bytes.try_into()?);
        Ok(float)
    }
}

impl ByteConverter for f64 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // f64
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // f64
        let float_bytes = get_multiple_bytes(bytes, index, 8)?;
        let float = f64::from_le_bytes(float_bytes.try_into()?);
        Ok(float)
    }
}

impl ByteConverter for i8 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // i8 as u8
        ((*self) as u8).append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // i8 as u8
        Ok(u8::extract_from_bytes(bytes, index)? as i8)
    }
}

impl ByteConverter for i16 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // i16 as u16
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // i16 as u16
        let i16_bytes = get_multiple_bytes(bytes, index, 2)?;
        Ok(i16::from_le_bytes(i16_bytes.try_into()?))
    }
}

impl ByteConverter for i32 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        let i32_bytes = get_multiple_bytes(bytes, index, 4)?;
        Ok(i32::from_le_bytes(i32_bytes.try_into()?))
    }
}

impl ByteConverter for i64 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // i64 as u64
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // i64 as u64
        let i64_bytes = get_multiple_bytes(bytes, index, 8)?;
        Ok(i64::from_le_bytes(i64_bytes.try_into()?))
    }
}

impl ByteConverter for i128 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // i128 as u128
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // i128 as u128
        let i128_bytes = get_multiple_bytes(bytes, index, 16)?;
        Ok(i128::from_le_bytes(i128_bytes.try_into()?))
    }
}

impl ByteConverter for isize {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        
        // isize as usize
        ((*self) as usize).append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // isize as usize
        Ok(usize::extract_from_bytes(bytes, index)? as isize)
    }
}

impl ByteConverter for String {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let string_bytes = self.as_bytes();
        let string_bytes_length = string_bytes.len();

        // usize
        string_bytes_length.append_to_bytes(bytes)?;

        // slice
        bytes.extend_from_slice(string_bytes);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // usize
        let string_bytes_length = usize::extract_from_bytes(bytes, index)?;
        
        // slice
        let string_bytes = get_multiple_bytes(bytes, index, string_bytes_length)?;

        Ok(Self::from_utf8(string_bytes.to_vec())?)
    }
}

impl ByteConverter for CString {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let cstring_bytes = self.as_bytes();
        cstring_bytes.len().append_to_bytes(bytes)?;
        bytes.extend_from_slice(cstring_bytes);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let cstring_length = usize::extract_from_bytes(bytes, index)?;
        let cstring_bytes = get_multiple_bytes(bytes, index, cstring_length)?;
        Ok(Self::new(&cstring_bytes[..])?)
    }
}

// implementing for OsString may not work since we cannot guarantee the destination system is the same operating system

impl ByteConverter for u8 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        bytes.push(*self);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let byte = get_single_byte(bytes, index)?;
        Ok(byte)
    }
}

impl ByteConverter for u16 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let u16_bytes = get_multiple_bytes(bytes, index, 2)?;
        Ok(u16::from_le_bytes(u16_bytes.try_into()?))
    }
}

impl ByteConverter for u32 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let u32_bytes = get_multiple_bytes(bytes, index, 4)?;
        Ok(u32::from_le_bytes(u32_bytes.try_into()?))
    }
}

impl ByteConverter for u64 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let u64_bytes = get_multiple_bytes(bytes, index, 8)?;
        Ok(u64::from_le_bytes(u64_bytes.try_into()?))
    }
}

impl ByteConverter for u128 {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        bytes.extend_from_slice(&self.to_le_bytes());
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let u128_bytes = get_multiple_bytes(bytes, index, 16)?;
        Ok(u128::from_le_bytes(u128_bytes.try_into()?))
    }
}

impl ByteConverter for usize {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

        if cfg!(target_pointer_width = "64") {
            // byte
            bytes.push(8);

            // u64
            ((*self) as u64).append_to_bytes(bytes)?;
        }
        else {
            // byte
            bytes.push(4);

            // u32
            ((*self) as u32).append_to_bytes(bytes)?;
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
        // byte
        let usize_length = get_single_byte(bytes, index)? as usize;

        if !cfg!(target_pointer_width = "64") && usize_length == 8 {
            return Err(ByteConverterError::FailedToExtractSixtyFourBitUsize.into());
        }

        let usize_value = match usize_length {
            8 => {
                // u64
                let u64_instance = u64::extract_from_bytes(bytes, index)?;
                usize::try_from(u64_instance)?
            },
            4 => {
                // u32
                let u32_instance = u32::extract_from_bytes(bytes, index)?;
                usize::try_from(u32_instance)?
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

impl<TKey, TValue> ByteConverter for HashMap<TKey, TValue>
where
    TKey: ByteConverter + Eq + std::hash::Hash,
    TValue: ByteConverter,
{
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.len().append_to_bytes(bytes)?;
        for (key, value) in self {
            key.append_to_bytes(bytes)?;
            value.append_to_bytes(bytes)?;
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let length = usize::extract_from_bytes(bytes, index)?;
        let mut output = HashMap::with_capacity(length);
        for _ in 0..length {
            output.insert(
                TKey::extract_from_bytes(bytes, index)?,
                TValue::extract_from_bytes(bytes, index)?,
            );
        }
        Ok(output)
    }
}

impl<T: ByteConverter> ByteConverter for Vec<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

        // usize
        self.len().append_to_bytes(bytes)?;

        for element in self {
            // T
            element.append_to_bytes(bytes)?;
        }

        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        
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

impl<T: ByteConverter> ByteConverter for VecDeque<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.len().append_to_bytes(bytes)?;
        let (front, back) = self.as_slices();
        for element in front {
            element.append_to_bytes(bytes)?;
        }
        for element in back {
            element.append_to_bytes(bytes)?;
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let vec_deque_length = usize::extract_from_bytes(bytes, index)?;
        let mut vec_deque = VecDeque::with_capacity(vec_deque_length);
        for _ in 0..vec_deque_length {
            let vec_deque_element = T::extract_from_bytes(bytes, index)?;
            vec_deque.push_back(vec_deque_element);
        }
        Ok(vec_deque)
    }
}

impl<T: ByteConverter, const C: usize> ByteConverter for [T; C] {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        for byte_converter in self.iter() {
            byte_converter.append_to_bytes(bytes)?;
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {

        let mut vec = Vec::with_capacity(C);

        for _ in 0..C {
            vec.push(T::extract_from_bytes(bytes, index)?);
        }

        let array: [T; C] = vec.try_into().map_err(|_| "Incorrect array length")?;
        Ok(array)
    }
}

impl<T: ByteConverter> ByteConverter for Option<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self {
            Some(inner) => {
                0u8.append_to_bytes(bytes)?;
                inner.append_to_bytes(bytes)?;
            },
            None => {
                1u8.append_to_bytes(bytes)?;
            }
        }
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
        match enum_variant_byte {
            0u8 => {
                Ok(Self::Some(T::extract_from_bytes(bytes, index)?))
            },
            1u8 => {
                Ok(Self::None)
            },
            _ => {
                Err(ByteConverterError::UnexpectedByteValueForOption {
                    byte_value: enum_variant_byte,
                }.into())
            }
        }
    }
}

//impl<T, E> ByteConverter for Result<T, E>
//where
//    T: ByteConverter,
//    E: Error + Send + Sync + 'static,
//{
//    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//        match self {
//            Ok(value) => {
//                0u8.append_to_bytes(bytes)?;
//                value.append_to_bytes(bytes)?;
//            },
//            Err(error) => {
//                return Err("TODO fix error type".into());
//            }
//        }
//        Ok(())
//    }
//    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
//        let enum_variant_byte = u8::extract_from_bytes(bytes, index)?;
//        match enum_variant_byte {
//            0u8 => {
//                Ok(Self::Ok(
//                    T::extract_from_bytes(bytes, index)?,
//                ))
//            },
//            _ => {
//                Err("Unexpected enum variant byte.".into())
//            },
//        }
//    }
//}

impl<T: ByteConverter> ByteConverter for Box<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.as_ref().append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Box::new(T::extract_from_bytes(bytes, index)?))
    }
}

impl<T: ByteConverter> ByteConverter for Rc<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.as_ref().append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Rc::new(T::extract_from_bytes(bytes, index)?))
    }
}

impl<T: ByteConverter> ByteConverter for Arc<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.as_ref().append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Arc::new(T::extract_from_bytes(bytes, index)?))
    }
}

impl<T: ByteConverter> ByteConverter for Mutex<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self
            .lock()
            .map_err(|_| {
                Box::new(ByteConverterError::FailedToLockMutex)
            })?
            .append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(Mutex::new(T::extract_from_bytes(bytes, index)?))
    }
}

impl<T: ByteConverter> ByteConverter for RwLock<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self
            .read()
            .map_err(|_| {
                Box::new(ByteConverterError::FailedToLockMutex)
            })?
            .append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(RwLock::new(T::extract_from_bytes(bytes, index)?))
    }
}

impl<T: ByteConverter> ByteConverter for RefCell<T> {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.borrow().append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(RefCell::new(T::extract_from_bytes(bytes, index)?))
    }
}

// TODO wait for specialization to become stable
//impl<T: ByteConverter + Copy> ByteConverter for Cell<T> {
//    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//        self.get().append_to_bytes(bytes)?;
//        Ok(())
//    }
//    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
//        Ok(Cell::new(T::extract_from_bytes(bytes, index)?))
//    }
//}
//
//impl<T: ByteConverter + Default> ByteConverter for Cell<T> {
//    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//        let value = self.replace(T::default());
//        value.append_to_bytes(bytes)?;
//        self.set(value);
//        Ok(())
//    }
//    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
//        Ok(Cell::new(T::extract_from_bytes(bytes, index)?))
//    }
//}

macro_rules! tuple_byte_converter {
    ($($index:tt $t:tt),+) => {
        paste::paste! {
            impl<
                $($t: ByteConverter),+
            > ByteConverter for ($(
                $t,
            )+) {
                #[inline(always)]
                fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
                    $(
                        self.$index.append_to_bytes(bytes)?;
                    )*
                    Ok(())
                }
                #[inline(always)]
                fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
                    $(
                        let [<$t:lower>] = $t::extract_from_bytes(bytes, index)?;
                    )*
                    Ok((
                        $(
                            [<$t:lower>],
                        )+
                    ))
                }
            }
        }
    };
}

tuple_byte_converter!(0 T1);
tuple_byte_converter!(0 T1, 1 T2);
tuple_byte_converter!(0 T1, 1 T2, 2 T3);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22, 22 T23);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22, 22 T23, 23 T24);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22, 22 T23, 23 T24, 24 T25);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22, 22 T23, 23 T24, 24 T25, 25 T26);
tuple_byte_converter!(0 T1, 1 T2, 2 T3, 3 T4, 4 T5, 5 T6, 6 T7, 7 T8, 8 T9, 9 T10, 10 T11, 11 T12, 12 T13, 13 T14, 14 T15, 15 T16, 16 T17, 17 T18, 18 T19, 19 T20, 20 T21, 21 T22, 22 T23, 23 T24, 24 T25, 25 T26, 26 T27);

// TODO uncomment once the nightly stability of overriding specialization is pushed into stable Rust
//impl ByteCon for Vec<usize> {
//    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
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
//    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
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
    fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error + Send + Sync + 'static>> {
        let mut bytes = Vec::new();
        const CHUNK_SIZE: usize = 1024;
        let mut chunk = [0u8; CHUNK_SIZE];

        // read the number of bytes we are expecting to read as part of the deserialization to T
        let mut initial_packet = [0u8; 8];
        let read_result = self.read_exact(&mut initial_packet);
        if let Err(error) = read_result {
            let result: Result<T, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
            return result;
        }
        let expected_bytes_length: u64 = u64::from_le_bytes(initial_packet);

        // loop over self and read out bytes until we have reach the expected number of bytes
        while (bytes.len() as u64) < expected_bytes_length {
            let expected_remaining_bytes_length = expected_bytes_length - bytes.len() as u64;
            if expected_remaining_bytes_length >= CHUNK_SIZE as u64 {
                let read_bytes_length_result = self.read(&mut chunk);
                if let Err(error) = read_bytes_length_result {
                    let result: Result<T, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
                    return result;
                };
                let read_bytes_length = read_bytes_length_result.unwrap();
                if read_bytes_length != 0 {
                    bytes.extend_from_slice(&chunk[..read_bytes_length]);
                }
            }
            else {
                // the remaining bytes is less than the chunk size, so only grab the remaining bytes
                let mut smaller_chunk = vec![0u8; expected_remaining_bytes_length as usize];
                let read_bytes_length_result = self.read(&mut smaller_chunk);
                if let Err(error) = read_bytes_length_result {
                    let result: Result<T, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
                    return result;
                };
                let read_bytes_length = read_bytes_length_result.unwrap();
                if read_bytes_length != 0 {
                    bytes.extend_from_slice(&smaller_chunk[..read_bytes_length]);
                }
            }
        }

        // we are explicitly pulling out the exact number of bytes for T to deserialize
        T::deserialize_from_bytes(&bytes)
    }
}

impl<TWrite: std::io::Write> ByteStreamWriter for TWrite {
    fn write_from_byte_converter(&mut self, byte_converter: &impl ByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let stream_bytes = byte_converter.to_vec_bytes()?;
        let stream_bytes_length: u64 = stream_bytes.len() as u64;
        self.write(&stream_bytes_length.to_le_bytes())?;
        self.write(&stream_bytes)?;
        Ok(())
    }
}

// NOTE: Path is a reference type like &str while PathBuf is owned

impl ByteConverter for PathBuf {
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        String::from(self.to_string_lossy()).append_to_bytes(bytes)?;
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes(bytes: &Vec<u8>, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        Ok(PathBuf::from(String::extract_from_bytes(bytes, index)?))
    }
}

// this seems to conflict with KeyCode
//impl<TError> ByteConverter for TError
//where
//    TError: Error + Send + Sync + 'static,
//{
//    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//        todo!();
//    }
//}

struct TypedByteConverterRegistration<TContext, TOutput, TByteConverter> {
    extract_from_bytes_function: fn(&Vec<u8>, &mut usize) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>,
    apply_function: fn(&TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>,
}

impl<TContext, TOutput, TByteConverter> TypedByteConverterRegistration<TContext, TOutput, TByteConverter> {
    #[inline(always)]
    fn extract_from_bytes_and_apply(&self, context: &TContext, bytes: &Vec<u8>, index: &mut usize) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>> {
        let byte_converter = (self.extract_from_bytes_function)(bytes, index)?;
        (self.apply_function)(context, byte_converter)
    }
}

struct UntypedByteConverterRegistration<TContext, TOutput> {
    type_id: TypeId,
    extract_from_bytes_function: unsafe fn(),
    apply_function: unsafe fn(),
    phantom_context: std::marker::PhantomData<TContext>,
    phantom_output: std::marker::PhantomData<TOutput>,
}

impl<TContext, TOutput> UntypedByteConverterRegistration<TContext, TOutput> {
    pub fn new<TByteConverter>(apply_function: fn(&TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>) -> Self
    where
        TByteConverter: ByteConverter + Any,
    {
        Self {
            type_id: std::any::TypeId::of::<TByteConverter>(),
            extract_from_bytes_function: unsafe { std::mem::transmute::<fn(&Vec<u8>, &mut usize) -> Result<TByteConverter, Box<dyn Error + Sync + Send + 'static>>, unsafe fn()>(TByteConverter::extract_from_bytes) },
            apply_function: unsafe { std::mem::transmute::<fn(&TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>, unsafe fn()>(apply_function) },
            phantom_context: std::marker::PhantomData::default(),
            phantom_output: std::marker::PhantomData::default(),
        }
    }
    fn cast<TByteConverter>(&self) -> TypedByteConverterRegistration<TContext, TOutput, TByteConverter> {
        TypedByteConverterRegistration {
            extract_from_bytes_function: unsafe { std::mem::transmute::<unsafe fn(), fn(&Vec<u8>, &mut usize) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>>(self.extract_from_bytes_function) },
            apply_function: unsafe { std::mem::transmute::<unsafe fn(), fn(&TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>>(self.apply_function) },
        }
    }
}

#[inline(always)]
fn extract_from_bytes_and_apply<TContext, TOutput, TByteConverter>(
    untyped_byte_converter_registration: &UntypedByteConverterRegistration<TContext, TOutput>,
    context: &TContext,
    bytes: &Vec<u8>,
    index: &mut usize,
) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>> {
    untyped_byte_converter_registration.cast::<TByteConverter>().extract_from_bytes_and_apply(context, bytes, index)
}

pub struct ByteConverterFactory<TContext, TOutput> {
    untyped_byte_converter_registration_per_type_id: HashMap<TypeId, (UntypedByteConverterRegistration<TContext, TOutput>, fn(&UntypedByteConverterRegistration<TContext, TOutput>, &TContext, &Vec<u8>, &mut usize) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>)>,
}

impl<TContext, TOutput> Default for ByteConverterFactory<TContext, TOutput> {
    fn default() -> Self {
        Self {
            untyped_byte_converter_registration_per_type_id: HashMap::new(),
        }
    }
}

impl<TContext, TOutput> ByteConverterFactory<TContext, TOutput> {
    pub fn register<TByteConverter>(&mut self, apply_function: fn(&TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>) -> &mut Self
    where
        TByteConverter: ByteConverter + Any,
    {
        let untyped_byte_converter_registration = UntypedByteConverterRegistration::new::<TByteConverter>(apply_function);
        self.untyped_byte_converter_registration_per_type_id.insert(
            untyped_byte_converter_registration.type_id,
            (
                untyped_byte_converter_registration,
                extract_from_bytes_and_apply::<TContext, TOutput, TByteConverter>,
            ),
        );
        self
    }
    #[inline(always)]
    pub fn extract_from_bytes_and_apply(&self, context: &TContext, type_id: TypeId, bytes: &Vec<u8>, index: &mut usize) -> Result<TOutput, Box<dyn Error + Sync + Send + 'static>>
    {
        let Some((untyped_byte_converter_registration, extract_from_bytes_and_apply)) = self.untyped_byte_converter_registration_per_type_id.get(&type_id) else {
            return Err("TypeId not registered to any ByteConverter.".into());
        };
        let output = extract_from_bytes_and_apply(untyped_byte_converter_registration, context, bytes, index)?;
        Ok(output)
    }
    pub fn deserialize_from_bytes_and_apply(&self, context: &TContext, type_id: TypeId, bytes: &Vec<u8>) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>> {
        let mut index = 0;
        let output = self.extract_from_bytes_and_apply(context, type_id, bytes, &mut index)?;
        if index != bytes.len() {
            return Err("Failed to deserialize all of the bytes. There may be more than one ByteConverter within the provided bytes collection.".into());
        }
        Ok(output)
    }
}