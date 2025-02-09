use std::{any::Any, collections::HashMap, error::Error, future::Future, io::Cursor, marker::PhantomData};

use bincode::Options;
use serde::{de::DeserializeOwned, Serialize};

// TODO add a version byte at the front of each append_to_bytes call
//      this can be used to match on within the extract so that changes in format across versions of this crate are unaffected
// TODO replace string errors with usage of ByteConverterError variants

#[cfg(feature = "base")]
pub mod base;

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
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized;
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
    fn deserialize_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let bytes_ref = bytes.as_ref();
        let mut index = 0;
        let instance = Self::extract_from_bytes(bytes, &mut index)?;
        if index != bytes_ref.len() {
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
fn get_single_byte<TBytes: AsRef<[u8]>>(bytes: TBytes, index: &mut usize) -> Result<u8, Box<dyn Error + Send + Sync + 'static>> {
    let bytes_ref = bytes.as_ref();
    let bytes_length = bytes_ref.len();
    let index_deref = *index;
    let next_index = index_deref + 1;
    if bytes_length < next_index {
        return Err(ByteConverterError::IndexOutOfRange {
            index: next_index,
            length: bytes_length,
        }.into());
    }
    let byte = bytes_ref[index_deref];
    *index = next_index;
    Ok(byte)
}

#[inline(always)]
fn get_multiple_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize, size: usize) -> Result<&'a [u8], Box<dyn Error + Send + Sync + 'static>> {
    let bytes_ref = bytes.as_ref();
    let bytes_length = bytes_ref.len();
    let index_deref = *index;
    let next_index = index_deref + size;
    if bytes_length < next_index {
        return Err(ByteConverterError::IndexOutOfRange {
            index: next_index,
            length: bytes_length,
        }.into());
    }
    let multiple_bytes = &bytes_ref[index_deref..next_index];
    *index = next_index;
    Ok(multiple_bytes)
}

struct TypedSerializationByteConverterRegistration<TContext> {
    extract_bytes_from_context_function: fn(&mut TContext, &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>,
}

impl<TContext> TypedSerializationByteConverterRegistration<TContext> {
    #[inline(always)]
    fn extract_bytes_from_context(&self, context: &mut TContext, type_name: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
        (self.extract_bytes_from_context_function)(context, type_name)
    }
}

struct TypedDeserializationByteConverterRegistration<TContext, TOutput, TByteConverter> {
    extract_byte_converter_from_context_function: fn(&mut TContext) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>,
    apply_function: fn(&mut TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>,
}

impl<TContext, TOutput, TByteConverter> TypedDeserializationByteConverterRegistration<TContext, TOutput, TByteConverter> {
    #[inline(always)]
    fn extract_byte_converter_from_context_and_apply(&self, context: &mut TContext) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>> {
        let byte_converter = (self.extract_byte_converter_from_context_function)(context)?;
        (self.apply_function)(context, byte_converter)
    }
}

struct UntypedSerializationByteConverterRegistration<TContext> {
    type_name: &'static str,
    extract_bytes_from_context_function: unsafe fn(),
    phantom_context: std::marker::PhantomData<TContext>,
}

impl<TContext> UntypedSerializationByteConverterRegistration<TContext> {
    pub fn new<TByteConverter>(
        extract_bytes_from_context_function: fn(&mut TContext, &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self
    where
        TByteConverter: ByteConverter + Any,
    {
        Self {
            type_name: std::any::type_name::<TByteConverter>(),
            extract_bytes_from_context_function: unsafe { std::mem::transmute::<fn(&mut TContext, &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>, unsafe fn()>(extract_bytes_from_context_function) },
            phantom_context: PhantomData::default(),
        }
    }
    #[inline(always)]
    fn cast<TByteConverter>(&self) -> TypedSerializationByteConverterRegistration<TContext> {
        TypedSerializationByteConverterRegistration::<TContext> {
            extract_bytes_from_context_function: unsafe { std::mem::transmute::<unsafe fn(), fn(&mut TContext, &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>>(self.extract_bytes_from_context_function) },
        }
    }
}

struct UntypedDeserializationByteConverterRegistration<TContext, TOutput> {
    type_name: &'static str,
    extract_byte_converter_from_context_function: unsafe fn(),
    apply_function: unsafe fn(),
    phantom_context: std::marker::PhantomData<TContext>,
    phantom_output: std::marker::PhantomData<TOutput>,
}

impl<TContext, TOutput> UntypedDeserializationByteConverterRegistration<TContext, TOutput> {
    pub fn new<TByteConverter>(
        extract_byte_converter_from_context_function: fn(&mut TContext) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>,
        apply_function: fn(&mut TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self
    where
        TByteConverter: ByteConverter + Any,
    {
        Self {
            type_name: std::any::type_name::<TByteConverter>(),
            extract_byte_converter_from_context_function: unsafe { std::mem::transmute::<fn(&mut TContext) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>, unsafe fn()>(extract_byte_converter_from_context_function) },
            apply_function: unsafe { std::mem::transmute::<fn(&mut TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>, unsafe fn()>(apply_function) },
            phantom_context: PhantomData::default(),
            phantom_output: PhantomData::default(),
        }
    }
    #[inline(always)]
    fn cast<TByteConverter>(&self) -> TypedDeserializationByteConverterRegistration<TContext, TOutput, TByteConverter> {
        TypedDeserializationByteConverterRegistration::<TContext, TOutput, TByteConverter> {
            extract_byte_converter_from_context_function: unsafe { std::mem::transmute::<unsafe fn(), fn(&mut TContext) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>>(self.extract_byte_converter_from_context_function) },
            apply_function: unsafe { std::mem::transmute::<unsafe fn(), fn(&mut TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>>(self.apply_function) },
        }
    }
}

#[inline(always)]
fn extract_bytes_from_context<TContext, TByteConverter>(
    untyped_byte_converter_registration: &UntypedSerializationByteConverterRegistration<TContext>,
    context: &mut TContext,
    type_name: &str,
) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
    untyped_byte_converter_registration.cast::<TByteConverter>().extract_bytes_from_context(context, type_name)
}

#[inline(always)]
fn extract_byte_converter_from_context_and_apply<TContext, TOutput, TByteConverter>(
    untyped_byte_converter_registration: &UntypedDeserializationByteConverterRegistration<TContext, TOutput>,
    context: &mut TContext,
) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>> {
    untyped_byte_converter_registration.cast::<TByteConverter>().extract_byte_converter_from_context_and_apply(context)
}

pub struct SerializationByteConverterFactory<TContext> {
    untyped_byte_converter_registration_per_type_name: HashMap<&'static str, (
        UntypedSerializationByteConverterRegistration<TContext>,
        fn(&UntypedSerializationByteConverterRegistration<TContext>, &mut TContext, &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>,
    )>,
}

impl<TContext> Default for SerializationByteConverterFactory<TContext> {
    fn default() -> Self {
        Self {
            untyped_byte_converter_registration_per_type_name: HashMap::new(),
        }
    }
}

impl<TContext> SerializationByteConverterFactory<TContext>
{
    pub fn register<TByteConverter>(
        &mut self,
        extract_bytes_from_context_function: fn(&mut TContext, &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>>,
    ) -> &mut Self
    where
        TByteConverter: ByteConverter + 'static,
    {
        let untyped_byte_converter_registration = UntypedSerializationByteConverterRegistration::new::<TByteConverter>(extract_bytes_from_context_function);
        self.untyped_byte_converter_registration_per_type_name.insert(
            untyped_byte_converter_registration.type_name,
            (
                untyped_byte_converter_registration,
                extract_bytes_from_context::<TContext, TByteConverter>,
            ),
        );
        self
    }
    pub fn get_registered_type_names(&self) -> Vec<&'static str> {
        self.untyped_byte_converter_registration_per_type_name.keys()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }
    #[inline(always)]
    pub fn serialize(&self, context: &mut TContext, type_name: &str) -> Result<Vec<u8>, Box<dyn Error + Sync + Send + 'static>>
    {
        let Some((untyped_byte_converter_registration, apply)) = self.untyped_byte_converter_registration_per_type_name.get(&type_name) else {
            return Err("type_name not registered to any ByteConverter.".into());
        };
        let output = apply(untyped_byte_converter_registration, context, type_name)?;
        Ok(output)
    }
}

pub struct DeserializationByteConverterFactory<TContext, TOutput> {
    untyped_byte_converter_registration_per_type_name: HashMap<&'static str, (
        UntypedDeserializationByteConverterRegistration<TContext, TOutput>,
        fn(&UntypedDeserializationByteConverterRegistration<TContext, TOutput>, &mut TContext) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>,
    )>,
}

impl<TContext, TOutput> Default for DeserializationByteConverterFactory<TContext, TOutput> {
    fn default() -> Self {
        Self {
            untyped_byte_converter_registration_per_type_name: HashMap::new(),
        }
    }
}

impl<TContext, TOutput> DeserializationByteConverterFactory<TContext, TOutput>
{
    pub fn register<TByteConverter>(
        &mut self,
        extract_byte_converter_from_context_function: fn(&mut TContext) -> Result<TByteConverter, Box<dyn Error + Send + Sync + 'static>>,
        apply_function: fn(&mut TContext, TByteConverter) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>>,
    ) -> &mut Self
    where
        TByteConverter: ByteConverter + 'static,
    {
        let untyped_byte_converter_registration = UntypedDeserializationByteConverterRegistration::new::<TByteConverter>(extract_byte_converter_from_context_function, apply_function);
        self.untyped_byte_converter_registration_per_type_name.insert(
            untyped_byte_converter_registration.type_name,
            (
                untyped_byte_converter_registration,
                extract_byte_converter_from_context_and_apply::<TContext, TOutput, TByteConverter>,
            ),
        );
        self
    }
    pub fn get_registered_type_names(&self) -> Vec<&'static str> {
        self.untyped_byte_converter_registration_per_type_name.keys()
            .into_iter()
            .cloned()
            .collect::<Vec<_>>()
    }
    #[inline(always)]
    pub fn deserialize(&self, context: &mut TContext, type_name: &str) -> Result<TOutput, Box<dyn Error + Sync + Send + 'static>>
    {
        let Some((untyped_byte_converter_registration, apply)) = self.untyped_byte_converter_registration_per_type_name.get(&type_name) else {
            return Err("type_name not registered to any ByteConverter.".into());
        };
        let output = apply(untyped_byte_converter_registration, context)?;
        Ok(output)
    }
}

// allow for wrapping a ByteConverter in the type below for utilizing bincode, if that is better for certain purposes
pub struct BincodeByteConverter<TByteConverter>(TByteConverter)
where
    TByteConverter: ByteConverter + Serialize + DeserializeOwned;

impl<TByteConverter> ByteConverter for BincodeByteConverter<TByteConverter>
where
    TByteConverter: ByteConverter + Serialize + DeserializeOwned,
{
    #[inline(always)]
    fn append_to_bytes(&self, bytes: &mut Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let serialized = bincode::serialize(&self.0)?;
        bytes.extend_from_slice(&serialized);
        Ok(())
    }
    #[inline(always)]
    fn extract_from_bytes<'a, TBytes: AsRef<[u8]>>(bytes: &'a TBytes, index: &mut usize) -> Result<Self, Box<dyn Error + Send + Sync + 'static>> where Self: Sized {
        let bytes_ref = bytes.as_ref();
        let mut cursor = Cursor::new(&bytes_ref[*index..]);
        let output: TByteConverter = bincode::options().deserialize_from(&mut cursor)?;
        *index += cursor.position() as usize;
        Ok(Self(output))
    }
}