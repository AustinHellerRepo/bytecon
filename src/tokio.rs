use std::error::Error;
use tokio::{io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt}, net::TcpStream};
use tokio_rustls::TlsStream;
use crate::{ByteConverter, ByteStreamReaderAsync, ByteStreamWriterAsync};

#[inline(always)]
async fn read_to_byte_converter<TOutput: ByteConverter, TStream: AsyncWrite + AsyncRead + Unpin>(stream: &mut TStream) -> Result<TOutput, Box<dyn Error + Send + Sync + 'static>> {
    let usize_length = {
        // byte
        let mut single_byte_chunk = [0u8; 1];
        let read_result = stream.read_exact(&mut single_byte_chunk)
            .await;
        if let Err(error) = read_result {
            let result: Result<TOutput, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
            return result;
        }

        // length of usize
        single_byte_chunk[0]
    } as usize;

    if !cfg!(target_pointer_width = "64") && usize_length == 8 {
        return Err(TokioByteConError::FailedToExtractSixtyFourBitUsize.into());
    }

    let expected_bytes_length = match usize_length {
        8 => {
            // 8 bytes
            let mut initial_packet = [0u8; 8];
            let read_result = stream.read_exact(&mut initial_packet)
                .await;
            if let Err(error) = read_result {
                let result: Result<TOutput, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
                return result;
            }

            // u64
            let u64_instance = u64::from_le_bytes(initial_packet);
            usize::try_from(u64_instance)?
        },
        4 => {
            // 4 bytes
            let mut initial_packet = [0u8; 4];
            let read_result = stream.read_exact(&mut initial_packet)
                .await;
            if let Err(error) = read_result {
                let result: Result<TOutput, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
                return result;
            }

            // u32
            let u32_instance = u32::from_le_bytes(initial_packet);
            usize::try_from(u32_instance)?
        },
        _ => {
            return Err(TokioByteConError::UnexpectedSizeOfUsize {
                bytes_length: usize_length,
            }.into());
        }
    };

    let mut bytes = Vec::new();
    let mut chunk = [0u8; 4096];
    while bytes.len() < expected_bytes_length {
        let read_bytes_length_result = stream.read(&mut chunk)
            .await;
        if let Err(error) = read_bytes_length_result {
            let result: Result<TOutput, Box<dyn Error + Send + Sync + 'static>> = Err(Box::new(error));
            return result;
        }

        let read_bytes_length = read_bytes_length_result.unwrap();

        if read_bytes_length != 0 {
            bytes.extend_from_slice(&chunk[..read_bytes_length]);
        }
    }

    TOutput::deserialize_from_bytes(&bytes)
}

#[inline(always)]
async fn write_from_byte_converter<TStream: AsyncWrite + AsyncRead + Unpin>(stream: &mut TStream, byte_converter: &impl crate::ByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let byte_converter_bytes = byte_converter.to_vec_bytes()?;
    let byte_converter_bytes_length_bytes = byte_converter_bytes.len().to_vec_bytes()?;
    stream.write(&byte_converter_bytes_length_bytes)
        .await?;
    stream.write(&byte_converter_bytes)
        .await?;
    Ok(())
}

impl ByteStreamReaderAsync for TcpStream {
    #[inline(always)]
    async fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error + Send + Sync + 'static>> {
        read_to_byte_converter(self).await
    }
}

impl ByteStreamWriterAsync for TcpStream {
    #[inline(always)]
    async fn write_from_byte_converter(&mut self, byte_converter: &impl ByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        write_from_byte_converter(self, byte_converter).await
    }
}

impl<TStream: AsyncWrite + AsyncRead + Unpin> ByteStreamReaderAsync for TlsStream<TStream> {
    #[inline(always)]
    async fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error + Send + Sync + 'static>> {
        read_to_byte_converter(self).await
    }
}

impl<TStream: AsyncWrite + AsyncRead + Unpin> ByteStreamWriterAsync for TlsStream<TStream> {
    #[inline(always)]
    async fn write_from_byte_converter(&mut self, byte_converter: &impl crate::ByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        write_from_byte_converter(self, byte_converter).await
    }
}

impl ByteStreamReaderAsync for tokio::sync::mpsc::Receiver<Vec<u8>> {
    #[inline(always)]
    async fn read_to_byte_converter<T: ByteConverter>(&mut self) -> Result<T, Box<dyn Error + Send + Sync + 'static>> {
        let bytes = self.recv()
            .await
            .ok_or(TokioByteConError::OptionVariantNoneReceivedFromReceiver)?;
        T::deserialize_from_bytes(&bytes)
    }
}

impl ByteStreamWriterAsync for tokio::sync::mpsc::Sender<Vec<u8>> {
    #[inline(always)]
    async fn write_from_byte_converter(&mut self, byte_converter: &impl ByteConverter) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        let bytes = byte_converter.to_vec_bytes()?;
        self.send(bytes)
            .await?;
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
enum TokioByteConError {
    #[error("Option variant None received from receiver.")]
    OptionVariantNoneReceivedFromReceiver,
    #[error("Failed to extract 64-bit usize on this machine.")]
    FailedToExtractSixtyFourBitUsize,
    #[error("Unexpected number of bytes {bytes_length} for usize that expects either 4 or 8.")]
    UnexpectedSizeOfUsize {
        bytes_length: usize,
    },
}